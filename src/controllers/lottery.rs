
use actix_web::{web, get , put , Responder, HttpResponse};

use crate::models::lotterry_model::{*, self};
use crate::models::basket_model::get_user_count_basket;

#[get("/lottery")] //[/]
async fn get_lottery(user_id: web::Json<GetUserData>) -> impl Responder {

    
    if user_id.user_id == 1{
    
        let lottery_item = lotterry_model::get_lottery();
        
        let res = UserLottery{
            user_basket_count:get_user_count_basket(user_id.user_id.try_into().unwrap()),
            lottery_all:lottery_item
        };

        
        return HttpResponse::Ok().json(res);

    }else{

        return HttpResponse::Unauthorized().json("Error");
    }
    
    
}


#[put("/lottery")] //[/]
async fn put_lottery(lottery: web::Json<UpdateStatus>) -> impl Responder {
    let lot = lottery.into_inner();
    update_lottery_status(lot.lottery_id ,lot.status);

    HttpResponse::Ok().body("")
}


