#![allow(missing_docs, unused_variables, trivial_casts)]

extern crate swagger_client;
#[allow(unused_extern_crates)]
extern crate futures;
#[allow(unused_extern_crates)]
extern crate swagger;
#[allow(unused_extern_crates)]
extern crate uuid;
extern crate clap;

#[allow(unused_imports)]
use futures::{Future, future, Stream, stream};
#[allow(unused_imports)]
use swagger_client::{ApiNoContext, ContextWrapperExt,
                      ApiError,
                      MakeHatResponse
                     };
use clap::{App, Arg};

fn main() {
    let matches = App::new("client")
        .arg(Arg::with_name("operation")
            .help("Sets the operation to run")
            .possible_values(&[
])
            .required(true)
            .index(1))
        .arg(Arg::with_name("https")
            .long("https")
            .help("Whether to use HTTPS or not"))
        .arg(Arg::with_name("host")
            .long("host")
            .takes_value(true)
            .default_value("localhost")
            .help("Hostname to contact"))
        .arg(Arg::with_name("port")
            .long("port")
            .takes_value(true)
            .default_value("8080")
            .help("Port to contact"))
        .get_matches();

    let is_https = matches.is_present("https");
    let base_url = format!("{}://{}:{}",
                           if is_https { "https" } else { "http" },
                           matches.value_of("host").unwrap(),
                           matches.value_of("port").unwrap());
    let client = if is_https {
        // Using Simple HTTPS
        swagger_client::Client::try_new_https(&base_url, "examples/ca.pem")
            .expect("Failed to create HTTPS client")
    } else {
        // Using HTTP
        swagger_client::Client::try_new_http(&base_url)
            .expect("Failed to create HTTP client")
    };

    // Using a non-default `Context` is not required; this is just an example!
    let client = client.with_context(swagger_client::Context::new_with_span_id(self::uuid::Uuid::new_v4().to_string()));

    match matches.value_of("operation") {

        // Disabled because there's no example.
        // Some("MakeHat") => {
        //     let result = client.make_hat(???).wait();
        //     println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
        //  },

        _ => {
            panic!("Invalid operation provided")
        }
    }
}

