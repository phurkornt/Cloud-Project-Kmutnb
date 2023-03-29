use actix_web::web;
use crate::controllers::prize::{get_prize} ;


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(get_prize);
}