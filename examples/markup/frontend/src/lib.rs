use zoon::*;

#[static_ref]
fn counter() -> &'static Mutable<i32> {
    Mutable::new(0)
}

fn increment() {
    counter().update(|counter| counter + 1)
}

fn decrement() {
    counter().update(|counter| counter - 1)
}

fn root() -> impl Element {
    // Column::new()
    //     .item(Button::new().label("- Markup").on_press(decrement))
    //     .item(Text::with_signal(counter().signal()))
    //     .item(Button::new().label("+").on_press(increment))
    // RawHtmlEl::new("div").inner_markup("<p>Hellow!<p>")
    RawHtmlEl::from_markup("<h1>Hello!</h1>").unwrap_throw()
}

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", root);
}