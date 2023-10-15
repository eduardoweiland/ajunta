use silkenweb::{elements::html::p, mount, prelude::ParentElement, router};

fn main() {
    mount("app", p().text(router::url_path().get_cloned().hash()));
}
