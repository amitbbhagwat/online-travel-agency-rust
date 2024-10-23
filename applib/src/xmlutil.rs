
use anyhow::{Result};
// use futures::future::ok;
use tokio::io::AsyncReadExt;
use tokio::fs::File;

use quick_xml::events::{BytesStart, Event, BytesEnd};
use quick_xml::reader::Reader;
use quick_xml::writer::Writer;
use std::io::Cursor;
use std::io::Write;

pub async fn read_file(path: &str) -> Result<Vec<u8>> {
    let mut f = File::open(path).await?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).await?;    
    Ok(buffer)
}

pub async fn read_text_in_node(xml: String, node: String) -> Result<String> {

    let res =
    tokio::task::spawn_blocking(move|| -> Result<String>{
        let mut reader = Reader::from_str(&xml);
        reader.config_mut().trim_text(true);

        let start = BytesStart::new(node);
        let end   = start.to_end().into_owned();   
        
        reader.read_event()?;
        reader.config_mut().check_end_names = false;
        
        let text = reader.read_text(end.name())?;
        reader.config_mut().check_end_names = true;
        Ok(text.into_owned())        
    }).await?;        

    res  

}

pub async fn extract_nodes_from_xml(xml: String, node: String) -> Result<(Vec<u8>, usize)> {

    let res =
    tokio::task::spawn_blocking(move || -> Result<(Vec<u8>, usize)>{

        let start = BytesStart::new(node.clone());
        let end   = start.to_end().into_owned(); 

        let mut count = 0;
        let mut reader = Reader::from_str(&xml);
        reader.config_mut().trim_text(true);
        let mut writer = Writer::new(Cursor::new(Vec::new()));
        
        reader.config_mut().check_end_names = false;
        loop {
            match reader.read_event() {
                Ok(Event::Start(e)) if e.name().as_ref() == node.as_bytes() => {
                    count += 1;
                    let mut elem = BytesStart::new(node.clone());
                    // collect existing attributes
                    elem.extend_attributes(e.attributes().map(|attr| attr.unwrap()));
                    
                    writer.write_event(Event::Start(elem))?;                    
                    let text = reader.read_text(end.name())?;
                    writer.get_mut().write_all(text.as_bytes())?;
                    writer.write_event(Event::End(BytesEnd::new(node.clone())))?;
                    
                },
                Ok(Event::Eof) => break,
                _ => (),
            }
        }

        let result = writer.into_inner().into_inner();

        Ok((result, count))

    }).await?;     
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use quick_xml::events::Event;
    use quick_xml::reader::Reader;
    
    #[tokio::test]
    async fn test_find_hotels() -> Result<()> {

        let xmlbuf = read_file("xmls/sup_1_10001.xml").await?;
        let xml = String::from_utf8(xmlbuf)?;

        let hotels = read_text_in_node(xml, "Hotels".into()).await?;

        println!("Hotels: \n{}", hotels);
        Ok(())
    }
    
  
    #[tokio::test]
    async fn test_find_and_count_hotels() -> Result<()> {

        let xml = r#"
        <Hotels>
            <Hotel>
                <Id>483291</Id>
                <Name>
                    <![CDATA[18 coins cafe and hostel]]>
                </Name>
                <Rating>2</Rating>
            </Hotel>
            <Hotel>
                <Id>693291</Id>
                <Name>
                    <![CDATA[The food]]>
                </Name>
                <Rating>4</Rating>
            </Hotel>
        </Hotels>
        "#;

        let node = "Hotel";
        
        let res = extract_nodes_from_xml(xml.into(), node.into()).await?;
       
        println!("\nText is: {}", String::from_utf8(res.0).unwrap());
       println!("\nHotel count {}", res.1);


        Ok(())
    }

}

