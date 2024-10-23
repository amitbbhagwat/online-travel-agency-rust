#[macro_use]
extern crate ini;
extern crate futures;


extern crate reqwest;
extern crate quick_xml;
extern crate serde;
extern crate uuid;
extern crate hex;
use actix_web::{web, HttpServer,App};//, middleware};

mod definitions;
 mod processing;
 mod common;
 use common::{load_config_data, AppState};
//  use futures::FutureExt;
use processing::get_accomodation_handler;

// use jemallocator::Jemalloc;

// #[global_allocator]
// static GLOBAL: Jemalloc = Jemalloc;

use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

 #[actix_web::main]
//#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
 async fn main() -> std::io::Result<()> {
 
   
            // let filedata = match load_file()
            // {
            //   Ok(x) => x,
            //   Err(e) => panic!("Error: {}", e),
            // };
         
            let appdata: AppState = match load_config_data() {
                Ok(appdata) => appdata,
                Err(err) => {
                    println!("{}", err);
                    return Ok(());
                }
            };

            println!("{:?}", appdata);
       
           let wthread = appdata.wthread;
       
            let confidata = web::Data::new(appdata);
            


            match wthread{
                -1 => {
                    println!("Server Started with default worker thread");
                    HttpServer::new(move || {
                        App::new()
                            //.wrap(middleware::Compress::default())
                            .app_data(confidata.clone())
                            .route("/api/get-accomodations/", web::get().to(get_accomodation_handler))
                    })                    //.workers(25)
                   
                    .bind(("0.0.0.0", 8090))?
                    .run()
                    .await
                },
                n=> {
                    println!("Server Started with {}  worker thread", n );
                    HttpServer::new(move || {
                        App::new()
                            //.wrap(middleware::Compress::default())
                            .app_data(confidata.clone())
                            .route("/api/get-accomodations/", web::get().to(get_accomodation_handler))
                    })
                    //.workers(25)
                    .worker_max_blocking_threads(n as usize)
                    .bind(("0.0.0.0", 8090))?
                    .run()
                    .await

                }

               
            }


        
}

// //#[actix_web::main]
// //#[tokio::main]
//  fn main() -> std::io::Result<()> {

//     tokio::runtime::Builder::new_multi_thread()
//       .event_interval(100)
//       .global_queue_interval(60)
//       // .worker_threads(20)
//        .thread_stack_size(6144)
//         //.enable_all()
//         .build()
//         .unwrap()
//         .block_on(async {

//             let filedata = match load_file()
//             {
//               Ok(x) => x,
//               Err(e) => panic!("Error: {}", e),
//             };
          
//             let appdata: AppState = match load_config_data(filedata[0].clone()) {
//                 Ok(appdata) => appdata,
//                 Err(err) => {
//                     println!("{}", err);
//                     return Ok(());
//                 }
//             };
        
            
        
//             let confidata = web::Data::new(appdata);
//             println!("Starting the server");
//             HttpServer::new(move || {
//                 App::new()
//                     //.wrap(middleware::Compress::default())
//                     .app_data(confidata.clone())
//                     .route("/api/get-accomodations/", web::get().to(get_accomodation_handler))
//             }).bind(("0.0.0.0", 8090))?
//             .run()
//             .await
        

//         }) 


    
   
// }















