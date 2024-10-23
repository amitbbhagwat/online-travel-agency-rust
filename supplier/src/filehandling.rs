use crate::definitions::SupplierCallError;
use std::fs;
use std::io::Read;
use async_std::task;

//use std::time::{Instant,Duration};
use std::time::Duration;
use serde::Serialize;
use serde::Deserialize;
use actix_web::{web, HttpResponse, Responder};


#[derive(Serialize)]
// struct ApiResponse {
//     message: String,
// }
pub struct AppState {
    pub filecontent: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Info {
     supplier_id: i32,
}


pub async  fn get_supplier(info: web::Query<Info>, data: web::Data<AppState>) -> impl Responder {
  //let start = Instant::now();
  
 //let index = info.supplier_id;
  let filecontent = &data.filecontent;
 // println!("Supplier ID: {}", supplier_id);
 let max_sleep_simulation_time: u64 = 4;
  //match supplier_id.parse::<i32>() {
    //Ok(index) => {
        
            //write a code for getting random number from 1 to 5 without seed
            let random_number = max_sleep_simulation_time;//fastrand::u64(0..max_sleep_simulation_time) + 1;
            
            //rand::random::<u64>() % 4 + 1;
           // println!("Random number: {}", random_number);
            task::sleep(Duration::from_secs(random_number)).await;
            let uindex = fastrand::u64(0..(filecontent.len() as u64)) as usize;
            //let uindex = index as usize;
            match filecontent.get(uindex) {
                Some(content) =>   {
                    //println!("Time taken in milli seconds: {:?} for supplierid {}", start.elapsed().as_millis(), supplier_id);
                    HttpResponse::Ok().
                content_type("application/xml")
                .body(content.to_string()) 
                },
                None => {
                    println!("Supplier not found");    
                    HttpResponse::NotFound().
                content_type("application/text").
                body("Supplier not found".to_string())   
                }
            }
           
    }
    // Err(_) => {
        
    //         HttpResponse::NotFound().
    //         content_type("application/text").body("Supplier not found".to_string())           
    // }

//}
   
//}

pub fn load_all_files() -> Result<Vec<String>, SupplierCallError>{

    println!("loading all files");
    let mut filecontent: Vec<String> = Vec::new();
    let directory = r"xmls";
    if let Ok(entries) = fs::read_dir(directory) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() && path.extension().unwrap_or_default() == "xml" {
                    let mut file =  match fs::File::open(&path)
                    {
                        Ok(file) => file,
                        Err(e) => {
                            println!("Error opening file: {}", e);
                            return Err(SupplierCallError::FileOpenError);
                        }
                    };
                    let mut content = String::new();
                    match file.read_to_string(&mut content)
                    {
                        Ok(_) => {    
                           // println!("File content: {}", content);                     
                            filecontent.push(content);                                   
                        },
                        Err(e) =>  {
                            println!("Error reading file: {}", e);
                            return Err(SupplierCallError::FileRedaError);
                        },
                    }
                }
            }
        }

      
    }
    Ok(filecontent)
}

