
use actix_web::{web, post , get  , Responder, HttpResponse};
use serde_json::json;
use log::{debug};


use crate::models::customer_model::*;
use crate::models::basket_model::{Lottery};


// use std::convert::TryFrom;

#[post("/customer")]
async fn post_customer_lottery(lottery_number: web::Json<LotteryWithUserID>) -> impl Responder {
   
    debug!("{:?}" , lottery_number);
    
    let lot = lottery_number.into_inner();

    for i in lot.lottery_number{
        insert_user_history(lot.user_id ,i);
    }

    return HttpResponse::Ok();
}

#[post("/customer/purchasing")]
async fn post_customer_purchasing(lottery: web::Json<LotteryList>) -> impl Responder {
    
    debug!("TesT1 -> {:?}", lottery);

    let lot = &lottery.into_inner();

    // ------- ประกอบชุดข้อมูล -------
    let mut lot_id:Vec<u32> =  Vec::new();
    let mut lot_number:Vec<String> =  Vec::new();
    
    for i in &lot.lottery{
        lot_id.push(i.lottery_id);
        lot_number.push(i.lottery_number.clone());
    }
    let struct_insert = LotteryWithUserID{
        user_id:lot.user_id,
        lottery_number:lot_number
    };

    let client = reqwest::Client::new();
    let body = json!({"lottery_id":lot_id });
    let response = client
        .get("http://127.0.0.1:3000/basket/verification")
        .json(&body)
        .send()
        .await
        .unwrap()
        .json::<Vec<Lottery>>() // แปลง JSON เป็น struct LotteryList
        .await
        .unwrap();

    if response.len() <= 0{

        let body_insert = json!(struct_insert);
        let _ = client
            .post("http://127.0.0.1:3000/customer")
            .json(&body_insert)
            .send()
            .await
            .unwrap();


        let body_status = json!({"lottery_id":lot_id , "status":"sold-out" });
        let _ = client
            .put("http://127.0.0.1:3000/lottery")
            .json(&body_status)
            .send()
            .await
            .unwrap();
        
        for i in lot_id{
            let body_delete = json!({"user_id":lot.user_id , "lottery_id":i });
            let _ = client
                .delete("http://127.0.0.1:3000/basket")
                .json(&body_delete)
                .send()
                .await
                .unwrap();
        }

        HttpResponse::Ok().json("")
    }else{
        HttpResponse::Ok().json(response)
    }

    // api in this function
    // ตรวจสอบเลขซ้่ำ [/]
    // insert ลง history ลูกค้า [/]
    // เปลี่ยน status ของ lottery หลัก [/]
    // ลบสินค้าในตระกร้า ทั้งหมด [/]

    // for i in lot.lottery_number{
    //     insert_user_history(lot.user_id ,i);
    // }

}

    
#[get("/customer")]
async fn get_customer_lottery(user: web::Json<UserID>) -> impl Responder {
   
    // debug!("{:?}" , lottery_number);
    
    let user_data =  user.into_inner();
    if user_data.user_id == 1{
        
        let data = get_user_history(user_data.user_id);
        debug!("testRead {:?}",data);
        
        return HttpResponse::Ok().json(data);

    }else{

        return HttpResponse::Unauthorized().json("Error");
    }

}


