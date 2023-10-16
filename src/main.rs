use silkenweb::{prelude::*, prelude::html::*};

silkenweb::css!("style.css");

fn main() {
    mount("app", div().class(class::CONTAINER).child(app_header()));
}

fn app_header() -> Header {
    header().child(div().class(class::LOGO).text("AJUNTA"))
}
