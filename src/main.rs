extern crate actix_web;
extern crate listenfd;

use listenfd::ListenFd;
use actix_web::{server, App};

mod controller;

fn main() {
    let host: String = "127.0.0.1".to_owned();
    let port: String = "8088".to_owned();
    let adress: String = format!("{}:{}", &host, &port);

    let mut listenfd = ListenFd::from_env();
    let mut server = server::new(|| {
        App::new()
            .resource("/", |r| r.f(controller::home::index))
            .resource("/version", |r| r.f(controller::version::index))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)
    } else {
        server.bind(&adress).unwrap()
    };

    println!("Server start on {}", &adress);
    server.run();
}
