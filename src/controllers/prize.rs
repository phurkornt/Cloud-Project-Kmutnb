
use actix_web::{web, get , Responder, HttpResponse, http::StatusCode};
use serde_json::json;
use serde::Deserialize;
use log::{debug};



use crate::models::prize_model::{LotteryNumber , LotteryReward ,get_prize_with_date};
// use std::convert::TryFrom;


#[get("/prize")]
async fn get_prize(lottery_number: web::Json<LotteryNumber>) -> impl Responder {
    debug!("{:?}" , &lottery_number);

    //  * ## เงื่อนไขรางวัล ##
    //  * ที่ 1 เลขถูกทุกตัว
    //  * 3ตัวหน้า 
    //  * 3ตัวหลัง 
    let lot_number = lottery_number.into_inner();
    let prize = get_prize_with_date();


    let mut reward_name:String;
    let mut money:u32;

    if prize == lot_number.lottery_number{
        reward_name = "รางวัลที่1".to_string();
        money = 10000;
    }else if prize[0..3] == lot_number.lottery_number[0..3] {
        reward_name = "รางวัล3ตัวหน้า".to_string();
        money = 3000;
    }else if prize[4..] == lot_number.lottery_number[4..] {
        reward_name = "รางวัล3ตัวท้าย".to_string();
        money = 3000;
    }else{
        reward_name = "คุณไม่ถูกรางวัล".to_string();
        money = 0;
    }

    // debug!("Test reward : {}",prize);
    let res = LotteryReward{
        reward_name:reward_name,
        money:money
    };

    return HttpResponse::Ok().json(res);
}
