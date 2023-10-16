use silkenweb::{prelude::html::*, prelude::*, router};

silkenweb::css!("style.css");

fn main() {
    mount(
        "app",
        div()
            .class(class::CONTAINER)
            .child(app_header())
            .child(router_view()),
    );
}

fn app_header() -> Header {
    header().child(div().class(class::LOGO).text("AJUNTA"))
}

fn router_view() -> Main {
    let route = Sig(router::url_path().signal_ref(|path| {
        if path.hash().is_empty() {
            home_view()
        } else if path.hash().starts_with("play/") {
            play_view()
        } else if path.hash().eq("build") {
            build_view()
        } else {
            not_found()
        }
    }));

    html::main().child(route)
}

fn home_view() -> Div {
    div().text("welcome home")
}

fn play_view() -> Div {
    div().text("let's play")
}

fn build_view() -> Div {
    div().text("let's build it")
}

fn not_found() -> Div {
    div().text("where are you?")
}
