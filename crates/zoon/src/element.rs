use dominator::Dom;

// -- modules --

pub mod button;
pub use button::Button;

pub mod column;
pub use column::Column;

pub mod el;
pub use el::El;

pub mod row;
pub use row::Row;

pub mod text;
pub use text::Text;

pub mod raw_el;
pub use raw_el::RawEl;

<<<<<<< HEAD
// ------ element_macro ------

#[macro_export]
macro_rules! element_macro {
    ( $name:tt, $element:expr ) => {
        // Replace $d with $ in the inner macro.
        $crate::with_dollar_sign! {
            ($d:tt) => {
                #[macro_export]
                macro_rules! $name {
                    ( $d ($d attribute:expr),* $d (,)?) => {
                        {
                            #[allow(unused_mut)]
                            let mut element = $element;
                            $d ( $d attribute.apply_to_element(&mut element); )*
                            element
                        }
                    }
                }
            }
        }
    }
}
=======
pub mod raw_text;
pub use raw_text::RawText;
>>>>>>> a70fd9927f5b90424db36fda687d6c6344278925

// ------ Element ------

pub trait Element {
    fn into_raw_element(self) -> RawElement;
}

// ------ RawElement ------

pub enum RawElement {
    El(RawEl),
    Text(RawText),
}

impl IntoDom for RawElement {
    fn into_dom(self) -> Dom {
        match self {
            RawElement::El(raw_el) => raw_el.into_dom(),
            RawElement::Text(raw_text) => raw_text.into_dom(),
        }
    }
}

// ------ IntoDom ------

pub trait IntoDom {
    fn into_dom(self) -> Dom;
}

// ------ IntoElement ------

pub trait IntoElement<'a> {
    type EL: Element;
    fn into_element(self) -> Self::EL; 
}

impl<'a, T: Element> IntoElement<'a> for T {
    type EL = T;
    fn into_element(self) -> Self::EL {
        self
    }
}

// ------ IntoOptionElement ------

pub trait IntoOptionElement<'a> {
    type EL: Element;
    fn into_option_element(self) -> Option<Self::EL>; 
}

impl<'a, E: Element, T: IntoElement<'a, EL = E>> IntoOptionElement<'a> for Option<T> {
    type EL = E;
    fn into_option_element(self) -> Option<Self::EL> {
        self.map(|into_element| into_element.into_element())
    }
}

impl<'a, E: Element, T: IntoElement<'a, EL = E>> IntoOptionElement<'a> for T {
    type EL = E;
    fn into_option_element(self) -> Option<Self::EL> {
        Some(self.into_element())
    }
}






