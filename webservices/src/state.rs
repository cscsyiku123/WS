use std::sync::Mutex;
use crate::moudels::Course;

// 应用程序状态
pub struct AppState{
    pub health_check_response:String,
    pub visit_count:Mutex<u32>,
    pub courses:Mutex<Vec<Course>>,
}
