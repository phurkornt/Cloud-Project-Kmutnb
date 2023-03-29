
use actix_web::{web, get ,post , Responder, HttpResponse, http::StatusCode};


#[get("/admin/prize")]
async fn get_admin_prize() -> impl Responder {
   
    
    return HttpResponse::Ok().json("OK get admin");
}
#[post("/admin/prize")]
async fn post_admin_prize() -> impl Responder {
   
    
    return HttpResponse::Ok().json("OK post admin");
}
