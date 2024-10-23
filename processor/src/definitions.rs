// print_results(&seen, &counter);
use serde_derive::Serialize;
use serde_derive::Deserialize;
use thiserror::Error;


#[derive(Debug, Deserialize, Serialize)]
pub struct Hotels {
    #[serde(rename = "Hotel")]
    pub hotels: Vec<Hotel>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Hotel {
    #[serde(rename = "Id")]
    id: u32,
    #[serde(rename = "Name")]
    name: CData,
    #[serde(rename = "Rating")]
    rating: u8,
    #[serde(rename = "ThumbImages")]
    thumb_images: CData,
    #[serde(rename = "Price")]
    price: f64,
    #[serde(rename = "Hotelwiseroomcount")]
    hotelwiseroomcount: u32,
    #[serde(rename = "RoomDetails")]
    roomdetails: RoomDetails
}

#[derive(Debug, Deserialize,  Serialize)]
pub struct  RoomDetails {
    #[serde(rename = "RoomDetail")]
    roometail: Vec<RoomDetail>        
}

#[derive(Debug, Deserialize,  Serialize)]
pub struct RoomDetail {

    #[serde(rename = "Type")]
    typeval: String,

    #[serde(rename = "BookingKey")]
    bookingkey: String,

    #[serde(rename = "Adults")]
    adults: u32,

    #[serde(rename = "Children")]
    children: u32,

    #[serde(rename = "TotalRate")]
    totalrate: f64,

    #[serde(rename = "RoomDescription")]
    roomdescription: String,

    #[serde(rename = "TermsAndConditions")]
    termsandconditions: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct CData {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Error, Debug)]
pub enum ProcessorCallError {
    #[error("error `{0}`")]
    Error(String),

    // #[error("Error Reading file")]
    // FileRedaError,

    // #[error("Error Opening file")]
    // FileOpenError, 

    // #[error("Error Reading xml")]
    // XmlReadError,

    // #[error("Error writing xml")]
    // XmlWriteError, 
    
}
