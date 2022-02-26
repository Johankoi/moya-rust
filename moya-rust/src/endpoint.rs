use std::error::Error;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use url::{Url, Host, Position};
use crate::moya_error::MoyaError;
use super::task::Task;
use reqwest::RequestBuilder;
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
    http_header_fields: Option<HashMap<String, String>>,
}

impl Endpoint {
   pub fn new(
        url: String,
        sample_response_closure: fn() -> EndpointSampleResponse,
        method: http::Method,
        task: Task,
        http_header_fields: Option<HashMap<String, String>>,
    ) -> Self {
        Endpoint {
            url,
            sample_response_closure,
            method,
            task,
            http_header_fields,
        }
    }

    pub fn replacing(&self, task: Task) -> Endpoint {
        Endpoint::new(self.url.clone(),self.sample_response_closure.clone(),self.method.clone(),task,self.http_header_fields.clone())
    }

    fn adding(&mut self, new_http_header_fields: Option<HashMap<String, String>>)  {
        match new_http_header_fields {
            Some(headers) => {
                if !headers.is_empty() {
                    match self.http_header_fields.as_mut() {
                        Some(header_mut_ref) => {
                            for (k, v) in headers {
                                header_mut_ref.insert(k, v);
                            }
                        }
                        None => {}
                    }
                }
            },
            None => {}
        }
    }

    pub fn urlRequest(&self) -> reqwest::RequestBuilder {
        let url = match Url::parse(self.url.as_str()) {
            Ok(url)  => url,
            Err(e) => panic!("{}",MoyaError::RequestMapping(self.url.as_str())),
        };
        let client = reqwest::Client::new();
        let requestBuilder= client.request(self.method.clone(), url);

        match &self.task {
            Task::RequestPlain => {
                requestBuilder
            },
            Task::RequestData(data) => {
                requestBuilder.body(data.clone())
            },
            Task::RequestJSONEncodable(encodable) => {
                requestBuilder.json(encodable)
            }
            _ => requestBuilder
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