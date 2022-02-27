use std::fmt;
use std::error::Error;
use crate::moya_respones::MoyaResponse;


pub enum MoyaError<'a> {
    ImageMapping(&'a MoyaResponse),
    JsonMapping(&'a MoyaResponse),
    StringMapping(&'a MoyaResponse),
    ObjectMapping(Box<dyn Error>, &'a MoyaResponse),
    EncodableMapping(Box<dyn Error>),
    StatusCode(&'a MoyaResponse),
    Underlying(Box<dyn Error>, Option<&'a MoyaResponse>),
    RequestMapping(&'a str),
    ParameterEncoding(Box<dyn Error>),
}

impl<'a> MoyaError<'a> {
    pub fn response(&self) -> Option<&MoyaResponse> {
        match *self {
            MoyaError::ImageMapping(resp) => Some(resp),  
            MoyaError::JsonMapping(resp) => Some(resp), 
            MoyaError::StringMapping(resp) => Some(resp),
            MoyaError::ObjectMapping(_, resp) => Some(resp), 
            MoyaError::EncodableMapping(_) => None, 
            MoyaError::StatusCode(resp) => Some(resp), 
            MoyaError::Underlying(_, resp) => Some(resp.unwrap()),
            MoyaError::RequestMapping(_) => None, 
            MoyaError::ParameterEncoding(_) => None, 
        }
    }

    pub fn underlying_error(&self) -> Option<&Box<dyn Error>> {
        match self { 
            MoyaError::ImageMapping(_) => None, 
            MoyaError::JsonMapping(_) => None, 
            MoyaError::StringMapping(_) => None,     
            MoyaError::ObjectMapping(e, _) => Some(e),
            MoyaError::EncodableMapping(e) => Some(e),
            MoyaError::StatusCode(_) => None, 
            MoyaError::Underlying(e, _) => Some(e),
            MoyaError::RequestMapping(_) => None, 
            MoyaError::ParameterEncoding(e) => Some(e),
        }
    }
    pub fn moya_error_description(&self) -> &str {
        match self {
            MoyaError::ImageMapping(_) => "Failed to map data to an Image.", 
            MoyaError::JsonMapping(_) => "Failed to map data to JSON.", 
            MoyaError::StringMapping(_) => "Failed to map data to a String.",     
            MoyaError::ObjectMapping(_, _) => "Failed to map data to a Decodable object.",
            MoyaError::EncodableMapping(_) => "Failed to encode Encodable object into data.",
            MoyaError::StatusCode(_) => "Status code didn't fall within the given range.", 
            MoyaError::Underlying(e, _) => e.description(),
            MoyaError::RequestMapping(_) => "Failed to map Endpoint to a URLRequest.", 
            MoyaError::ParameterEncoding(e) => e.description(),
        }
    }
}

impl fmt::Display for MoyaError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}", self.moya_error_description());
    }
}