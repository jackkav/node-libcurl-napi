#![deny(clippy::all)]

use napi_derive::napi;
extern crate curl;

use curl::easy::Easy;
use std::io::{stdout, Write};

#[napi]
pub fn fetcher(input: String) -> String {
  let res = fetch(input);
  match res {
    Ok(_) => "OK".to_string(),
    Err(_) => "Error".to_string(),
  }
}
pub fn fetch(input: String) -> Result<(), curl::Error> {
  let mut curl = Easy::new();

  curl.url(&input)?;
  curl.write_function(|data| {
    stdout().write_all(data).unwrap();
    Ok(data.len())
  })?;

  curl.perform()?;

  Ok(())
}

#[napi]
pub fn plus_100(input: u32) -> u32 {
  input + 100
}

#[napi]
pub fn plus_100_string(input: u32) -> String {
  format!("{}", input + 100)
}
