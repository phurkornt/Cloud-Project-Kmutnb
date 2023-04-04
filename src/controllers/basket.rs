
use actix_web::{web, get ,post,delete , Responder, HttpResponse, http::StatusCode};
use serde_json::json;
use serde::Deserialize;
use log::{debug};



use crate::models::basket_model::*;
// use std::convert::TryFrom;


#[derive(Debug , Deserialize)]
struct GetUserData {
    user_id:i32
}

#[post("/basket")]//[/]
async fn post_basket(lottery: web::Json<LotteryWithUserID>) -> impl Responder {
   
    // [1] insert ลง db ตะกร้า
    debug!("TEST {:?}",&lottery);
    // [2] res จำนวน lottery ในตะกร้า 
    
    let count = LotteryCount { 
        lottery_count: 50 
    };
    
    return HttpResponse::Ok().json(&count);
}


#[get("/basket")]//[/]
async fn get_basket(user_id: web::Json<GetUserData>) -> impl Responder {
   
    // [1] ตรวจสอบ userID
    debug!("TEST {:?}",&user_id);
    if user_id.user_id == 1{
        // [2] res select lottery ทั้งหมดในตะกร้า user
        let lottery_item = vec![
            Lottery {
                lottery_id: 1,
                lottery_number: "123456".to_string()
            },
            Lottery {
                lottery_id: 2,
                lottery_number: "123456".to_string()
            },
            Lottery {
                lottery_id: 3,
                lottery_number: "123456".to_string()
            }
        ];

        HttpResponse::Ok().json(lottery_item)

    }else{
        HttpResponse::Unauthorized().json("Error")
    }
    
}


#[delete("/basket")]//[/]
async fn delete_basket(lottery: web::Json<LotteryIDwithUserID>) -> impl Responder {
    
    // [1] ตรวจสอบ userID
    debug!("TEST {:?}",&lottery);
    // [2] delete lottery by id ในตะกร้า 

    return HttpResponse::Ok().json("OK delete");
}


#[get("/basket/verification")]//[]
async fn get_basket_verification(lottery: web::Json<LotteryArrayID>) -> impl Responder {

    // [1] ตรวจสอบ userID
    debug!("TEST {:?}",&lottery);

    
    // [2] ตรวจ lottery by id ในตะกร้า ว่ามีคนซื้อไปยัง 
    let lottery_item = vec![
            Lottery {
                lottery_id: 8,
                lottery_number: "123456".to_string()
            },
            Lottery {
                lottery_id: 5,
                lottery_number: "123456".to_string()
            }
        ];

    return HttpResponse::Ok().json(lottery_item);
}