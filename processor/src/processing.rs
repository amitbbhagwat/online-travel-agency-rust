use std::sync::Arc;

use actix_web::{web, HttpResponse,Responder};
 use anyhow::Result;
// use std::time::Instant;
use applib::xmlutil::extract_nodes_from_xml;
use awc::Client;
use tokio::task::JoinSet;
use crate::common::{AppState};// , simulate_cpu_usage};
//use rand::{thread_rng, Rng};
//use fastrand;
// use async_std::sync::Arc;
// use async_std::sync::Mutex;
//use futures::future;
use crate::definitions::ProcessorCallError;
// use std::sync::atomic::{AtomicUsize, Ordering};

pub async fn get_accomodation_handler(data: web::Data<AppState>) -> impl Responder {

    match get_accomodations(data).await {
        Ok(result) => HttpResponse::Ok().body(result),
        Err(err) => {
            
            println!("{}", err);
            HttpResponse::InternalServerError().body("Error in processing request")
        } 
    }
 }

 async fn  get_accomodations(appdata: web::Data<AppState>) -> Result<String, ProcessorCallError> {
    //let now = Instant::now();  
   
    let supplier_list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];   
 

    //let mut rng = thread_rng();
    let  max_no_of_supplier: u64 = fastrand::u64(appdata.min_noofsuppliers_forrandomness..appdata.max_noofsuppliers_forrandomness);
    //rng.gen_range(appdata.min_noofsuppliers_forrandomness..appdata.max_noofsuppliers_forrandomness);

    let mut supplier_from_index = fastrand::usize(0..supplier_list.len());// gen_range);
   
    let mut suppliers: Vec<i32> = Vec::new();   
    
    for _i in 0..max_no_of_supplier {
        suppliers.push(supplier_list[supplier_from_index]);
        supplier_from_index = (supplier_from_index + 1) % supplier_list.len();
    }
  
    let mut set  = JoinSet::new();
    let sleep = appdata.retrysleep.clone();
    let retry_ct   = appdata.retrycount.clone();
   
    for  (i, supplier) in suppliers.iter().enumerate(){      
        let url = appdata.adaptorhosturl.clone();
        let client = appdata.client.clone();
        //  let resultc = Arc::clone(&result);
        //  let counter = Arc::clone(&counter);      
         
         let isupplierdata = supplier.clone();       
         set.spawn(async move {          
            let data = {
                let mut sleepval = sleep;
                let mut retry = retry_ct;
                let data;
                loop {
                    if retry == 0 {
                        return Err("Error in getting response from adaptor for supplier".to_string());
                    }
                    //TODO : check if by moving the retry logic inside, can we prevent client clone?
                    match  get_accommodation_by_supplier(isupplierdata, &url, client.clone()).await {
                        Ok(data1) => {
                            data = data1;
                            break;
                        },
                        Err(error) => {
                            println!("{}", error);
                        }
                    };
                    retry -= 1;
                    tokio::time::sleep(tokio::time::Duration::from_secs(sleepval)).await;
                    sleepval*=(retry_ct - retry + 1);
            };
            data
          };        

            let res = match extract_nodes_from_xml(data.to_string(), "Hotel".into()).await {
                Ok(res) => res,
                Err(error) => {
                    println!("{}", error);
                    return Err(error.to_string()); 
                },
              };   

              //get counter value
             
            let strval = match String::from_utf8(res.0)
            {
                Ok(strval) => strval,
                Err(error) => {
                                println!("{}", error);
                                return Err(error.to_string()); 
                }
            };
           
            Ok((strval, res.1, i))
             
         });  
     
    }
 
    let mut resdata = vec![String::new(); suppliers.len()];
    let mut opdata = String::new();
    let mut counter = 0 as usize;
    while let Some(r) = set.join_next().await {
        match r {
            Ok(Ok((r, c, i))) => {
                opdata.push_str(r.as_str());
                resdata[i] = r;
                counter += c;
            },
            Ok(Err(error)) => return Err(ProcessorCallError::Error(error.to_string())),
            Err(error) => return Err(ProcessorCallError::Error(error.to_string())),
        }
       
        //seen[idx] = true;
    }

    //let resdata = result.lock().await; 
    let capacity = opdata.len() + 350;
    let mut resultdata = String::with_capacity(capacity);
    resultdata.push_str("<HotelFindResponse time=\"0.21500015258789\" ipaddress=\"14.140.153.130\" count=\"");
    //resultdata.push_str(counter.load(Ordering::Relaxed).to_string().as_str());
    resultdata.push_str(counter.to_string().as_str());
    resultdata.push_str( "\">\r\n    <ArrivalDate>01/06/2024</ArrivalDate>\r\n    <DepartureDate>10/06/2024</DepartureDate>\r\n    <Currency>INR</Currency>\r\n    <GuestNationality>IN</GuestNationality>\r\n    <SearchSessionId>17168872488751716887248949665</SearchSessionId><Hotels>");
   
    resultdata.push_str(&opdata);
    resultdata.push_str("\r\n</Hotels>\r\n</HotelFindResponse>");
  
    let arc_appdata = Box::new(appdata);
    tokio::task::spawn_blocking( move || {
            simulate_cpu_usage_async(&resdata[0],
             arc_appdata.max_cpu_usage_in_milliseconds,arc_appdata.min_cpu_usage_in_milliseconds);
    }).await.unwrap();
    //TODO: use block_inpace
    // tokio::task::spawn_blocking( move || {
    //     simulate_cpu_usage_async( &arc_appdata.firstsupplierdata, arc_appdata.max_cpu_usage_in_milliseconds,arc_appdata.min_cpu_usage_in_milliseconds); 
    // }).await.unwrap();
   
   Ok(resultdata.to_string())
}
 //

 pub  fn simulate_cpu_usage_async(ref xml_document: &String,  max_cpu_usage_in_milliseconds:u64,  min_cpu_usage_in_milliseconds: u64) {  
   
    // let mut rng = thread_rng();
      
    let loopcounter = fastrand::u64(min_cpu_usage_in_milliseconds..max_cpu_usage_in_milliseconds);
    if xml_document.len() > 0 {
        for _i in 0..loopcounter {
            let _xml_hash = crate::common::create_hash(&xml_document);
        
        }
    }
 
    // let delayduration = fastrand::u64(min_cpu_usage_in_milliseconds..max_cpu_usage_in_milliseconds);
    //  let loop_till_time = std::time::Instant::now() +  std::time::Duration::from_millis(delayduration);
   
    //  //let mut   loop_counter = 0;
    //  if xml_document.len() > 0 {
    //      while std::time::Instant::now() < loop_till_time {          
    //         let _xml_hash = crate::common::create_hash(&xml_document);
    //         //loop_counter = loop_counter + 1;
    //         //if loop_counter % 10 == 0 {
    //             //tokio::time::sleep(tokio::time::Duration::from_millis(0)).await;
    //         //}
    //      }
    //  }

 }
 pub async fn get_accommodation_by_supplier(supplier_id: i32,hostname: &String, client: Arc<reqwest::Client>) -> Result<String, ProcessorCallError>  {
   //let url = "http://localhost:9000/adapter/supplier?supplierId=1";
   let url = format!("{0}{1}",hostname, supplier_id);
   //println!("URL: {}", url);
//    let body = match client.get(url).send().await {
//     Ok(response) => match response.status(){
//                         reqwest::StatusCode::OK => {
//                                                 match response.text().await {
//                                                     Ok(body) => body,
//                                                     Err(err) => {

//                                                         let s = format!("Error in getting response from adaptor for supplier {}", err.to_string());
//                                                         return Err(ProcessorCallError::Error(s))

//                                                     }
//                                                 }

//                             } ,
//                             reqwest::StatusCode::UNAUTHORIZED => {
//                                 let s = format!("Need to grab a new token");
//                                 return Err(ProcessorCallError::Error(s))
//                             },
//                             _ => {
//                                 let s ="Error in getting response from adaptor for supplier {}".to_string();
//                                 return Err(ProcessorCallError::Error(s))
//                             }
//     },
//     Err(err) => {
//         let s = format!("Error in getting response from adaptor for supplier {}", err.to_string());
//         return Err(ProcessorCallError::Error(s))
//     }
// };   
     //let body = match reqwest::get(url).await {
        let body = match client.get(url).send().await {
            Ok(response) => {
                if response.status() != 200 {
                    let s = format!("Error in getting response from adaptor for supplier {}", response.status());
                    return Err(ProcessorCallError::Error(s))
                }
                match response.text().await {
                    Ok(body) => body,
                    Err(err) => {
                        let s = format!("Error in getting response from adaptor for supplier {}", err.to_string());
                        return Err(ProcessorCallError::Error(s))
                    },
                }
            },
            Err(err) => {
                let s = format!("Error in getting response from adaptor for supplier {}", err.to_string());
                return Err(ProcessorCallError::Error(s))
            }
        };   
    
  
   Ok(body.to_string())
 

}


