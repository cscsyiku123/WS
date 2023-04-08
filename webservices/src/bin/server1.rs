use actix_web::{web,App,HttpResponse,HttpServer,Responder};
use std::io;
pub fn general_routes(cfg: &mut web::ServiceConfig){
    cfg.route("/health",web::get().to(health_checkout_handler));
}
pub async fn health_checkout_handler()->impl Responder{
     HttpResponse::Ok().json("Actix web service is runing!")
}



 #[actix_rt::main]
 async fn main()->io::Result<()>{
     let app=move || App::new().configure(general_routes);
     HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
 }

