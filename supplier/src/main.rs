#[macro_use]
extern crate ini;


extern crate actix_web;


use actix_web::{web::Data, App, HttpServer}; //middleware
mod filehandling;
mod definitions;
use  filehandling::{AppState,load_all_files,get_supplier};

// use jemallocator::Jemalloc;

// #[global_allocator]
// static GLOBAL: Jemalloc = Jemalloc;
use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

  #[actix_web::main]
  async fn main() -> std::io::Result<()> {
    println!("Server Starting"); 

    let app_ini = ini!(r"app_sup.ini");
    let workthread: i16 = match app_ini["section1"]["workerthread"].as_ref() 
    {
        Some(value) => value.clone().parse::<i16>().unwrap(),
        None =>  panic!("Error in reading ini file"),
    };

    let filedata = match load_all_files()
    {
      Ok(x) => x,
      Err(e) => panic!("Error: {}", e),
    };
    let supplier_files  =  Data::new(AppState {
      filecontent: filedata
  });

  

    match workthread {
      -1  =>{
        println!("Server Started with default worker thread");
        HttpServer::new(move || {
          App::new()
           // .wrap(middleware::Compress::default())
            .app_data(supplier_files.clone())
            .route("/api/supplier", actix_web::web::get().to(get_supplier))
        })       
        .bind(("0.0.0.0", 8080))?
        .run()
        .await

      },
      n => {
        println!("Server Started with {} worker thread", n);
        HttpServer::new(move || {
          App::new()
           // .wrap(middleware::Compress::default())
            .app_data(supplier_files.clone())
            .route("/api/supplier", actix_web::web::get().to(get_supplier))
        })
        .worker_max_blocking_threads(n as usize)
        .bind(("0.0.0.0", 8080))?
        .run()
        .await

      },
     



    }





   
  }


  
 
    



