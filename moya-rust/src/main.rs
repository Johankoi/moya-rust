use crate::validation_type::ValidationType;
use std::ptr;
mod moya_error;
mod validation_type;
mod task;
mod endpoint;
mod target_type;
mod moya_provider;
mod cancellable;
mod moya_respones;
mod moya_provider_defaults;

use std::collections::HashMap;


fn main() {
    // let a = ValidationType::NotFound(15);
    // let p = std::ptr::addr_of!(a);
    //
    // println!("{:?}",p);
    // println!("{:?}",a.status_code());


}

