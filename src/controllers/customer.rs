
use actix_web::{web, post , Responder, HttpResponse};
use serde_json::json;
use serde::Deserialize;
use log::{debug};



use crate::models::customer_model::{Lottery,LotteryCount,LotteryWithUserID};
// use std::convert::TryFrom;

#[post("/customer/lottery")]
async fn post_customer_lottery(lottery_number: web::Json<LotteryWithUserID>) -> impl Responder {
   
    debug!("{:?}" , lottery_number);
    
    let res = LotteryCount{
        lottery_count:10
    };


    return HttpResponse::Ok().json(res);
}

