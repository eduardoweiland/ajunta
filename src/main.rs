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
    div()
        .child(builder_row().class(class::ORANGE))
        .child(builder_row().class(class::GREEN))
        .child(builder_row().class(class::BLUE))
        .child(builder_row().class(class::PURPLE))
}

fn builder_row() -> Div {
    div()
        .class("builder-row")
        .child(input().required(true).placeholder("Nome da categoria"))
        .child(input().required(true).placeholder("Palavra"))
        .child(input().required(true).placeholder("Palavra"))
        .child(input().required(true).placeholder("Palavra"))
        .child(input().required(true).placeholder("Palavra"))
}

fn not_found() -> Div {
    div().text("where are you?")
}
