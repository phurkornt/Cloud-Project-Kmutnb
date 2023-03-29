
use actix_web::{web, get , Responder, HttpResponse, http::StatusCode};
use serde_json::json;
use serde::Deserialize;
use log::{debug};



// use crate::models::lotterry::{Lottery,UserLottery};
// use std::convert::TryFrom;


#[get("/prize")]
async fn get_prize() -> impl Responder {
    return HttpResponse::Ok().json("OK get");
}
