use serde::{Serialize, Deserialize};
use mysql::*;
use mysql::prelude::*;
use crate::config::db::getDB;


#[derive(Debug,Serialize, Deserialize)]
pub struct Lottery {
    pub lottery_id: u32,
    pub lottery_number: String,
}


// ====================== LotteryWithUserID ======================
#[derive(Debug, Deserialize)]// >
pub struct LotteryWithUserID {
    pub user_id: i32,
    pub lottery: Lottery
}


// ====================== LotteryCount ======================
#[derive(Debug,Serialize, Deserialize)] //>
pub struct LotteryCount {
    pub lottery_count: i32
}

// ====================== LotteryCount ======================
#[derive(Debug,Serialize, Deserialize)] //>
pub struct LotteryIDwithUserID {
    pub user_id: i32,
    pub lottery_id: i32
}

#[derive(Debug,Serialize, Deserialize)] //>
pub struct LotteryArrayID {
    pub lottery_id:Vec<i32>
}

#[derive(Debug,Serialize, Deserialize)] //>
pub struct CountBasketUser {
    pub count:u32
}


//-----------------------------------------------------------------------------


// --------------------- Get count lottery ของ user  ---------------------
pub fn get_user_count_basket() -> u32{
    let db = getDB()
    .map(|mut conn| {
        conn.query_map(
            "SELECT COUNT(lottery_id) FROM basket WHERE user_id = 1;",
            |(count)| {
                CountBasketUser 
                {   
                    count
                }
            },
        )
    })
    .unwrap_or_else(|_| {
        // กรณีเกิด error หรือไม่สามารถเชื่อมต่อฐานข้อมูลได้
        // return ค่า default ของ Vec<Payment>
        Ok(Vec::new())
    });
   
    let mut count:u32 = 0;
    match db {
        Ok(number) => {
            count = number[0].count;
        }
        Err(e) => println!("Error: {}", e)
    }

    return count;
    
}