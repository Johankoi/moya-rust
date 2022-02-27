use std::collections::HashMap;  
use url::Url;
use erased_serde::{Serialize, Serializer};

pub enum Task {
    RequestPlain,
    RequestData(bytes::Bytes),
    RequestJSONEncodable(Box<dyn Serialize>),
    RequestParameters {parameters: HashMap<String, Box<dyn Serialize>>, encoding: Box<dyn Serializer>},
    RequestCompositeData {body_data: bytes::Bytes, url_parameters: HashMap<String, Box<dyn Serialize>>},
    RequestCompositeParameters {body_parameters: HashMap<String, Box<dyn Serialize>>, body_encoding: Box<dyn Serializer>,
        url_parameters: HashMap<String, Box<dyn Serialize>>},
    UploadFile(Url),
}

