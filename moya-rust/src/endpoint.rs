use std::error::Error;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use url::{Url, Host, Position};
use crate::moya_error::MoyaError;
use std::ptr::null;
use super::task::Task;

type Data = bytes::BytesMut;


pub enum EndpointSampleResponse {
    NetworkResponse(u16, Data),
    Response(http::response::Response<()>, Data),
    NetworkError(Box<dyn Error>),
}

pub struct Endpoint {
    url: String,
    sample_response_closure: fn() -> EndpointSampleResponse,
    method: http::Method,
    task: Task,
    http_header_fields: HashMap<String, String>,
}

impl Endpoint {
   pub fn new(
        url: String,
        sample_response_closure: fn() -> EndpointSampleResponse,
        method: http::Method,
        task: Task,
        http_header_fields: HashMap<String, String>,
    ) -> Self {
        Endpoint {
            url,
            sample_response_closure,
            method,
            task,
            http_header_fields,
        }
    }

    // // #[inline]
    pub fn get_task(&self) -> &Task {
        &self.task
    }
    pub fn http_header_fields_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.http_header_fields
    }
    
    pub fn add(&mut self, http_header_fields: Option<HashMap<String, String>>) -> &HashMap<String, String> {
        match http_header_fields {
            Some(headers) => {
                if headers.is_empty() {
                    &self.http_header_fields
                } else {
                    let orign_header = self.http_header_fields_mut();
                    for (k, v) in headers {
                        orign_header.insert(k, v);
                    }
                    &self.http_header_fields
                }
            },
            None => &self.http_header_fields
        }
    }
    pub fn replacing(&self, task: crate::task::Task) -> Endpoint {
        Endpoint::new(self.url.clone(),self.sample_response_closure.clone(),self.method.clone(),task,self.http_header_fields.clone())
    }

    pub fn into_clent(&self) -> reqwest::Client {

         // let parse_r = Url::parse(self.url.as_str());
         let url = match Url::parse(self.url.as_str()) {
            Ok(url)  => url,
            Err(e) => panic!("{}",MoyaError::RequestMapping(self.url.as_str())),
        };

        let client = reqwest::Client::new();
        let requestBuilder = client.request(self.method.clone(), url);

        match &self.task {
            Task::RequestPlain => {
                client
            },
            Task::RequestData(byte_m) => {
                requestBuilder.body(byte_m.clone());
                client
            },
            Task::RequestJSONEncodable(json_str) => {
                requestBuilder.json(json_str);
                client
            }
            _ => client
        }
    }
}


impl Hash for Endpoint {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.url.hash(state);
        // self.into_clent().hash(state);
    }
}

impl PartialEq for Endpoint {
    fn eq(&self, other: &Endpoint) -> bool {
        let mut lhs_hasher = DefaultHasher::new();
        self.hash(&mut lhs_hasher);
        let mut rhs_hasher = DefaultHasher::new();
        other.hash(&mut lhs_hasher);
        lhs_hasher.finish() == lhs_hasher.finish()
    }
}