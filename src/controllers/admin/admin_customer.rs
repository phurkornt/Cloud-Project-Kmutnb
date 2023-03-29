
use actix_web::{web, get ,delete , Responder, HttpResponse, http::StatusCode};


#[get("/admin/customer")]
async fn get_admin_customer() -> impl Responder {
   
    
    return HttpResponse::Ok().json("OK get admin");
}

#[delete("/admin/customer")]
async fn delete_admin_customer() -> impl Responder {
   
    
    return HttpResponse::Ok().json("OK delete admin");
}
