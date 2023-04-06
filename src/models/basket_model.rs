use serde::{Serialize, Deserialize};
use mysql::*;
use mysql::prelude::*;
use crate::config::db::conDB;


#[derive(Debug,Serialize, Deserialize)]
pub struct Lottery {
    pub lottery_id: u32,
    pub lottery_number: String,
}


// ====================== LotteryWithUserID ======================
#[derive(Debug, Deserialize)]// >
pub struct LotteryWithUserID {
    pub user_id: u32,
    pub lottery: Lottery
}


// ====================== LotteryCount ======================
#[derive(Debug,Serialize, Deserialize)] //>
pub struct LotteryCount {
    pub lottery_count: u32
}

// ====================== LotteryCount ======================
#[derive(Debug,Serialize, Deserialize)] //>
pub struct LotteryIDwithUserID {
    pub user_id: u32,
    pub lottery_id: u32
}

#[derive(Debug,Serialize, Deserialize)] //>
pub struct LotteryArrayID {
    pub lottery_id:Vec<u32>
}

#[derive(Debug,Serialize, Deserialize)] //>
pub struct CountBasketUser {
    pub count:u32
}


//-----------------------------------------------------------------------------


// --------------------- Get count lottery ของ user  ---------------------
pub fn get_user_count_basket(user_id:u32) -> u32{
    let db = conDB()
    .map(|mut conn| {
        conn.query_map(
            "SELECT COUNT(lottery_id) FROM basket WHERE user_id = ".to_owned() + user_id.to_string().as_str(),
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
            count = 0 ;
            if number.len() > 0{
                count = number[0].count;
            }
        }
        Err(e) => println!("Error: {}", e)
    }

    return count;
    
}

//  ------------------ get user basket lottery -> user_id , loterry_id  ------------------
pub fn get_user_lottery_id() -> Vec<LotteryIDwithUserID>{

    let db = conDB()
    .map(|mut conn| {
        conn.query_map(
            "SELECT user_id , lottery_id FROM `basket` WHERE user_id = 1",
            |(user_id , lottery_id)| {
                LotteryIDwithUserID 
                {   
                    user_id,
                    lottery_id
                }
            },
        )
    })
    .unwrap_or_else(|_| {
        // กรณีเกิด error หรือไม่สามารถเชื่อมต่อฐานข้อมูลได้
        // return ค่า default ของ Vec<Payment>
        Ok(Vec::new())
    });
   
    let mut data:Vec<LotteryIDwithUserID> = Default::default();
    match db {
        Ok(result) => {
            for i in result{
                data.push(i);
            }
        }
        Err(e) => println!("Error: {}", e)
    }

    return data;
}


//  ------------------ get user basket lottery -> user_id , loterry_id  ------------------
pub fn get_user_lottery_number() -> Vec<Lottery>{
    let db = conDB()
    .map(|mut conn| {
        conn.query_map(
            " SELECT bt.lottery_id , lot.lottery_number FROM basket bt , lottery lot WHERE bt.user_id = 1  and bt.lottery_id = lot.lottery_id",
            |(lottery_id , lottery_number)| {
                Lottery 
                {   
                    lottery_id,
                    lottery_number
                }
            },
        )
    })
    .unwrap_or_else(|_| {
        // กรณีเกิด error หรือไม่สามารถเชื่อมต่อฐานข้อมูลได้
        // return ค่า default ของ Vec<Payment>
        Ok(Vec::new())
    });
   
    let mut data:Vec<Lottery> = Default::default();
    match db {
        Ok(result) => {
            for i in result{
                data.push(i);
            }
        }
        Err(e) => println!("Error: {}", e)
    }

    return data;
}



//  ------------------ insert user basket ------------------
pub fn insert_user_basket(user_id:u32 , lottery_id:u32){

    let _ = match conDB() {
        Ok(mut conn) => {
            conn.exec_drop(
            "INSERT INTO basket (user_id, lottery_id) VALUES (:user_id , :lottery_id)",
            params! {
                "user_id" => user_id,
                "lottery_id" => lottery_id
            },
        )},
        Err(e) => {
            println!("Failed to get DB connection: {}", e);
            return;
        }
    };
}


//  ------------------ delete user basket by lot_id ------------------
pub fn delete_user_basket(user_id:u32 , lottery_id:u32){

    let _ = match conDB() {
        Ok(mut conn) => {
            conn.exec_drop(
            "DELETE FROM basket WHERE user_id = :user_id and lottery_id = :lottery_id;",
            params! {
                "user_id" => user_id,
                "lottery_id" => lottery_id
            },
        )},
        Err(e) => {
            println!("Failed to get DB connection: {}", e);
            return;
        }
    };
}



// --------------------- Get vertifycation lottery ของ user กับ lottery หลัก   ---------------------
pub fn get_user_lottery_soldout(list_lottery:Vec<u32>) -> Vec<Lottery>{
    // SELECT bt.lottery_id , lot.lottery_number FROM basket bt , lottery lot  WHERE  bt.user_id = 1 and bt.lottery_id = lot.lottery_id and lot.lottery_id  in (1,4) and lot.lottery_status = "sold-out"
    let list_lottery_string = list_lottery.iter().map(|n| n.to_string()).collect::<Vec<String>>().join(",");
    let list_lottery_str: &str = &list_lottery_string;
    let db = conDB()
    .map(|mut conn| {
        conn.query_map(
            "  SELECT lot.lottery_id , lot.lottery_number FROM lottery lot  WHERE  lot.lottery_status = 'sold-out'  and lot.lottery_id in ( ".to_owned() +list_lottery_str+ "  ) ",
            |(lottery_id , lottery_number)| {
                Lottery 
                {   
                    lottery_id,
                    lottery_number
                }
            },
        )
    })
    .unwrap_or_else(|_| {
        Ok(Vec::new())
    });

    let mut data:Vec<Lottery> = Default::default();
    match db {
        Ok(result) => {
            for i in result{
                data.push(i);
            }
        }
        Err(e) => println!("Error: {}", e)
    }
 
    return data;
    
}




// --------------------- insert to history for user  ---------------------

// wait 