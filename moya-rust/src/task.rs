use std::collections::HashMap;  
use url::{Url, Host, Position};
use serde::{Serialize,Serializer};
use serde_json::Value;
pub struct ParameterEncoding {
 
}

pub enum Task {
    RequestPlain,
    RequestData(bytes::Bytes),
    RequestJSONEncodable(String),
    RequestParameters {parameters: HashMap<String, ()>, encoding: ParameterEncoding},
    RequestCompositeData {body_data: bytes::Bytes, url_arameters: HashMap<String, ()>},
    RequestCompositeParameters {body_parameters: HashMap<String, ()>, body_encoding: ParameterEncoding, 
        url_arameters: HashMap<String, ()>},
    UploadFile(Url),
}

