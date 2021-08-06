use std::fmt;
use std::ops::RangeBounds;
type Data = bytes::BytesMut;
use crate::moya_error::MoyaError;

pub struct Response {
    status_code: u16,
    data: Data,
    request: Option<http::request::Request<()>>,
    response: Option<http::response::Response<()>>
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Status Code: {}, Data Length:: {}", self.status_code, self.data.len())
    }
}

impl PartialEq for Response {
    fn eq(&self, other: &Response) -> bool {
        if self.status_code == other.status_code {
            return true
        } else if self.data == other.data {
            return true
        } else {
            return false
        }
    }
}

impl Response {
    pub fn new(
        status_code: u16, 
        data: Data, 
        request: Option<http::request::Request<()>>,
        response: Option<http::response::Response<()>>) 
        -> Self {
            Response {
                status_code,
                data,
                request,
                response,
            }
    }

    pub fn filter<R>(&self, status_codes: R) -> &Response
    where R: RangeBounds<u16> 
    {
        if !status_codes.contains(&self.status_code) {
            panic!("{}",MoyaError::StatusCode(self))
        }  
        self
    }

    pub fn filter_status_code(&self, status_code: u16) -> &Response {
        self.filter(status_code..status_code)
    }

    pub fn filter_successful_status_codes(&self) -> &Response {
        self.filter(200..299)
    }
    pub fn filter_successful_and_redirect_codes(&self) -> &Response {
        self.filter(200..399)
    }
}

