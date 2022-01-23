use std::fmt;
use std::ops::RangeBounds;
type Data = bytes::BytesMut;
use crate::moya_error::MoyaError;
use http::Response as httpReps;
use serde::ser;



pub struct Response {
    status_code: u16,
    data: Data,
    request: Option<http::request::Request<()>>,
    response: Option<http::response::Response<()>>
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

    // fn serialize<T>(req: httpReps<T>) -> serde_json::Result<httpReps<Vec<u8>>>
    //     where T: ser::Serialize,
    // {
    //     let (parts, body) = req.into_parts();
    //     let body = serde_json::to_vec(&body)?;
    //     Ok(Response::from_parts(parts, body))
    // }

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

    pub fn filter_successfully_status_codes(&self) -> &Response {
        self.filter(200..299)
    }
    pub fn filter_successfully_and_redirect_codes(&self) -> &Response {
        self.filter(200..399)
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Status Code: {}, Data Length:: {}", self.status_code, self.data.len())
    }
}

impl PartialEq for Response {
    fn eq(&self, other: &Response) -> bool {
        self.status_code == other.status_code
            && self.data == other.data
            // && self.response == other.response
    }
}