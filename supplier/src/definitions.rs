use thiserror::Error;

#[derive(Error, Debug)]
pub enum SupplierCallError {
    // #[error("error")]
    // Error(String),

    #[error("Error Reading file")]
    FileRedaError,

    #[error("Error Opening file")]
    FileOpenError, 
    
}


