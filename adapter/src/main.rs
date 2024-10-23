#[macro_use]
extern crate ini;

extern crate reqwest;
extern crate rand;
extern crate actix_web;
mod common;
use actix_web::{web, App, HttpServer}; //middleware

use common::{load_config_data,adapterhandler, AppState};

// use jemallocator::Jemalloc;

// #[global_allocator]
// static GLOBAL: Jemalloc = Jemalloc;

use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[actix_web::main]
  async fn main() -> std::io::Result<()> {
    println!("Server Starting"); 

    let appdata: AppState = match load_config_data() {
        Ok(appdata) => appdata,
        Err(err) => {
            println!("{}", err);
            return Ok(());
        }
    };

    
    let wthread = appdata.wthread;
    let confidata = web::Data::new(appdata);

    match wthread {
      -1 => {
        println!("Server Started with default worker thread");
        HttpServer::new(move || {
          App::new()
           // .wrap(middleware::Compress::default())
            .app_data(confidata.clone())
            .route("/api/adapter/supplier", actix_web::web::get().to(adapterhandler))
        })
       
        .bind(("0.0.0.0", 9000))?
        .run()
        .await

      }
      n => {
        println!("Server Started with {} worker thread", n); 
        HttpServer::new(move || {
          App::new()
           // .wrap(middleware::Compress::default())
            .app_data(confidata.clone())
            .route("/api/adapter/supplier", actix_web::web::get().to(adapterhandler))
        })
        .worker_max_blocking_threads(n as usize)
        .bind(("0.0.0.0", 9000))?
        .run()
        .await

      },
     
    }

  
   
  }
