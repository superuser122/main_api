use serde::{Serialize};

#[derive(Debug, Serialize)]
pub struct ApiResponse<T : Serialize>{
    pub status: ResponseStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors : Option<Vec<ApiError>>,
}

impl<T: Serialize> ApiResponse<T>{
    pub fn ok_with_body( body : T) -> Self{
        Self{
            status: ResponseStatus::Success,
            body: Some(body),
            errors: None,
        }

    }
    pub fn ok()-> Self {
        Self{
            status: ResponseStatus::Success,
            body: None,
            errors: None,
        }
    }

    pub fn error(errors :Vec<(String, String)>) -> Self{
        let errors : Vec<ApiError> = errors.into_iter().map( |(code,msg)| {
            ApiError{ code: code, message: msg}
        }).collect();
        Self{
            status: ResponseStatus::Error,
            body: None,
            errors: Some(errors)
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ApiError{
    pub code: String,
    pub message: String 
}

#[derive(Clone, Copy, Serialize, Debug)]
pub enum ResponseStatus{
    Success,
    Error,
}
