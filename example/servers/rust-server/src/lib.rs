#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate futures;
extern crate chrono;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

// Logically this should be in the client and server modules, but rust doesn't allow `macro_use` from a module.
#[cfg(any(feature = "client", feature = "server"))]
#[macro_use]
extern crate hyper;

extern crate swagger;

use futures::Stream;
use std::io::Error;

#[allow(unused_imports)]
use std::collections::HashMap;

pub use futures::Future;

#[cfg(any(feature = "client", feature = "server"))]
mod mimetypes;

pub use swagger::{ApiError, Context, ContextWrapper};


#[derive(Debug, PartialEq)]
pub enum MakeHatResponse {
    /// 
     ( models::ExampleHat ) ,
}


/// API
pub trait Api {

    /// MakeHat produces a hat of mysterious, randomly-selected color!
    fn make_hat(&self, body: models::ExampleSize, context: &Context) -> Box<Future<Item=MakeHatResponse, Error=ApiError> + Send>;

}

/// API without a `Context`
pub trait ApiNoContext {

    /// MakeHat produces a hat of mysterious, randomly-selected color!
    fn make_hat(&self, body: models::ExampleSize) -> Box<Future<Item=MakeHatResponse, Error=ApiError> + Send>;

}

/// Trait to extend an API to make it easy to bind it to a context.
pub trait ContextWrapperExt<'a> where Self: Sized {
    /// Binds this API to a context.
    fn with_context(self: &'a Self, context: Context) -> ContextWrapper<'a, Self>;
}

impl<'a, T: Api + Sized> ContextWrapperExt<'a> for T {
    fn with_context(self: &'a T, context: Context) -> ContextWrapper<'a, T> {
         ContextWrapper::<T>::new(self, context)
    }
}

impl<'a, T: Api> ApiNoContext for ContextWrapper<'a, T> {

    /// MakeHat produces a hat of mysterious, randomly-selected color!
    fn make_hat(&self, body: models::ExampleSize) -> Box<Future<Item=MakeHatResponse, Error=ApiError> + Send> {
        self.api().make_hat(body, &self.context())
    }

}

#[cfg(feature = "client")]
pub mod client;

// Re-export Client as a top-level name
#[cfg(feature = "client")]
pub use self::client::Client;

#[cfg(feature = "server")]
pub mod server;

// Re-export router() as a top-level name
#[cfg(feature = "server")]
pub use self::server::router;

pub mod models;
