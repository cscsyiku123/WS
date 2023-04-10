use std::sync::Mutex;

// 应用程序状态
pub struct AppState{
    pub health_check_response:String,
    pub visit_count:Mutex<u32>
}
