use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError{
    pub error: String 
}
