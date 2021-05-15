pub use wasm_bindgen::{self, prelude::*, JsCast};

pub mod element;
mod dom;
mod console;
<<<<<<< HEAD
mod hook;
mod el_var;
mod el_var_map;
mod cmp_var;
mod cmp_var_map;
mod s_var;
mod s_var_map;
mod var;
mod var_pointer;
mod var_ref;
mod var_map;
mod c_var;
mod c_var_map;
mod cache;
mod cache_map;
mod runtime;
mod block_call_stack;
mod component_call_stack;
mod tracked_call_stack;
mod relations;
mod tracked_call;
mod render_context;
=======
mod futures_signals_ext;
>>>>>>> a70fd9927f5b90424db36fda687d6c6344278925

pub use futures_signals_ext::{MutableExt, MutableVecExt};
pub use element::*;
<<<<<<< HEAD
pub use render_context::RenderContext;
pub use component::{Cmp, IntoComponent, __ComponentData};
pub use dom::{Node, window, document}; 
pub use el_var::{ElVar, CloneElVar};
pub use cmp_var::{CmpVar, CloneCmpVar};
pub use c_var::{CVar, CloneCVar};
pub use s_var::{SVar, CloneSVar, s_var};
pub use var::{Var, CloneVar};
pub use var_ref::{VarRef, CloneVarRef, ToVarRef, Variable};
pub use cache::{Cache, CloneCache, cache};
=======
pub use dom::{window, document}; 
pub use futures_signals::{
    self,
    map_mut,
    map_ref,
    signal::{Mutable, Signal, SignalExt},
    signal_vec::{MutableVec, SignalVec, SignalVecExt},
    signal_map::{MutableBTreeMap, SignalMap, SignalMapExt},
};
pub use dominator::{self, Dom, DomBuilder, events, traits::StaticEvent};
pub use paste;
>>>>>>> a70fd9927f5b90424db36fda687d6c6344278925
pub use console::log;

#[cfg(feature = "panic_hook")]
pub use console_error_panic_hook;

#[cfg(feature = "static_ref")]
pub use static_ref_macro::static_ref;
#[cfg(feature = "static_ref")]
pub use once_cell;

#[cfg(feature = "clone")]
pub use enclose::enc as clone;

#[cfg(feature = "small_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(feature = "fast_alloc")]
compile_error!("Do you know a fast allocator working in Wasm?");

// #[cfg(feature = "tracing_alloc")]
// #[global_allocator]
// static GLOBAL_ALLOCATOR: wasm_tracing_allocator::WasmTracingAllocator<std::alloc::System> = wasm_tracing_allocator::WasmTracingAllocator(std::alloc::System);

#[cfg(feature = "fmt")]
pub use ufmt::{self, uDebug, uDisplay, uWrite, uwrite, uwriteln};
#[cfg(feature = "fmt")]
pub use lexical::{self, WriteIntegerOptions, WriteFloatOptions, NumberFormat};

#[cfg(not(feature = "fmt"))]
pub use std::format as format;

#[cfg(feature = "fmt")]
#[macro_export]
macro_rules! format {
    ($($arg:tt)*) => {{
        let mut text = String::new();
        $crate::ufmt::uwrite!(&mut text, $($arg)*).unwrap_throw();
        text
    }}
}

pub trait FlagSet {}
pub trait FlagNotSet {}

#[macro_export]
macro_rules! make_flags {
    ($($flag:ident),*) => {
        $(paste::paste!{
            #[derive(Default)]
            pub struct [<$flag FlagSet>];
            #[derive(Default)]
            pub struct [<$flag FlagNotSet>];
            impl $crate::FlagSet for [<$flag FlagSet>] {}
            impl $crate::FlagNotSet for [<$flag FlagNotSet>] {}
        })*
    }
}

pub fn start_app<'a, E: Element>(browser_element_id: impl Into<Option<&'a str>>, view_root: impl FnOnce() -> E) {
    #[cfg(feature = "panic_hook")]
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let parent = browser_element_id
        .into()
        // @TODO we need a better error message
        .map(dominator::get_id)
        .unwrap_or_else(|| dominator::body().unchecked_into());

    dominator::append_dom(&parent, view_root().into_raw_element().into_dom());
}
