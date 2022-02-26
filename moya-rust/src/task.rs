use std::collections::HashMap;  
use url::{Url, Host, Position};
use erased_serde::{Serialize, Serializer};
use reqwest::blocking::Client;
use serde_json::Value;
use std::any::Any;


#[derive(Clone, PartialEq, Eq)]
pub struct ParameterEncoding {
 
}


pub enum Task {
    RequestPlain,
    RequestData(bytes::Bytes),
    RequestJSONEncodable(Box<dyn Serialize>),
    RequestParameters {parameters: HashMap<String, Box<dyn Any>>, encoding: ParameterEncoding},
    RequestCompositeData {body_data: bytes::Bytes, url_parameters: HashMap<String, Box<dyn Any>>},
    RequestCompositeParameters {body_parameters: HashMap<String, Box<dyn Any>>, body_encoding: ParameterEncoding,
        url_parameters: HashMap<String, Box<dyn Any>>},
    UploadFile(Url),
}

// impl Clone for Task {
//     fn clone(&self) -> Self {
//         match self {
//             RequestPlain => Task::RequestPlain,
//             Task::RequestData(bytes) => Task::RequestData(bytes.clone()),
//             Task::RequestJSONEncodable(x) => Task::RequestJSONEncodable(Box::new()),
//             Task::RequestParameters { .. } => {}
//             Task::RequestCompositeData { .. } => {}
//             Task::RequestCompositeParameters { .. } => {}
//             Task::UploadFile(_) => {}
//         }
//     }
// }