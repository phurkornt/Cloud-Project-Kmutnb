use serde::{Serialize, Deserialize};
use mysql::*;
use mysql::prelude::*;

use crate::config::db::conDB;
use chrono::{Local, DateTime};


//  ---------------------------------- Data Input ----------------------------------
#[derive(Debug,Serialize, Deserialize)]
pub struct LotteryID {
    pub lottery_id: u32,
    pub lottery_number: String
}
#[derive(Debug,Serialize, Deserialize , Default)]
pub struct LotteryList {
    pub user_id: u32,
    pub lottery: Vec<LotteryID>
}



#[derive(Debug,Serialize, Deserialize)]
pub struct Lottery {
    pub lottery_number: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LotteryWithUserID {
    pub user_id: u32,
    pub lottery_number: Vec<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserID {
    pub user_id: u32,
}



//  ---------------------------------- Data Output ----------------------------------

#[derive(Debug,Serialize, Deserialize)]
pub struct LotteryCount {
    pub lottery_count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LotteryWithDate {
    pub lottery_number: String,
    pub datetime: String
}




//  ---------------------------------- Function ----------------------------------

// ####################### insert_user_history with ["111111" ,"123345"] , "user_id : 1" #######################
pub fn insert_user_history(user_id:u32 , lottery_number:String){
    let time_now: DateTime<Local> = Local::now();
    let formatted = time_now.format("%Y-%m-%d %H:%M:%S").to_string();

    let _ = match conDB() {
        Ok(mut conn) => {
            conn.exec_drop(
            "INSERT INTO user_history (`user_id`, `lottery_number`, `Datetime`) VALUES (:user_id,:lottery_number,:time_now);",
            params! {
                "user_id" => user_id,
                "lottery_number" => lottery_number,
                "time_now" => formatted
            },
        )},
        Err(e) => {
            println!("Failed to get DB connection: {}", e);
            return;
        }
    };
    
}


// ####################### get_user_history with "user_id : 1" #######################
pub fn get_user_history(user_id:u32) -> Vec<LotteryWithDate>{
    let db = conDB()
    .map(|mut conn| {
        conn.query_map(
            "SELECT lottery_number, Datetime FROM user_history WHERE user_id = ".to_owned() + user_id.to_string().as_str(),
            |(lottery_number ,datetime)| {
                LotteryWithDate 
                {   
                    lottery_number,
                    datetime
                }
            },
        )
    })
    .unwrap_or_else(|_| {
        Ok(Vec::new())
    });
   
    let mut data:Vec<LotteryWithDate> = Default::default();
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

