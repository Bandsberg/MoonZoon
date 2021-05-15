use crate::*;

#[macro_export]
macro_rules! println {
<<<<<<< HEAD
    ($($t:tt)*) => ($crate::log(&format_args!($($t)*).to_string()))
=======
    ($($arg:tt)*) => ($crate::log(&$crate::format!($($arg)*)))
>>>>>>> a70fd9927f5b90424db36fda687d6c6344278925
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(input: &str);
}
