
use actix_web::{web, post , Responder, HttpResponse, http::StatusCode};
use serde_json::json;
use serde::Deserialize;
use log::{debug};



// use crate::models::lotterry::{Lottery,UserLottery};
// use std::convert::TryFrom;

#[post("/customer/lottery")]
async fn post_customer_lottery() -> impl Responder {
   
    
    return HttpResponse::Ok().json("OK post");
}

