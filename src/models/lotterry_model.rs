use serde::{Serialize, Deserialize};

use mysql::*;
use mysql::prelude::*;

use crate::config::db::conDB;

#[derive(Debug,Serialize, Deserialize)]
pub struct Lottery {
    pub lottery_id: u32,
    pub lottery_number: String
}

#[derive(Debug,Serialize, Deserialize , Default)]
pub struct UserLottery {
    pub user_basket_count: u32,
    pub lottery_all: Vec<Lottery>
}

#[derive(Debug,Serialize, Deserialize , Default)]
pub struct LotteryList {
    pub lottery_all: Vec<Lottery>
}



// --------------------- Get All lottery in db ---------------------
pub fn get_lottery() -> Vec<Lottery>{
    let db = conDB()
    .map(|mut conn| {
        conn.query_map(
            // เพิม่ Date ด้วย
            "SELECT lottery_id,lottery_number FROM lottery where lottery_status = 'available'; ",
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
        Ok(lotteries) => {
            for i in lotteries{
                data.push(i);
            }
        }
        Err(e) => println!("Error: {}", e)
    }

    return data;
    
}





/*



// let mut tt:UserLottery = Default::default();
// match db {
//     Ok(lotteries) => {
//         tt = UserLottery{
//             user_basket_count:u32::try_from(lotteries.len()).unwrap(),
//             lottery_all:lotteries
//         };
//         // for lottery in lotteries {
//         //     debug!("Lottery ID: {}, Lottery Number: {}", lottery.lottery_id, lottery.lottery_number);
//         // }
//     }
//     Err(e) => println!("Error: {}", e)
// }
 */


