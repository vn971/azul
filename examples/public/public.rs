//! Example of the new, public API

use azul::{
    app::{App, AppConfig},
    css::Css,
    dom::Dom,
    window::WindowCreateOptions,
    callbacks::{RefAny, LayoutInfo},
};

struct Data {
    counter: usize,
}

fn layout(data: RefAny, _info: LayoutInfo) -> Dom {
    let data = data.downcast_ref::<Data>().unwrap();
    println!("counter: {:?}", data.counter);
    Dom::div()
}

fn main() {
    let data = Data {
        counter: 5,
    };
    let app = App::new(AppConfig::new(), RefAny::new(data), layout);
    app.run(WindowCreateOptions::new(Css::native()));
}