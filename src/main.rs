#![feature(slice_patterns)]

#[macro_use] extern crate failure;
extern crate fruently;

use failure::Error;
use fruently::fluent::Fluent;
use fruently::forwardable::JsonForwardable;
use std::collections::HashMap;
use std::env;

#[derive(Debug, Fail)]
enum FluentError {
    #[fail(display = "")]
    InnerFluentError {
        e: fruently::error::FluentError,
    },
}

impl From<fruently::error::FluentError> for FluentError {
    fn from(e: fruently::error::FluentError) -> FluentError {
        FluentError::InnerFluentError{ e: e }
    }
}

type Result<T> = std::result::Result<T, Error>;

fn run() -> Result<()> {
    let args: Vec<_> = env::args().collect();

    match *args.as_slice() {
        [_, ref addr, ref key, ref value] => {
            let mut obj: HashMap<String, String> = HashMap::new();
            obj.insert(key.to_owned(), value.to_owned());
            let fruently = Fluent::new(addr, "test-rs");

            fruently.post(&obj).map_err(|e| -> FluentError { e.into() })?;
            Ok(())
        },

        _ => {
            bail!("Usage: program <address (e.g. 127.0.0.1:24224)> <key> <value>");
        },
    }
}

fn main() {
    match run() {
        Ok(()) => println!("Program completed!"),
        Err(e) => println!("ERROR: {}", e),
    }
}
