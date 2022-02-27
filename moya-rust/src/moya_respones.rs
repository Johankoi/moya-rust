use std::fmt;
use std::string::String;
use std::ops::RangeBounds;
use serde::{ser, de};
use serde_json::{Value, Map};
use reqwest::{Request, Response};
use crate::moya_error::MoyaError;

pub struct MoyaResponse {
    status_code: u16,
    data: Vec<u8>,
    request: Option<Request>,
    response: Option<Response>
}

impl MoyaResponse {
    pub fn new(
        status_code: u16,
        data: Vec<u8>,
        request: Option<Request>,
        response: Option<Response>)
        -> Self {
        MoyaResponse {
                status_code,
                data,
                request,
                response,
            }
    }

    pub fn filter<R>(&self, status_codes: R) -> &MoyaResponse
        where R: RangeBounds<u16>
    {
        if !status_codes.contains(&self.status_code) {
            panic!("{}",MoyaError::StatusCode(self))
        }  
        self
    }

    pub fn filter_status_code(&self, status_code: u16) -> &MoyaResponse {
        self.filter(status_code..status_code)
    }

    pub fn filter_successfully_status_codes(&self) -> &MoyaResponse {
        self.filter(200..299)
    }
    pub fn filter_successfully_and_redirect_codes(&self) -> &MoyaResponse {
        self.filter(200..399)
    }

    pub fn mapJSON<T>(&self) -> Result<T, MoyaError>
        where for<'de> T: de::Deserialize<'de>
    {
        match serde_json::from_slice(&self.data) {
            Ok(r) => Ok(r),
            Err(error) => Err(MoyaError::JsonMapping(self))
        }
    }

    pub fn mapString(&self, atKeyPath: Option<&str>) -> Result<String, MoyaError> {
        let key = atKeyPath.ok_or(MoyaError::StringMapping(self))?;
        match self.mapJSON::<Value>() {
            Ok(value) => {
                let json_map = value.as_object().unwrap();
                match json_map.get(key).ok_or(MoyaError::StringMapping(self))?.as_str() {
                    Some(s) => Ok(s.to_string()),
                    None => Err(MoyaError::StringMapping(self))
                }
            }
            Err(e) => Err(MoyaError::StringMapping(self))
        }
    }
}

impl fmt::Display for MoyaResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Status Code: {}, Data Length:: {}", self.status_code, self.data.len())
    }
}

impl PartialEq for MoyaResponse {
    fn eq(&self, other: &MoyaResponse) -> bool {
        self.status_code == other.status_code
            && self.data == other.data
            // && self.response == other.response
    }
}