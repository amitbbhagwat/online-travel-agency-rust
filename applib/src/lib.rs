pub mod xmlutil;

use std::sync::{Arc};
use tokio::sync::Mutex;
use anyhow::{Ok, Result};
use reqwest;
use std::time::Instant;
use futures::future::join_all;
use crate::xmlutil::{read_text_in_node, extract_nodes_from_xml};
use std::sync::{atomic::{AtomicUsize, Ordering}};


use reqwest::Client;
use reqwest::ClientBuilder;

async fn getxml(url: &str, supplier_id: &str) -> Result<String> {
    
    let response = reqwest::get(format!("{}/{}", url, supplier_id)).await?;    
    let body = response.text().await?;
    Ok(body)
}

pub async fn get_hotel_data_from_suppliers_serial(suppliers: Vec<String>, url: &str) -> Result<Vec<String>> {
    //declare string vector without mutex
    //let mut result = String::with_capacity(50000);
    let results =  Arc::new(Mutex::new(Vec::new()));
    //let counter = Arc::new(AtomicUsize::new(0));

    let mut handles: Vec<tokio::task::JoinHandle<Result<()>>> = vec![];

    //result.push_str("<Hotels>");
    
    for supplier in suppliers {
        let urlc = String::from(url);       
        let resultc = Arc::clone(&results);
        handles.push(
            tokio::spawn(async move {
                let xml_full = getxml(&urlc, &supplier).await?;
               // let data = extract_nodes_from_xml(xml_full, "Hotels".into()).await?;  
                let mut results = resultc.lock().await;     
                            
                results.push(xml_full);
             

                Ok(())
            })
        );
    }
    join_all(handles).await;        
    //result.push_str("</Hotels>");

    Ok(Arc::try_unwrap(results).unwrap().into_inner())

}

pub async fn get_hotel_data_from_suppliers(suppliers: Vec<String>, url: &str) -> Result<(String, usize)> {
    let result =  Arc::new(Mutex::new(String::with_capacity(50000)));
    let counter = Arc::new(AtomicUsize::new(0));

    let mut handles: Vec<tokio::task::JoinHandle<Result<()>>> = vec![];

    result.lock().await.push_str("<Hotels>");
    
    for supplier in suppliers {
        let urlc = String::from(url);
        let resultc = Arc::clone(&result);
        let counter = Arc::clone(&counter);
        handles.push(
            tokio::spawn(async move {
                let xml_full = getxml(&urlc, &supplier).await?;
                let data = extract_nodes_from_xml(xml_full, "Hotels".into()).await?;
                let mut result = resultc.lock().await;                
                result.push_str(&String::from_utf8(data.0)?);
                counter.fetch_add(data.1, Ordering::Relaxed);

                Ok(())
            })
        );
    }
    join_all(handles).await;        
    result.lock().await.push_str("</Hotels>");

    Ok((
        Arc::try_unwrap(result).unwrap().into_inner(), 
        counter.load(Ordering::Relaxed) 
    ))
}

pub async fn get_hotel_accomodations_from_suppliers(suppliers: Vec<String>, url: &str) -> Result<String> {
    let result =  Arc::new(Mutex::new(String::with_capacity(50000)));

    let mut handles: Vec<tokio::task::JoinHandle<Result<()>>> = vec![];

    result.lock().await.push_str("<Hotels>");
    
    for supplier in suppliers {
        let urlc = String::from(url);
        let resultc = Arc::clone(&result);
        handles.push(
            tokio::spawn(async move {
                let xml_full = getxml(&urlc, &supplier).await?;
                let xml = read_text_in_node(xml_full, "Hotels".into()).await?;
                let mut result = resultc.lock().await;
                result.push_str(&xml);

                Ok(())
            })
        );
    }
    join_all(handles).await;
    
    result.lock().await.push_str("</Hotels>");

    Ok(Arc::try_unwrap(result).unwrap().into_inner())
}

pub fn get_client() -> reqwest::Result<Client>{

    // let mut http_connector = HttpConnector::new();
    // http_connector.set_keepalive(Some(Duration::from_secs(90)));

    ClientBuilder::new()
        .pool_max_idle_per_host(100)
        .pool_idle_timeout(std::time::Duration::from_secs(90))
        .timeout(std::time::Duration::from_secs(60))
        .build()
}


pub fn create_hash(xml_document: &str) -> String {
    if xml_document.len() == 0 {
        return String::new();
    }
    let text = xml_document;
    let actual = ring::digest::digest(&ring::digest::SHA256, text.as_bytes());
    //format!("{}{}", xml_document, create_new_guid());
    // let mut hasher = Sha256::new();
    // hasher.update(text.as_bytes());
    // hex::encode(hasher.finalize())
    hex::encode(actual)
}

pub fn simulate_cpu(ref xml_document: &String,  max_cpu_usage_in_milliseconds:u64,  min_cpu_usage_in_milliseconds: u64) {  
   
    // let mut rng = thread_rng();
    // let delayduration = fastrand::u64(min_cpu_usage_in_milliseconds..max_cpu_usage_in_milliseconds);
    let loopcounter = fastrand::u64(min_cpu_usage_in_milliseconds..max_cpu_usage_in_milliseconds);
    if xml_document.len() > 0 {
        for _i in 0..loopcounter {
            let _xml_hash = create_hash(&xml_document);
        
        }
    }
}


