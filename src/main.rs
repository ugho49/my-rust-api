#[macro_use]
extern crate log;
extern crate env_logger;
extern crate actix_web;
extern crate listenfd;

use listenfd::ListenFd;
use actix_web::{server, App};

mod routes;
mod middlewares;

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info,my_rust_api=info");
    env_logger::init();

    info!("Starting up ...");

    let host: String = "127.0.0.1".to_owned();
    let port: String = "8088".to_owned();
    let adress: String = format!("{}:{}", &host, &port);

    let mut listenfd = ListenFd::from_env();
    let mut server = server::new(|| {
        let mut app = App::new();
        app = routes::add_all(app);
        app = middlewares::add_all(app);
        return app;
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)
    } else {
        server.bind(&adress).unwrap()
    };

    info!("Server run on http://{}", &adress);
    server.run();
}
