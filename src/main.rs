#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![cfg_attr(feature = "clippy", deny(warnings))]
#![feature(slice_patterns)]

#[macro_use]
extern crate failure;
extern crate fruently;
extern crate serde;
extern crate serde_json;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use failure::Error;
use fruently::fluent::Fluent;
use fruently::forwardable::JsonForwardable;
use serde_json::Value;
use structopt::StructOpt;

#[derive(Debug, Fail)]
enum FluentError {
    #[fail(display = "")] InnerFluentError { e: fruently::error::FluentError },
}

impl From<fruently::error::FluentError> for FluentError {
    fn from(e: fruently::error::FluentError) -> FluentError {
        FluentError::InnerFluentError { e: e }
    }
}

type Result<T> = std::result::Result<T, Error>;

#[derive(StructOpt, Debug)]
#[structopt(name = "fruent-test-rs", about = "Test logging using fruently")]
struct MainConfig {
    #[structopt(short = "a", long = "addr",
                default_value = "127.0.0.1:24224", help = "Fruentd hostname")]
    addr: String,

    #[structopt(short = "t", long = "tag", default_value = "app.rs",
                help = "Tag to use when sending to Fruentd")]
    tag: String,

    #[structopt(short = "v", long = "value",
                help = "JSON serializable string value to log")]
    value: String,
}

fn run() -> Result<()> {
    let config = MainConfig::from_args();

    let v: Value = serde_json::from_str(&config.value)?;
    let fruently = Fluent::new(config.addr.as_str(), config.tag.as_str());

    fruently.post(&v).map_err(|e| -> FluentError { e.into() })?;

    Ok(())
}

fn main() {
    match run() {
        Ok(()) => println!("Program completed!"),
        Err(e) => println!("ERROR: {}", e),
    }
}
