use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Lottery {
    pub lot_id: i32,
    pub lot_number: String,
}