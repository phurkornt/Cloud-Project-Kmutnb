use actix_web::web;
use crate::controllers::customer::{post_customer_lottery} ;


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(post_customer_lottery);
}