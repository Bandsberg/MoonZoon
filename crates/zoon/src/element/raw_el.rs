<<<<<<< HEAD
use wasm_bindgen::{closure::Closure, JsCast};
use crate::{RenderContext, ElVar, Element, __TrackedCall, __TrackedCallStack, IntoElement, ApplyToElement, render, element_macro};
use tracked_call_macro::tracked_call;
use crate::hook::el_var;
use crate::dom::{document, Node};
use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};
use std::mem;
=======
use crate::*;
>>>>>>> a70fd9927f5b90424db36fda687d6c6344278925

// ------ ------
//   Element 
// ------ ------

<<<<<<< HEAD
element_macro!(raw_el, RawEl::default());

#[derive(Default)]
pub struct RawEl<'a> {
    tag: Option<&'a str>,
    attrs: HashMap<&'a str, &'a str>,
    event_handlers: HashMap<&'static str, Vec<Box<dyn Fn(web_sys::Event)>>>,
    children: Vec<Box<dyn Element + 'a>>,
}

impl<'a> Element for RawEl<'a> {
    #[render]
    fn render(&mut self, rcx: RenderContext) {
        // @TODO resolve changed tag

        // log!("raw_el, index: {}", rcx.index);

        let node = dom_element(rcx, self.tag, |mut rcx| {
            for child in &mut self.children {
                child.render(rcx.inc_index().clone());
            }
        });

        let attrs = el_var(|| HashMap::<String, String>::new());
        attrs.update_mut(|attrs| {
            node.update_mut(|node| {
                let element = node.node_ws.unchecked_ref::<web_sys::Element>();

                attrs.retain(|name, value| {
                    if let Some(new_value) = self.attrs.remove(name.as_str()) {
                        if new_value != value {
                            element.set_attribute(name, value).unwrap();
                            *value = new_value.to_owned();
                        }
                        return true
                    } 
                    element.remove_attribute(name).unwrap();
                    false
                });

                for (new_name, new_value) in mem::take(&mut self.attrs) {
                    attrs.insert(new_name.to_owned(), new_value.to_owned());
                    element.set_attribute(new_name, new_value).unwrap();
                }
            });
        });

        let listeners = el_var(|| HashMap::new());
        listeners.update_mut(|listeners| {
            for (event, handlers) in mem::take(&mut self.event_handlers) {
                if handlers.is_empty() {
                    listeners.remove(event);
                    continue;
                }
                listeners
                    .entry(event)
                    .or_insert(Listener::new(event, node))
                    .set_handlers(handlers);
            }
        });
    }
}

#[tracked_call]
pub fn dom_element(mut rcx: RenderContext, tag: Option<&str>, children: impl FnOnce(RenderContext)) -> ElVar<Node> {
    // log!("el, index: {}", rcx.index);

    let node = el_var(|| {
        let el_ws = document().create_element(tag.unwrap_or("div")).expect("element");
        // el_ws.set_attribute("class", "el").expect("set class attribute");
        let node_ws = web_sys::Node::from(el_ws);
        rcx.node.update_mut(|node| {
            let parent_node_ws = &node.node_ws;
            parent_node_ws.insert_before(&node_ws, parent_node_ws.child_nodes().get(rcx.index + 1).as_ref()).expect("insert node");
        });
        Node { node_ws }
    });
    rcx.node = node;
    rcx.reset_index();
    children(rcx);
    node
}

struct Listener {
    event: &'static str,
    node: ElVar<Node>,
    handlers: Rc<RefCell<Vec<Box<dyn Fn(web_sys::Event)>>>>,
    callback: Closure<dyn Fn(web_sys::Event)>,
}

impl Listener {
    fn new(event: &'static str, node: ElVar<Node>) -> Self {
        let handlers = Rc::new(RefCell::new(Vec::new()));

        let handlers_clone = Rc::clone(&handlers);
        let callback = Closure::wrap(Box::new(move |event: web_sys::Event| {
            let user_handlers = mem::take::<Vec<Box<dyn Fn(web_sys::Event)>>>(&mut handlers_clone.borrow_mut());
            for user_handler in &user_handlers {
                user_handler(event.clone());
            }
            *handlers_clone.borrow_mut() = user_handlers;
        }) as Box<dyn Fn(web_sys::Event)>);

        node.update_mut(|node| {
            node
                .node_ws
                .unchecked_ref::<web_sys::EventTarget>()
                .add_event_listener_with_callback(event, callback.as_ref().unchecked_ref())
                .expect("add event listener");
        });

        Self {
            event,
            node,
            handlers,
            callback,
        }
    }

    fn set_handlers(&mut self, handlers: Vec<Box<dyn Fn(web_sys::Event)>>) {
        *self.handlers.borrow_mut() = handlers;
    }
}

impl Drop for Listener{
    fn drop(&mut self) {
        if !self.node.exists() {
            return;
        }
        self.node.update_mut(|node| {
            node
                .node_ws
                .unchecked_ref::<web_sys::EventTarget>()
                .remove_event_listener_with_callback(
                    self.event,
                    self.callback.as_ref().unchecked_ref(),
                )
                .expect("remove event listener");
        });
    }
}

