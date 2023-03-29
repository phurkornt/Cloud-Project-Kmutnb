use actix_web::web;
use crate::controllers::admin::*;


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
    .service(admin_prize::get_admin_prize)
    .service(admin_prize::post_admin_prize)

    .service(admin_lottery::get_admin_lottery)
    .service(admin_lottery::post_admin_lottery)
    .service(admin_lottery::delete_admin_lottery)

    .service(admin_customer::get_admin_customer)
    .service(admin_customer::delete_admin_customer);

}