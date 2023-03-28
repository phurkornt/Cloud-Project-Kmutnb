use actix_web::web;
use crate::controllers::shipping_handler::get_shipping;


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_shipping);
}