=======
pub struct RawEl {
    dom_builder: DomBuilder<web_sys::HtmlElement>,
}

impl RawEl {
    pub fn new(tag: &str) -> Self {
        Self {
            dom_builder: DomBuilder::new_html(tag)
        }
    }
}

impl From<RawEl> for RawElement {
    fn from(raw_el: RawEl) -> Self {
        RawElement::El(raw_el)
    }
}

impl IntoDom for RawEl {
    fn into_dom(self) -> Dom {
        self.dom_builder.into_dom()
    }
}

impl Element for RawEl {
    fn into_raw_element(self) -> RawElement {
        self.into()
    }
}
    
>>>>>>> a70fd9927f5b90424db36fda687d6c6344278925
// ------ ------
//  Attributes 
// ------ ------

<<<<<<< HEAD
impl<'a> RawEl<'a> {
    pub fn child(mut self, child: impl IntoElement<'a> + 'a) -> Self {
        child.into_element().apply_to_element(&mut self);
        self
    }
} 

// ------ IntoElement ------

impl<'a, T: IntoElement<'a> + 'a> ApplyToElement<RawEl<'a>> for T {
    fn apply_to_element(self, raw_el: &mut RawEl<'a>) {
        raw_el.children.push(Box::new(self.into_element()));
    }
}

// ------ raw_el::tag(...)

pub struct Tag<'a>(&'a str);
pub fn tag<'a>(tag: &'a str) -> Tag<'a> {
    Tag(tag)
}
impl<'a> ApplyToElement<RawEl<'a>> for Tag<'a> {
    fn apply_to_element(self, raw_el: &mut RawEl<'a>) {
        raw_el.tag = Some(self.0);
    }
}

// ------ raw_el::attr(...)

pub struct Attr<'a> {
    name: &'a str,
    value: &'a str
}
pub fn attr<'a>(name: &'a str, value: &'a str) -> Attr<'a> {
    Attr { name, value }
}
impl<'a> ApplyToElement<RawEl<'a>> for Attr<'a> {
    fn apply_to_element(self, raw_el: &mut RawEl<'a>) {
        raw_el.attrs.insert(self.name, self.value);
    }
}

// ------ raw_el::event_handler(...)

pub struct EventHandler {
    event: &'static str,
    handler: Box<dyn Fn(web_sys::Event)>
}
pub fn event_handler(event: &'static str, handler: impl FnOnce(web_sys::Event) + Clone + 'static) -> EventHandler {
    EventHandler {
        event,
        handler: Box::new(move |event| handler.clone()(event))
    }
}
impl<'a> ApplyToElement<RawEl<'a>> for EventHandler {
    fn apply_to_element(self, raw_el: &mut RawEl<'a>) {
        raw_el
            .event_handlers
            .entry(self.event)
            .or_insert(vec![])
            .push(self.handler);
=======
impl<'a> RawEl {
    pub fn attr(self, name: &str, value: &str) -> Self {
        Self {
            dom_builder: self.dom_builder.attribute(name, value)
        }
    }

    pub fn attr_signal(self, name: impl ToString, value: impl Signal<Item = Option<impl ToString>> + Unpin + 'static) -> Self {
        Self {
            dom_builder: self.dom_builder.attribute_signal(
                name.to_string(), 
                value.map(|value| value.map(|value| value.to_string()))
            )
        }
    }

    pub fn event_handler<E: StaticEvent>(self, handler: impl FnOnce(E) + Clone + 'static) -> Self {
        let handler = move |event: E| handler.clone()(event);
        Self {
            dom_builder: self.dom_builder.event(handler)
        }
    }

    pub fn child(self, child: impl IntoOptionElement<'a> + 'a) -> Self {
        let dom_builder = if let Some(child) = child.into_option_element() {
            self.dom_builder.child(child.into_raw_element().into_dom())
        } else {
            self.dom_builder
        };
        Self {
            dom_builder,
        }
    }

    pub fn child_signal(
        self, 
        child: impl Signal<Item = impl IntoOptionElement<'a>> + Unpin + 'static
    ) -> Self {
        Self {
            dom_builder: self.dom_builder.child_signal(
                child.map(|child| child.into_option_element().map(|element| {
                    element.into_raw_element().into_dom()
                }))
            ),
        }
    }

    pub fn children(self, 
        children: impl IntoIterator<Item = impl IntoElement<'a> + 'a>
    ) -> Self {
        Self {
            dom_builder: self.dom_builder.children(
                children.into_iter().map(|child| child.into_element().into_raw_element().into_dom())
            ),
        }
    }

    pub fn children_signal_vec(
        self, 
        children: impl SignalVec<Item = impl IntoElement<'a>> + Unpin + 'static
    ) -> Self {
        Self {
            dom_builder: self.dom_builder.children_signal_vec(
                children.map(|child| child.into_element().into_raw_element().into_dom())
            ),
        }
>>>>>>> a70fd9927f5b90424db36fda687d6c6344278925
    }
}
