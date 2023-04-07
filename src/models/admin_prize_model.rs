use serde::{Serialize, Deserialize};
use mysql::*;
use mysql::prelude::*;
use crate::config::db::conDB;
use chrono::{Local, DateTime};

//  ---------------------------------- Data Input ----------------------------------
#[derive(Debug, Deserialize, Serialize)]
pub struct AdminID {
    pub admin_id: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RewardNumber {
    pub reward_number: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct AdminIDAndReward {
    pub admin_id: u32,
    pub reward_number: String
}




//  ---------------------------------- Data Output ----------------------------------

#[derive(Debug,Serialize, Deserialize)]
pub struct Reward {
    pub lottery_number: String,
    pub date: String
}




// ####################### get_reward(ดูรางวัลทั้งหมด) reward_number, DATE(Datetime) #######################
pub fn get_reward() -> Vec<Reward>{
    let db = conDB()
    .map(|mut conn| {
        conn.query_map(
            // เพิม่ Date ด้วย
            "SELECT  reward_number, DATE(Datetime) FROM reward",
            |(lottery_number , date)| {
                Reward 
                {   
                    lottery_number,
                    date
                }
            },
        )
    })
    .unwrap_or_else(|_| {
        Ok(Vec::new())
    });
   
    let mut data:Vec<Reward> = Default::default();
    match db {
        Ok(lotteries) => {
            for i in lotteries{
                data.push(i);
            }
        }
        Err(e) => println!("Error: {}", e)
    }

    return data;
    
}



// ####################### is_reward_out(ตรวจสอบว่าวันนี้ได้ออกรางวัลหรือยัง?) with [Now data] #######################
pub fn is_reward_out() -> String{
    let time_now: DateTime<Local> = Local::now();
    let formatted = time_now.format("%Y-%m-%d").to_string();

    let db = conDB()
    .map(|mut conn| {
        conn.query_map(
            // เพิม่ Date ด้วย
            "SELECT `reward_number` FROM `reward` WHERE DATE(Datetime) ='".to_owned() + formatted.as_str() +   "' ",
            |reward_number| {
                RewardNumber 
                {   
                    reward_number
                }
            },
        )
    })
    .unwrap_or_else(|_| {
        Ok(Vec::new())
    });
   
    let mut data:String = "No".to_string() ;
    match db {
        Ok(lotteries) => {
            if lotteries.len() > 0{
                data = "Yes".to_string();
            }
        }
        Err(e) => println!("Error: {}", e)
    }

    return data;
    
}



// ####################### insert_reward with ["111111"]  #######################
pub fn insert_reward(reward_number:String){
    let time_now: DateTime<Local> = Local::now();
    let formatted = time_now.format("%Y-%m-%d %H:%M:%S").to_string();

    let _ = match conDB() {
        Ok(mut conn) => {
            conn.exec_drop(
            "INSERT INTO reward (`reward_number`, `Datetime`) VALUES (:reward_number,:time_now);",
            params! {
                "reward_number" => reward_number,
                "time_now" => formatted
            },
        )},
        Err(e) => {
            println!("Failed to get DB connection: {}", e);
            return;
        }
    };
    
}