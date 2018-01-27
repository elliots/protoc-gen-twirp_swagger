//! Server implementation of swagger_client.

#![allow(unused_imports)]

use futures::{self, Future};
use chrono;

use std::collections::HashMap;

use swagger;

use swagger_client::{Api, ApiError, Context,
                      MakeHatResponse
};
use swagger_client::models;

#[derive(Copy, Clone)]
pub struct Server;

impl Api for Server {

    /// MakeHat produces a hat of mysterious, randomly-selected color!
    fn make_hat(&self, body: models::ExampleSize, context: &Context) -> Box<Future<Item=MakeHatResponse, Error=ApiError> + Send> {
        let context = context.clone();
        println!("make_hat({:?}) - X-Span-ID: {:?}", body, context.x_span_id.unwrap_or(String::from("<none>")).clone());
        Box::new(futures::failed("Generic failure".into()))
    }

}
