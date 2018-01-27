#![allow(unused_imports, unused_qualifications, unused_extern_crates)]
extern crate chrono;
extern crate uuid;


use serde::ser::Serializer;

use std::collections::HashMap;
use models;
use swagger;


/// A Hat is a piece of headwear made by a Haberdasher.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExampleHat {
    /// The size of a hat should always be in inches.
    #[serde(rename = "size")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub size: Option<i32>,

    /// The color of a hat will never be 'invisible', but other than that, anything is fair game.
    #[serde(rename = "color")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub color: Option<String>,

    /// The name of a hat is it's type. Like, 'bowler', or something.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

}

impl ExampleHat {
    pub fn new() -> ExampleHat {
        ExampleHat {
            size: None,
            color: None,
            name: None,
        }
    }
}

/// Size is passed when requesting a new hat to be made. It's always measured in inches.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExampleSize {
    #[serde(rename = "inches")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub inches: Option<i32>,

}

impl ExampleSize {
    pub fn new() -> ExampleSize {
        ExampleSize {
            inches: None,
        }
    }
}
