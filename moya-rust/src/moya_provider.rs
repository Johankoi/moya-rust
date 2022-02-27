use std::collections::HashMap;
use super::moya_respones::MoyaResponse;
use super::moya_error::MoyaError;
use super::target_type::TargetType;
use super::cancellable::*;
use super::endpoint::Endpoint;
use std::result::Result;

type DispatchQueue = Vec<i16>;
type Progress = f32;
type Session = reqwest::Client;

/// Closure to be executed when a request has completed.
type Completion = dyn Fn(Result<MoyaResponse, MoyaError>);

type EndpointClosure<T> = dyn Fn(T) -> Endpoint;
type Request = http::request::Request<()>;
type RequestResultClosure = dyn Fn(Result<Request, MoyaError>);
type RequestClosure = dyn Fn(Endpoint, RequestResultClosure);


pub struct ProgressResponse {
    response: Option<MoyaResponse>,
    progress_object: Progress
}


pub trait MoyaProviderType {
    type Target;

    /// Designated request-making method. Returns a `Cancellable` token to cancel the request later.
    fn request(target: Self::Target, callback_queue: Option<DispatchQueue>, progress: Option<ProgressResponse>, completion: &Completion) -> CancellableWrapper;
}


pub struct MoyaProvider<T> {
    end_point_closure: Box<EndpointClosure<T>>,
    request_closure: Box<RequestClosure>,
    client: Session,
    plugins: Vec<i16>,
    track_in_flights: bool,
    in_flight_requests: HashMap<Endpoint,Box<Completion>>,
    callback_queue: Option<DispatchQueue>,
    lock: i16,
}
impl<T> MoyaProvider<T> where T: TargetType {
    fn new(
        end_point_closure: Box<EndpointClosure<T>>,
        request_closure: Box<RequestClosure>,
        client: Session,
        plugins: Vec<i16>,
        track_in_flights: bool,
        in_flight_requests: HashMap<Endpoint,Box<Completion>>,
        callback_queue: Option<DispatchQueue>,
        lock: i16)
        -> Self {
            MoyaProvider {
                end_point_closure,
                request_closure,
                client,
                plugins,
                track_in_flights,
                in_flight_requests,
                callback_queue,
                lock,
            }
        }
}

impl<T> MoyaProviderType for MoyaProvider<T> {
    type Target = T;
    fn request(target: Self::Target,
               callbackQueue: Option<DispatchQueue>,
               progress: Option<ProgressResponse>,
               completion: &Completion) -> CancellableWrapper {
        CancellableWrapper::new()
    }
}

