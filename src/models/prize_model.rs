use serde::{Serialize, Deserialize};


//  ---------------------------------- Data Input ----------------------------------
#[derive(Debug,Serialize, Deserialize)]
pub struct LotteryNumber {
    pub lottery_number: String
}



//  ---------------------------------- Data Output ----------------------------------

#[derive(Debug,Serialize, Deserialize)]
pub struct LotteryReward {
    pub reward_name: String
}
