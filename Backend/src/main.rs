use actix_web::{web , App, HttpServer, middleware, Result};
use env_logger::Env;
use actix_files::NamedFile;

mod controllers;
mod models;
mod config;
pub mod routes;
use crate::routes::*;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

   HttpServer::new(|| {
       App::new()
            .wrap(middleware::Logger::default())
            .configure(lottery_routes::config)
            .configure(basket_routes::config)
            .configure(prize_routes::config)
            .configure(customer_routes::config)
            .configure(admin_routes::config)
            .default_service(web::route().to(not_found)) // กำหนด handler 404 ด้วย middleware
   })

   .bind("127.0.0.1:3000")?
   .run()
   .await
}


async fn not_found() -> Result<NamedFile> {
    Ok(NamedFile::open("src/public/error404.html")?)
}