#[cfg(test)]
mod tests {
    use mockito::ServerGuard;

    use super::*;
    use crate::xmlutil::read_file;
    use tokio::fs::File;
    use tokio::io::{AsyncWriteExt};

    #[tokio::test]
    async fn test_get_xml_serial() {

        let (suppliers, xml_data) =
        get_supplier_data(vec![
            "xmls/sup_1_10001.xml",
            "xmls/sup_2_10001.xml",
            "xmls/sup_3_10001.xml",
            "xmls/sup_4_10001.xml"
        ]).await;

        let mut server = mockito::Server::new_async().await;
   

        let url = server.url();
        println!("url {}", url);

        let m0 = get_supplier_url_mock(&mut server, suppliers.get(0).unwrap(), xml_data.get(0).unwrap()).await;
        let m1 = get_supplier_url_mock(&mut server, suppliers.get(1).unwrap(), xml_data.get(1).unwrap()).await;
        let _m2 = get_supplier_url_mock(&mut server, suppliers.get(2).unwrap(), xml_data.get(2).unwrap()).await;
        let _m3 = get_supplier_url_mock(&mut server, suppliers.get(3).unwrap(), xml_data.get(3).unwrap()).await;
        
        //start time
        let start_time = Instant::now();
        let mut result = Vec::new();
        result.push("<Hotels>".to_string());
        //let result = get_hotel_accomodations_from_suppliers(suppliers, &url).await.unwrap();
        let data = get_hotel_data_from_suppliers_serial(suppliers, &url).await.unwrap();

        for xml in data {
            let res = match extract_nodes_from_xml(xml.to_string(), "Hotel".into()).await {
                core::result::Result::Ok(res) => res,
                Err(error) => {
                    println!("{}", error);
                    return; 
                },
              }; 
              let strval = match String::from_utf8(res.0)
              {
                  core::result::Result::Ok(strval) => strval,
                  Err(error) => {
                                  println!("{}", error);
                                  return; 
                  }
              };
              result.push(strval);

        }
        result.push("</Hotels>".to_string());
        //end time
        let end_time = Instant::now();
        let elapsed_time = end_time.duration_since(start_time).as_millis();
        println!("Elapsed time for serial: {} ms", elapsed_time);
        //println!("result {}", result);
        let mut file = File::create("out.xml").await.unwrap();
    
        for data in result {
            file.write_all(data.as_bytes()).await.unwrap();
        }
     
  
        m0.assert_async().await;
        m1.assert_async().await;
    }
    #[tokio::test]
    async fn test_get_xml() {
        

        let (suppliers, xml_data) =
        get_supplier_data(vec![
                      "xmls/sup_1_10001.xml",
                        "xmls/sup_2_10001.xml",
                        "xmls/sup_3_10001.xml",
                        "xmls/sup_4_10001.xml"
        ]).await;

        let mut server = mockito::Server::new_async().await;
   

        let url = server.url();
        println!("url {}", url);

        let m0 = get_supplier_url_mock(&mut server, suppliers.get(0).unwrap(), xml_data.get(0).unwrap()).await;
        let m1 = get_supplier_url_mock(&mut server, suppliers.get(1).unwrap(), xml_data.get(1).unwrap()).await;
        let _m2 = get_supplier_url_mock(&mut server, suppliers.get(2).unwrap(), xml_data.get(2).unwrap()).await;
        let _m3 = get_supplier_url_mock(&mut server, suppliers.get(3).unwrap(), xml_data.get(3).unwrap()).await;
        
        //start time
        let start_time = Instant::now();
        //let result = get_hotel_accomodations_from_suppliers(suppliers, &url).await.unwrap();
        let result = get_hotel_data_from_suppliers(suppliers, &url).await.unwrap();
        //end time
         //end time
         let end_time = Instant::now();
         let elapsed_time = end_time.duration_since(start_time).as_millis();
         println!("Elapsed time for non serial: {} ms", elapsed_time);
        //println!("result {}", result);
        let mut file = File::create("out.xml").await.unwrap();

        file.write_all(result.0.as_bytes()).await.unwrap();
  
        m0.assert_async().await;
        m1.assert_async().await;

    }

    async fn get_supplier_url_mock(server: &mut ServerGuard, supplier: &str, buf: &[u8]) -> mockito::Mock {
        let supplierpath = format!("/{}", supplier);
        let _m = server.mock("GET", supplierpath.as_str())
        .with_status(200)        
        .with_body(buf)
        .create_async().await;
        _m
    }

    async fn get_supplier_data(files: Vec<&str>) -> (Vec<String>, Vec<Vec<u8>>){

        let mut suppliers = Vec::new();
        let mut data = Vec::new();

        for (i, file) in files.iter().enumerate() {
           // println!("file {}", file);
            let xml_buf = read_file(file).await.unwrap();
            suppliers.push(i.to_string());
            data.push(xml_buf)
        }
        (suppliers, data)
    }
}
