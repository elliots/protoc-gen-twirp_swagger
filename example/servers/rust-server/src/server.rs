#![allow(unused_extern_crates)]
extern crate serde_ignored;
extern crate iron;
extern crate router;
extern crate bodyparser;
extern crate urlencoded;
extern crate uuid;
extern crate chrono;


use futures::Future;
use futures::future;
use futures::{stream, Stream};
use hyper;
use hyper::header::{Headers, ContentType};
use self::iron::prelude::*;
use self::iron::{status, modifiers, BeforeMiddleware};
use self::iron::url::percent_encoding::percent_decode;
use self::router::Router;
use self::urlencoded::UrlEncodedQuery;
use mimetypes;


use serde_json;


#[allow(unused_imports)]
use std::collections::{HashMap, BTreeMap};
#[allow(unused_imports)]
use swagger;
use std::io::Error;

#[allow(unused_imports)]
use std::collections::BTreeSet;

pub use swagger::auth::Authorization;
use swagger::auth::{AuthData, Scopes};
use swagger::{ApiError, Context, XSpanId};

use {Api,
     MakeHatResponse
     };
#[allow(unused_imports)]
use models;

header! { (Warning, "Warning") => [String] }

/// Create a new router for `Api`
pub fn router<T>(api: T) -> Router where T: Api + Send + Sync + Clone + 'static {
    let mut router = Router::new();
    add_routes(&mut router, api);
    router
}

/// Add routes for `Api` to a provided router.
///
/// Note that these routes are added straight onto the router. This means that if the router
/// already has a route for an endpoint which clashes with those provided by this API, then the
/// old route will be lost.
///
/// It is generally a bad idea to add routes in this way to an existing router, which may have
/// routes on it for other APIs. Distinct APIs should be behind distinct paths to encourage
/// separation of interfaces, which this function does not enforce. APIs should not overlap.
///
/// Alternative approaches include:
///
/// - generate an `iron::middleware::Handler` (usually a `router::Router` or
///   `iron::middleware::chain`) for each interface, and add those handlers inside an existing
///   router, mounted at different paths - so the interfaces are separated by path
/// - use a different instance of `iron::Iron` for each interface - so the interfaces are
///   separated by the address/port they listen on
///
/// This function exists to allow legacy code, which doesn't separate its APIs properly, to make
/// use of this crate.
#[deprecated(note="APIs should not overlap - only for use in legacy code.")]
pub fn route<T>(router: &mut Router, api: T) where T: Api + Send + Sync + Clone + 'static {
    add_routes(router, api)
}

/// Add routes for `Api` to a provided router
fn add_routes<T>(router: &mut Router, api: T) where T: Api + Send + Sync + Clone + 'static {

    let api_clone = api.clone();
    router.post(
        "/twirp/twitch.twirp.example.Haberdasher/MakeHat",
        move |req: &mut Request| {
            let mut context = Context::default();

            // Helper function to provide a code block to use `?` in (to be replaced by the `catch` block when it exists).
            fn handle_request<T>(req: &mut Request, api: &T, context: &mut Context) -> Result<Response, Response> where T: Api {

                context.x_span_id = Some(req.headers.get::<XSpanId>().map(XSpanId::to_string).unwrap_or_else(|| self::uuid::Uuid::new_v4().to_string()));
                context.auth_data = req.extensions.remove::<AuthData>();
                context.authorization = req.extensions.remove::<Authorization>();




                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.

                let param_body = req.get::<bodyparser::Raw>().map_err(|e| Response::with((status::BadRequest, format!("Couldn't parse body parameter body - not valid UTF-8: {}", e))))?;

                let mut unused_elements = Vec::new();

                let param_body = if let Some(param_body_raw) = param_body { 
                    let deserializer = &mut serde_json::Deserializer::from_str(&param_body_raw);

                    let param_body: Option<models::ExampleSize> = serde_ignored::deserialize(deserializer, |path| {
                            warn!("Ignoring unknown field in body: {}", path);
                            unused_elements.push(path.to_string());
                        }).map_err(|e| Response::with((status::BadRequest, format!("Couldn't parse body parameter body - doesn't match schema: {}", e))))?;

                    param_body
                } else {
                    None
                };
                let param_body = param_body.ok_or_else(|| Response::with((status::BadRequest, "Missing required body parameter body".to_string())))?;


                match api.make_hat(param_body, context).wait() {
                    Ok(rsp) => match rsp {
                        MakeHatResponse::(body) => {

                            let body_string = serde_json::to_string(&body).expect("impossible to fail to serialize");

                            let mut response = Response::with((status::Status::from_u16(200), body_string));    
                            response.headers.set(ContentType(mimetypes::responses::MAKE_HAT_.clone()));

                            context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                            if !unused_elements.is_empty() {
                                response.headers.set(Warning(format!("Ignoring unknown fields in body: {:?}", unused_elements)));
                            }
                            Ok(response)
                        },
                    },
                    Err(_) => {
                        // Application code returned an error. This should not happen, as the implementation should
                        // return a valid response.
                        Err(Response::with((status::InternalServerError, "An internal error occurred".to_string())))
                    }
                }
            }

            handle_request(req, &api_clone, &mut context).or_else(|mut response| {
                context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                Ok(response)
            })
        },
        "MakeHat");

}

/// Middleware to extract authentication data from request
pub struct ExtractAuthData;

impl BeforeMiddleware for ExtractAuthData {
    fn before(&self, req: &mut Request) -> IronResult<()> {

        Ok(())
    }
}
