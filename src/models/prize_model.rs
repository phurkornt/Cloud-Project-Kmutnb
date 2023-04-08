use serde::{Serialize, Deserialize};
use mysql::*;
use mysql::prelude::*;

use crate::config::db::conDB;
use chrono::{Local, DateTime};
//  ---------------------------------- Data Input ----------------------------------
#[derive(Debug,Serialize, Deserialize)]
pub struct LotteryNumber {
    pub lottery_number: String
}





//  ---------------------------------- Data Output ----------------------------------

#[derive(Debug,Serialize, Deserialize)]
pub struct LotteryReward {
    pub reward_name: String,
    pub money: u32
}
#[derive(Debug,Serialize, Deserialize)]
pub struct RewardNumber {
    pub lottery_number: String,
}








// ####################### get 1st prize with "lottery_number : 123456" #######################
pub fn get_prize_with_date() -> String{
    let time_now: DateTime<Local> = Local::now();
    let formatted = time_now.format("%Y-%m-%d").to_string();

    let db = conDB()
    .map(|mut conn| {
        conn.query_map(
            " SELECT reward_number FROM reward WHERE DATE(Datetime) = '".to_owned() + formatted.as_str() + "' ",
            |lottery_number| {
                RewardNumber 
                {   
                    lottery_number
                }
            },
        )
    })
    .unwrap_or_else(|_| {
        Ok(Vec::new())
    });
   
    let mut data:String = "".to_string();

    match db {
        Ok(lottery) => {
            if lottery.len() > 0{
                data = lottery[0].lottery_number.clone();
            }
        }
        Err(e) => println!("Error: {}", e)
    }

    return data;
}
