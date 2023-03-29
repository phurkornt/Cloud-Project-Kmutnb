
use actix_web::{web, get ,post ,delete , Responder, HttpResponse, http::StatusCode};


#[get("/admin/lottery")]
async fn get_admin_lottery() -> impl Responder {
   
    
    return HttpResponse::Ok().json("OK get admin");
}
#[post("/admin/lottery")]
async fn post_admin_lottery() -> impl Responder {
   
    
    return HttpResponse::Ok().json("OK post admin");
}
#[delete("/admin/lottery")]
async fn delete_admin_lottery() -> impl Responder {
   
    
    return HttpResponse::Ok().json("OK delete admin");
}
