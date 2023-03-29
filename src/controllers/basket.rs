
use actix_web::{web, get ,post,delete , Responder, HttpResponse, http::StatusCode};
use serde_json::json;
use serde::Deserialize;
use log::{debug};



// use crate::models::lotterry::{Lottery,UserLottery};
// use std::convert::TryFrom;


#[get("/basket")]
async fn get_basket() -> impl Responder {
   
    
    return HttpResponse::Ok().json("OK get");
}

#[post("/basket")]
async fn post_basket() -> impl Responder {
   
    
    return HttpResponse::Ok().json("OK post");
}

#[delete("/basket")]
async fn delete_basket() -> impl Responder {
   
    
    return HttpResponse::Ok().json("OK delete");
}


#[get("/basket/verification")]
async fn get_basket_verification() -> impl Responder {
   
    
    return HttpResponse::Ok().json("OK get /verification");
}