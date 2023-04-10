use actix_web::{HttpResponse, web};

use super::state::AppState;

pub async fn health_checkout_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let response = &app_state.health_check_response.clone();
    let mut count = app_state.visit_count.lock().unwrap();
    *count += 1;
    let response = format!("{} {}", response, count);
    HttpResponse::Ok().json(&response)
}
