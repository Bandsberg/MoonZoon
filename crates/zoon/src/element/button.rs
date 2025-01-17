use crate::*;
use std::marker::PhantomData;

// ------ ------
//    Element 
// ------ ------

make_flags!(Label, OnPress);

pub struct Button<LabelFlag, OnPressFlag> {
    raw_el: RawEl,
    flags: PhantomData<(LabelFlag, OnPressFlag)>
}

impl Button<LabelFlagNotSet, OnPressFlagNotSet> {
    pub fn new() -> Self {
        Self {
            raw_el: RawEl::new("div")
                .attr("class", "button")
                .attr("role", "button")
                .attr("tabindex", "0"),
            flags: PhantomData,
        }
    }
}

impl<OnPressFlag> Element for Button<LabelFlagSet, OnPressFlag> {
    fn into_raw_element(self) -> RawElement {
        self.raw_el.into()
    }
}

// ------ ------
//  Attributes 
// ------ ------

impl<'a, LabelFlag, OnPressFlag> Button<LabelFlag, OnPressFlag> {
    pub fn label(
        self, 
        label: impl IntoElement<'a> + 'a
    ) -> Button<LabelFlagSet, OnPressFlag>
        where LabelFlag: FlagNotSet
    {
        Button {
            raw_el: self.raw_el.child(label),
            flags: PhantomData
        }
    }

    pub fn label_signal(
        self, 
        label: impl Signal<Item = impl IntoElement<'a>> + Unpin + 'static
    ) -> Button<LabelFlagSet, OnPressFlag> 
        where LabelFlag: FlagNotSet
    {
        Button {
            raw_el: self.raw_el.child_signal(label),
            flags: PhantomData
        }
    }

    pub fn on_press(
        self, 
        on_press: impl FnOnce() + Clone + 'static
    ) -> Button<LabelFlag, OnPressFlagSet> 
        where OnPressFlag: FlagNotSet
    {
        Button {
            raw_el: self.raw_el.event_handler(move |_: events::Click| (on_press.clone())()),
            flags: PhantomData
        }
    }
} 
