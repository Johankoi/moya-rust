use std::borrow::{Borrow, BorrowMut};
use std::error::Error;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use url::Url;
use super::moya_error::MoyaError;
use super::task::Task;
use reqwest::{Method, RequestBuilder, Response};

pub enum EndpointSampleResponse {
    NetworkResponse(u16, bytes::BytesMut),
    Response(Response, bytes::BytesMut),
    NetworkError(Box<dyn Error>),
}

pub struct Endpoint  {
    pub(crate) url: String,
    pub(crate) sample_response_closure: Box<dyn Fn() -> EndpointSampleResponse>,
    pub(crate) method: Method,
    pub(crate) task: Task,
    pub(crate) http_header_fields: Option<HashMap<String, String>>,
}

impl Endpoint {
   pub fn new(
        url: String,
        sample_response_closure: Box<dyn Fn() -> EndpointSampleResponse>,
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

    // pub fn replacing(self, task: Task) -> Endpoint {
    //
    //     let ss= self.sample_response_closure;
    //     Endpoint::new(self.url.clone(),Box::new( &self.sample_response_closure),self.method.clone(),task,self.http_header_fields.clone())
    // }

    pub fn adding(&mut self, new_http_header_fields: Option<HashMap<String, String>>)  {
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
            },
            Task::RequestParameters {parameters, encoding} => {
                requestBuilder.form(parameters)
            },
            Task::RequestCompositeData {body_data, url_parameters} => {
                requestBuilder.query(url_parameters).body(body_data.clone())
            },
            Task::RequestCompositeParameters {body_parameters, body_encoding,
                url_parameters} => {
                requestBuilder.query(url_parameters).form(body_parameters)
            }
            _ => requestBuilder
        }
    }
}


impl Hash for Endpoint {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match &self.task {
            Task::UploadFile(url) => {
                url.hash(state);
            }
            _ => {}
        }
        self.url.hash(state);
        //self.urlRequest().hash(state);
    }
}

impl PartialEq for Endpoint {
    fn eq(&self, other: &Endpoint) -> bool {
        let areEndpointsEqualInAdditionalProperties =
        match (&self.task, &other.task) {
            (Task::UploadFile(file1), Task::UploadFile(file2)) => file1 == file2,
            _ => true
        };
        let mut lhs_hasher = DefaultHasher::new();
        self.hash(&mut lhs_hasher);
        let mut rhs_hasher = DefaultHasher::new();
        other.hash(&mut lhs_hasher);
        return areEndpointsEqualInAdditionalProperties &&  (lhs_hasher.finish() == lhs_hasher.finish());
    }
}