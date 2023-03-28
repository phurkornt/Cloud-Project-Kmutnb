use actix_web::{web,get, Responder, HttpResponse};
use serde_json::json;
use log::{info, debug};

use crate::models::lotterry::Lottery;

use serde::Deserialize;

#[derive(Deserialize)]
struct UserData {
    id:i32
}

#[get("/lottery")]
async fn get_lottery(user_id: web::Json<UserData>) -> impl Responder {
    
    if user_id.id == 1{
        let lottery_item = vec![
        Lottery {
            lot_id: 1,
            lot_number: "123456".to_string()
        },
        Lottery {
            lot_id: 2,
            lot_number: "654321".to_string()
        }
        ];
        let response_body = json!(lottery_item);

        return HttpResponse::Ok().json(response_body);

    }else{

        return HttpResponse::Unauthorized().json("Error");
    }
    
    
}