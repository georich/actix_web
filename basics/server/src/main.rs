extern crate actix;
extern crate actix_web;
use actix_web::{server::HttpServer, App, HttpResponse};

fn main() {
    let sys = actix::System::new("guide");

    HttpServer::new(|| App::new().resource("/", |r| r.f(|_| HttpResponse::Ok())))
        .bind("127.0.0.1:59080")
        .unwrap()
        .start();

    let _ = sys.run();
}
