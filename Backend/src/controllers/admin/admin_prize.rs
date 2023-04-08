
use actix_web::{web, get ,post , Responder, HttpResponse};



use crate::models::admin_prize_model::*;




#[get("/admin/prize")] //[/]
async fn get_admin_prize(admin: web::Json<AdminID>) -> impl Responder {
   
    if admin.admin_id == 1{
    
        let reward_item = get_reward();
        return HttpResponse::Ok().json(reward_item);
    }else{

        return HttpResponse::Unauthorized().json("Error");
    }
}

#[post("/admin/prize")]
async fn post_admin_prize(reward: web::Json<AdminIDAndReward>) -> impl Responder {
    
    let reward = reward.into_inner();
    let reward_number = reward.reward_number;


    if reward.admin_id == 1{
    
        if is_reward_out() == "No"{

            // insert
            insert_reward(reward_number);
            return HttpResponse::Ok().json("succeed");
        }else{
            return HttpResponse::PreconditionFailed().json("ได้ออกรางวัลสำหรับวันนี้เเล้ว");
        }

    }else{

        return HttpResponse::Unauthorized().json("Error");
    }
    

}
