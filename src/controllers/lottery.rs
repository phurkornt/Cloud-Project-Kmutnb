
use actix_web::{web, get , put , Responder, HttpResponse, http::StatusCode};
use serde_json::json;
use serde::Deserialize;
use log::{debug};



use crate::models::lotterry_model::{Lottery,UserLottery};
use std::convert::TryFrom;

#[derive(Deserialize)]
struct GetUserData {
    id:i32
}

#[get("/lottery")]
async fn get_lottery(user_id: web::Json<GetUserData>) -> impl Responder {
    // match id and return all lottery  and basket count
    if user_id.id == 1{
        
        // println!("{:#?}", user_lottery);
        

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

        
        let res = UserLottery{
            user_basket_count:u32::try_from(lottery_item.len()).unwrap(),
            lottery_all:lottery_item
        };

        return HttpResponse::Ok().json(res);

    }else{

        return HttpResponse::Unauthorized().json("Error");
    }
    
    
}


#[put("/lottery")]
async fn put_lottery(lottery: web::Json<Lottery>) -> impl Responder {
    

    // รอใช้ DB
    return HttpResponse::Ok().json(lottery);
    
    
}

