use actix_web::{App, HttpServer, middleware};
use env_logger::Env;

pub mod routes;

mod controllers;
mod models;
mod config;


use crate::routes::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

   HttpServer::new(|| {
       App::new()
            .wrap(middleware::Logger::default())
            .configure(cart_routes::config)
            .configure(shipping_routes::config)

            .configure(lottery_routes::config)
            .configure(basket_routes::config)
            .configure(prize_routes::config)
            .configure(customer_routes::config)
            
            .configure(admin_routes::config)

   })
   .bind("127.0.0.1:3000")?
   .run()
   .await
}
