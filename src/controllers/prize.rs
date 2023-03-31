
use actix_web::{web, get , Responder, HttpResponse, http::StatusCode};
use serde_json::json;
use serde::Deserialize;
use log::{debug};



use crate::models::prize_model::{LotteryNumber , LotteryReward};
// use std::convert::TryFrom;


#[get("/prize")]
async fn get_prize(lottery_number: web::Json<LotteryNumber>) -> impl Responder {
    debug!("{:?}" , &lottery_number);

    let res = LotteryReward{
        reward_name:"รางวัลที่ 1 2 3 4".to_string()
    };

    return HttpResponse::Ok().json(res);
}
