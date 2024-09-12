#![allow(non_snake_case)]
extern crate VisEQ;
use VisEQ::lib::*;
use VisEQ::linear::*;
use VisEQ::quadratic::*;
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};




#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

fn main() {
    let testQuadratic:QuadraticStandardForm = QuadraticStandardForm {
        a:1.0,
        b:-2.0,
        c:-9.0,
    };
    let roots:Vec<Point> = QuadraticStandardForm.findRoots();
    let root_as_string:String = "(".to_string() + roots[0].x.to_string() + ", " + roots[0].y.to_string() + ")";

    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");

    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {root_as_string}"
    }
}

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        Link {
            to: Route::Blog {
                id: count()
            },
            "Go to blog"
        }
        div {
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
        }
    }
}
