use actix_web::middleware::Logger;

pub fn init() -> Logger {
    return Logger::new("%r - %s - %a - %D");
}