use actix_web::web;
use crate::controllers::basket::* ;


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(get_basket)
        .service(post_basket)
        .service(delete_basket)
        .service(get_basket_verification);
}