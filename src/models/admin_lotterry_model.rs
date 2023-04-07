use serde::{Serialize, Deserialize};
use mysql::*;
use mysql::prelude::*;
use crate::config::db::conDB;


#[derive(Debug,Serialize, Deserialize)]
pub struct Lottery {
    pub lottery_id: u32,
    pub lottery_number: String,
}