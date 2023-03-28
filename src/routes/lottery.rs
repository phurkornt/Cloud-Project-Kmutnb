use actix_web::web;
use crate::controllers::lottery::{get_lottery} ;


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(get_lottery);
}