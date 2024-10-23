pub mod xmlutil;

use std::sync::{Arc};
use tokio::sync::Mutex;
use anyhow::{Ok, Result};
use reqwest;

use futures::future::join_all;
use crate::xmlutil::{read_text_in_node, extract_nodes_from_xml};
use std::sync::{atomic::{AtomicUsize, Ordering}};

async fn getxml(url: &str, supplier_id: &str) -> Result<String> {
    
    let response = reqwest::get(format!("{}/{}", url, supplier_id)).await?;    
    let body = response.text().await?;
    Ok(body)
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
                counter.fetch_add(data.1, Ordering::SeqCst);

                Ok(())
            })
        );
    }
    join_all(handles).await;        
    result.lock().await.push_str("</Hotels>");
    
    Ok((
        Arc::try_unwrap(result).unwrap().into_inner(), 
        counter.load(Ordering::SeqCst) 
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


#[cfg(test)]
mod tests {
    use mockito::ServerGuard;

    use super::*;
    use crate::xmlutil::read_file;
    use tokio::fs::File;
    use tokio::io::{AsyncWriteExt};

    #[tokio::test]
    async fn test_get_xml() {
        

        let (suppliers, xml_data) =
        get_supplier_data(vec![
            "xmls/sup_1_10001.xml",
            "xmls/sup_1_10002.xml",
            "xmls/sup_1_10003.xml",
            "xmls/sup_1_10004.xml"
        ]).await;

        let mut server = mockito::Server::new_async().await;
   

        let url = server.url();
        println!("url {}", url);

        let m0 = get_supplier_url_mock(&mut server, suppliers.get(0).unwrap(), xml_data.get(0).unwrap()).await;
        let m1 = get_supplier_url_mock(&mut server, suppliers.get(1).unwrap(), xml_data.get(1).unwrap()).await;
        let _m2 = get_supplier_url_mock(&mut server, suppliers.get(2).unwrap(), xml_data.get(2).unwrap()).await;
        let _m3 = get_supplier_url_mock(&mut server, suppliers.get(3).unwrap(), xml_data.get(3).unwrap()).await;
        
        let result = get_hotel_accomodations_from_suppliers(suppliers, &url).await.unwrap();

        //println!("result {}", result);
        let mut file = File::create("out.xml").await.unwrap();

        file.write_all(result.as_bytes()).await.unwrap();
  
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
            let xml_buf = read_file(file).await.unwrap();
            suppliers.push(i.to_string());
            data.push(xml_buf)
        }
        (suppliers, data)
    }
}
