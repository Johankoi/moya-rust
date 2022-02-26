use std::collections::HashMap;
use crate::task;
use crate::validation_type;
use reqwest::Method;

pub trait TargetType {
    fn base_url(&self) -> String;

    fn path(&self) -> String;

    fn method(&self) -> Method;

    fn sample_data(&self) -> Vec<u8> {
        Vec::new()
    }

    fn task(&self) -> task::Task;

    // The type of validation to perform on the request. Default is `None`.
    fn validation_type(&self) -> validation_type::ValidationType {
        validation_type::ValidationType::None
    }

    // The headers to be used in the request.
    fn headers(&self) -> HashMap<String, String>;
}