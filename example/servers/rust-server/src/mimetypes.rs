/// mime types for requests and responses

pub mod responses {
    use hyper::mime::*;

    // The macro is called per-operation to beat the recursion limit
    /// Create Mime objects for the response content types for MakeHat
    lazy_static! {
        pub static ref MAKE_HAT_: Mime = mime!(Application/Json);
    }

}

pub mod requests {
    use hyper::mime::*;
   /// Create Mime objects for the request content types for MakeHat
    lazy_static! {
        pub static ref MAKE_HAT: Mime = mime!(Application/Json);
    }

}
