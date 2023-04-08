
use actix_web::{web, get ,post,delete , Responder, HttpResponse };
// use log::{debug};

use crate::models::basket_model::*;


#[post("/basket")] //[/]
async fn post_basket(lottery: web::Json<LotteryWithUserID>) -> impl Responder {

    let vail_lot = get_user_lottery_id();
    let mut is_same = false;
    for i in vail_lot{
        if i.lottery_id == lottery.lottery.lottery_id {
            is_same = true;
            break;
        }
    }
    if is_same == false{
        // [1] insert ลง db ตะกร้า   
        insert_user_basket( lottery.user_id , lottery.lottery.lottery_id);
        let count = LotteryCount { 
            lottery_count: get_user_count_basket(lottery.user_id.try_into().unwrap())
        };
        // [2] res จำนวน lottery ในตะกร้า 
        return HttpResponse::Ok().json(&count);
    }else{
        return HttpResponse::Conflict().json("มีเลขนี้อยู่เเล้ว");
    }

    
    
    
}


#[get("/basket")] //[/]
async fn get_basket(user_id: web::Json<GetUserData>) -> impl Responder {
   
    // [1] ตรวจสอบ userID
    // debug!("TEST {:?}",&user_id);
    if user_id.user_id == 1{
        // [2] res select lottery ทั้งหมดในตะกร้า user
        let lottery_item = get_user_lottery_number();

        HttpResponse::Ok().json(lottery_item)

    }else{
        HttpResponse::Unauthorized().json("Error")
    }
    
}



#[delete("/basket")] //[/]
async fn delete_basket(lottery: web::Json<LotteryIDwithUserID>) -> impl Responder {
    
    // [1] ตรวจสอบ userID
    // debug!("TEST {:?}",&lottery);
    // lottery.user_id , lottery.lottery.lottery_id
    delete_user_basket(lottery.user_id , lottery.lottery_id);
    // [2] delete lottery by id ในตะกร้า 
    return HttpResponse::Ok().json("OK delete");
}


#[get("/basket/verification")] //[/]
async fn get_basket_verification(lottery: web::Json<LotteryArrayID>) -> impl Responder {

    // [1] ตรวจสอบ userID
    // debug!("TEST verification {:?}",&lottery);
    // [2] ตรวจ lottery by id ในตะกร้า ว่ามีคนซื้อไปยัง 
    let lottery_item =get_user_lottery_soldout(lottery.into_inner().lottery_id);

    return HttpResponse::Ok().json(lottery_item);
}