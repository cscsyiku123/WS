use actix_web::{App, HttpServer, web};
use routes::*;
use state::AppState;
use std::io;
use std::sync::Mutex;

#[path = "../handlers.rs"]
mod handlers;
#[path = "../routes.rs"]
mod routes;
#[path = "../state.rs"]
mod state;
#[path = "../models.rs"]
mod moudels;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let app = move || {
        App::new()
            .app_data(web::Data::new(AppState {
                health_check_response: String::from("i am ok !"),
                visit_count: Mutex::new(0),
                courses: Mutex::new(vec![]),
            }))
            .configure(general_routes)
            .configure(course_routes)
    };
    let server = HttpServer::new(app).bind("127.0.0.1:3000")?.run().await;
    server
}
