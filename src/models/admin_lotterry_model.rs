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
pub struct AdminIDCount {
    pub admin_id: u32,
    pub lottery_count:u32
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AdminIdDate {
    pub admin_id: u32,
    pub date:String
}





//  ---------------------------------- Data Output ----------------------------------

#[derive(Debug,Serialize, Deserialize)]
pub struct LotteryDateCount {
    pub date: String,
    pub lottery_count: u32
}

#[derive(Debug,Serialize, Deserialize)]
pub struct Date {
    pub date: String,
}




// ####################### get_reward(ดูรางวัลทั้งหมด) reward_number, DATE(Datetime) #######################
pub fn get_lottery(date:String) -> LotteryDateCount{

    let db = conDB()
    .map(|mut conn| {
        conn.query_map(
            // เพิม่ Date ด้วย
            "SELECT Date(Datetime) as date , COUNT(lottery_number) as count FROM lottery WHERE Date(Datetime) = '".to_owned() + date.as_str()+" ' ",
            |(date , lottery_count)| {
                LotteryDateCount 
                {   
                    date,
                    lottery_count
                }
            },
        )
    })
    .unwrap_or_else(|_| {
        Ok(Vec::new())
    });
   
    let mut data:LotteryDateCount = LotteryDateCount { date: String::new(), lottery_count: 0 };
    match db {
        Ok(lotteries) => {
            for i in lotteries{
                data =  i;
                break;
            }
        }
        Err(e) => println!("Error: {}", e)
    }

    return data;
    
}




// ####################### get_date_unique (วันที่เพื่อนำไปประกอบข้อมูล) ->["11/11/111" , "11/11/111"] #######################
pub fn get_date_unique() -> Vec<Date>{
    let db = conDB()
    .map(|mut conn| {
        conn.query_map(
            // เพิม่ Date ด้วย
            "SELECT DISTINCT Date(Datetime) FROM lottery;",
            |date| {
                Date 
                {   
                    date
                }
            },
        )
    })
    .unwrap_or_else(|_| {
        Ok(Vec::new())
    });
   
    let mut data:Vec<Date> =  Default::default();
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



// ####################### insert_lottery ->"123456" #######################
pub fn insert_lottery(lottery_number:String){
    let time_now: DateTime<Local> = Local::now();
    let formatted = time_now.format("%Y-%m-%d %H:%M:%S").to_string();

    let _ = match conDB() {
        Ok(mut conn) => {
            conn.exec_drop(
            "INSERT INTO lottery(lottery_number, lottery_status, Datetime) VALUES (:lottery_number,'available',:time_now)",
            params! {
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



// ####################### delete_lottery_byDate ->"" #######################
pub fn delete_lottery_by_date(date:String){
    let _ = match conDB() {
        Ok(mut conn) => {
            conn.exec_drop(
            "DELETE FROM lottery WHERE Date(Datetime) = :date ;",
            params! {
                "date" => date
            },
        )},
        Err(e) => {
            println!("Failed to get DB connection: {}", e);
            return;
        }
    };
}
