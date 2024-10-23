
use anyhow::{Ok, Result};
use tokio::io::{self, AsyncReadExt};
use tokio::fs::File;

use quick_xml::events::{BytesStart, Event};
use quick_xml::reader::Reader;

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

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    
    #[tokio::test]
    async fn test_find_hotels() -> Result<()> {

        let xmlbuf = read_file("../xmls/sup_1_10001.xml").await?;
        let xml = String::from_utf8(xmlbuf)?;

        let hotels = read_text_in_node(xml, "Hotels".into()).await?;

        println!("Hotels: \n{}", hotels);
        Ok(())
    }

}

