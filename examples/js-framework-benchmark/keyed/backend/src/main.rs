use moon::*;

async fn init() {}

async fn frontend() -> Frontend {
    Frontend::new()
        .title("Benchmark example")
        .append_to_head(r#"<link href="/public/css/currentStyle.css" rel="stylesheet"/>"#)
<<<<<<< HEAD
=======
        .body_content(r#"<div id="main"></div>"#)
>>>>>>> a70fd9927f5b90424db36fda687d6c6344278925
}

async fn up_msg_handler(_: UpMsgRequest) {}

fn main() {
    start!(init, frontend, up_msg_handler).unwrap();
}
