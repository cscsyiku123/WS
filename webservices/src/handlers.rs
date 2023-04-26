use actix_web::{HttpResponse, web};
use chrono::Utc;

use crate::moudels::Course;

use super::state::AppState;

pub async fn health_checkout_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let response = &app_state.health_check_response.clone();
    let mut count = app_state.visit_count.lock().unwrap();
    *count += 1;
    let response = format!("{} {}", response, count);
    HttpResponse::Ok().json(&response)
}

pub async fn new_course(
    new_course: web::Json<Course>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    println!("receive new course {:?}", new_course);
    let course_count = app_state.courses.lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|course| course.teacher_id == new_course.teacher_id).collect::<Vec<Course>>()
        .len();


    let new_course = Course {
        teacher_id: new_course.teacher_id,
        id: Some(course_count + 1),
        name: new_course.name.clone(),
        time: Some(Utc::now().naive_utc()),
    };
    app_state.courses.lock().unwrap().push(new_course);
    HttpResponse::Ok().json("course ok")
}
