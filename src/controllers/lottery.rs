
use actix_web::{web, get , put , Responder, HttpResponse};

use serde::{Deserialize, Serialize};



use log::{debug};

use crate::models::lotterry_model::{*, self};
use crate::models::basket_model::get_user_count_basket;

// use std::convert::TryFrom;

// use crate::config::db::getDB;

#[derive(Debug, Deserialize, Serialize)]
struct Payment {
    lottery_number: String,
}


#[derive(Debug, Deserialize, Serialize)]
struct LotteryData {
    lotteries: Vec<Lottery>,
}


#[derive(Deserialize , Serialize)]
struct GetUserData {
    user_id:i32
}

#[get("/lottery")]
async fn get_lottery(user_id: web::Json<GetUserData>) -> impl Responder {

    
    if user_id.user_id == 1{
    
        let lottery_item = lotterry_model::get_lottery();
        
        let res = UserLottery{
            user_basket_count:get_user_count_basket(),
            lottery_all:lottery_item
        };

        
        return HttpResponse::Ok().json(res);

    }else{

        return HttpResponse::Unauthorized().json("Error");
    }
    
    
}


#[put("/lottery")]
async fn put_lottery(lottery: web::Json<Vec<Lottery>>) -> impl Responder {
    debug!("{:?}", lottery);
    for i in &lottery[..] {
        println!("Lottery ID: {}", i.lottery_id);
        println!("Lottery number: {}", i.lottery_number);
    }

    // Next action at DB

    HttpResponse::Ok().body("Lotteries received")
    // รอใช้ DB
    // return HttpResponse::Ok().json(lottery);
    
    
}


