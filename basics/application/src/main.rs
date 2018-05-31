extern crate actix_web;
use actix_web::{http, server, App, HttpRequest, HttpResponse, Responder};
use std::cell::Cell;

// This struct is representing state
struct AppState {
    counter: Cell<usize>,
}

// fn index(req: HttpRequest) -> impl Responder {
//     "Hello world!"
// }
fn index(req: HttpRequest<AppState>) -> String {
    let count = req.state().counter.get() + 1; // get count
    req.state().counter.set(count); // store new count in state

    format!("Request number: {}", count) // response with count
}

fn main() {
    // let app = App::new()
    //     .prefix("/app")
    //     .resource("/index.html", |r| r.method(http::Method::GET).f(index))
    //     .finish();

    let server = server::new(|| {
        vec![
            App::new()
                .prefix("/app1")
                .resource("/", |r| r.f(|r| HttpResponse::Ok())),
            App::new()
                .prefix("/app2")
                .resource("/", |r| r.f(|r| HttpResponse::Ok())),
            App::new().resource("/", |r| r.f(|r| HttpResponse::Ok())),
            App::with_state(AppState { counter: Cell::new(0) })
                .resource("/", |r| r.method(http::Method::GET).f(index))
                .finish(),
        ]
    });
}
