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


