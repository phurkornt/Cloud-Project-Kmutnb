use serde::{Serialize, Deserialize};



//  ---------------------------------- Data Input ----------------------------------

#[derive(Debug,Serialize, Deserialize)]
pub struct Lottery {
    pub lottery_number: String
}

#[derive(Debug, Deserialize)]
pub struct LotteryWithUserID {
    pub user_id: i32,
    pub lottery: Vec<Lottery>
}






//  ---------------------------------- Data Output ----------------------------------

#[derive(Debug,Serialize, Deserialize)]
pub struct LotteryCount {
    pub lottery_count: i32,
}