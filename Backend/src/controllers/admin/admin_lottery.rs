
use actix_web::{web, get ,post ,delete , Responder, HttpResponse };

// use log::debug;


use crate::models::admin_lotterry_model::*;
use rand::seq::SliceRandom;


#[get("/admin/lottery")] //[/]
async fn get_admin_lottery(admin: web::Json<AdminID>) -> impl Responder {

    if admin.admin_id == 1{
        let all_date = get_date_unique();
        let mut lottery_detail:Vec<LotteryDateCount> =  Default::default();
        for i in all_date{
            lottery_detail.push(get_lottery(i.date));
        }
        return HttpResponse::Ok().json(lottery_detail);
    }else{

        return HttpResponse::Unauthorized().json("Error");
    }


}

#[post("/admin/lottery")] //[/]
async fn post_admin_lottery(admin: web::Json<AdminIDCount>) -> impl Responder {
    let admin = admin.into_inner();
    if admin.admin_id == 1{
        let number = random_numbers(admin.lottery_count as usize);
        for i in 0..admin.lottery_count{
            // debug!("Test Get {:?}","1");
            insert_lottery(number[i as usize].to_string());
        }

        return HttpResponse::Ok().json("succeed");

    }else{

        return HttpResponse::Unauthorized().json("Error");
    }
}

#[delete("/admin/lottery")] //[/]
async fn delete_admin_lottery(admin: web::Json<AdminIdDate>) -> impl Responder {
    let admin = admin.into_inner();
    if admin.admin_id == 1{

        delete_lottery_by_date(admin.date);
        return HttpResponse::Ok().json("succeed");

    }else{

        return HttpResponse::Unauthorized().json("Error");
    }
    
}







/* ---------------------------- Function ---------------------------- */

fn random_numbers(n: usize) -> Vec<String> {
    let mut numbers: Vec<String> = Vec::new();
    let digits: Vec<u8> = (0..=9).collect();
    let mut rng = rand::thread_rng();
    
    while numbers.len() < n {
        let mut nums: Vec<u8> = digits.clone();
        nums.shuffle(&mut rng);
        let num_str = nums[..6].iter().map(|d| d.to_string()).collect::<String>();
        if !numbers.contains(&num_str) {
            numbers.push(num_str);
        }
    }
    
    numbers
}