
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use reqwest::Client;
use std::sync::Arc;
use applib::get_client;
// use thiserror::Error;

// #[derive(Error, Debug)]
// pub enum ProcessorCallError {
//     // #[error("error")]
//     // Error(String),

//     // #[error("Error Reading xml")]
//     // XmlReadError,

//     // #[error("Error writing xml")]
//     // XmlWriteError, 
    
// }

#[derive(Clone)]
pub struct AppState {
    pub supplierhosname: String,
    pub supplierport: String,
    pub wthread: i16,
    pub client: Arc<Client>,
 }

 #[derive(Debug, Deserialize)]
pub struct Info {
     supplier_id: String,
}

 pub fn load_config_data() -> Result<AppState,&'static str> {

    let app_ini = ini!(r"app_adpater.ini");
    let workthread: i16 = match app_ini["section1"]["workerthread"].as_ref() 
    {
        Some(value) => value.clone().parse::<i16>().unwrap(),
        None =>  return Err("minnoofsuppliers is not found in app.ini"),
    };
    let supplier_host_name  = match app_ini["section1"]["supplierhostname"].as_ref() 
    {
        Some(value) => value.clone(),
        None => return Err("minnoofsuppliers is not found in app.ini")
    };

    let supplier_port  = match app_ini["section1"]["supplierport"].as_ref() 
    {
        Some(value) => value.clone(),
        None => return Err("minnoofsuppliers is not found in app.ini")
    };

    Ok(AppState {
        supplierhosname: supplier_host_name,
        supplierport: supplier_port,
        wthread: workthread,
        client: Arc::new(get_client().unwrap()),
    })

 }


 pub async  fn adapterhandler(info: web::Query<Info>, data: web::Data<AppState>) -> impl Responder {

    let configdata = data.get_ref().clone();
    let _supplier_id = &info.supplier_id;
    let adapterurl = "http://".to_string() + &configdata.supplierhosname + ":" + 
    &configdata.supplierport + "/api/supplier?supplier_id=0";
  
    //match reqwest::get(adapterurl).await {
    match configdata.client.get(adapterurl).send().await {
        Ok(response) => match response.text().await {
            Ok(body) => HttpResponse::Ok().content_type("application/xml").body(body),
            Err(err) => {
                println!("Error in getting response from supplier {}", err.to_string());
                HttpResponse::InternalServerError().content_type("application/text").body(err.to_string())
            }
        },
        Err(err) => {
            println!("Error in getting response from supplier {}", err.to_string());
            HttpResponse::InternalServerError().content_type("application/text").body(err.to_string())
        }
    }  

    // match get_data(adapterurl).await {
    //     Ok(response) => HttpResponse::Ok().content_type("application/xml").body(response),
    //     Err(err) => HttpResponse::InternalServerError().content_type("application/text").body(err.to_string())
    // }
 }


//  pub async fn get_data(url: String) -> Result<String, ProcessorCallError>  {

//     let body = match reqwest::get(url).await {
//         Ok(response) => match response.text().await {
//             Ok(body) => body,
//             Err(_err) => return Err(ProcessorCallError::Error("Error in getting response from supplier".to_string()))
//         },
//         Err(_err) => return Err(ProcessorCallError::Error("Error in getting response from supplier".to_string()))
//     };   


 
//   Ok(body.to_string())
//  }