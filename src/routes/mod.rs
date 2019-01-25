use actix_web::{App};

mod version;
mod home;

pub fn add_all(app: App) -> App {
    return app
        .resource("/", |r| r.f(home::index))
        .resource("/version", |r| r.f(version::index));
}
