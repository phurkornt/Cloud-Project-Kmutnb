use serde::{Serialize, Deserialize};

#[derive(Debug,Serialize, Deserialize)]
pub struct Lottery {
    pub lottery_id: u32,
    pub lottery_number: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserLottery {
    pub user_basket_count: u32,
    pub lottery_all: Vec<Lottery>,
}

// ====================== LotteryWithUserID ======================
#[derive(Debug, Deserialize)]
pub struct LotteryWithUserID {
    pub user_id: i32,
    pub lottery: Lottery,
}


// ====================== LotteryCount ======================
#[derive(Debug,Serialize, Deserialize)]
pub struct LotteryCount {
    pub lottery_count: i32,
}

// ====================== LotteryCount ======================
#[derive(Debug,Serialize, Deserialize)]
pub struct LotteryIDwithUserID {
    pub user_id: i32,
    pub lottery_id: i32,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct LotteryArrayID {
    pub lottery_id:Vec<i32>

}

