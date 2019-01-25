use actix_web::App;

mod logger;

pub fn add_all(app: App) -> App {
    return app.middleware(logger::init());
}