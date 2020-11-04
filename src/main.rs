#![warn(clippy::all)]

use rusoto_core::Region;
use rusoto_ssm::{GetParameterRequest, GetParameterResult, Parameter, Ssm, SsmClient};
use std::string::String;

#[tokio::main]
async fn main() {
    let client = SsmClient::new(Region::default());
    match client
        .get_parameter(GetParameterRequest {
            name: String::from("/moge/moga"),
            with_decryption: Some(true),
        })
        .await
    {
        Ok(GetParameterResult {
            parameter: Some(Parameter { value: Some(v), .. }),
            ..
        }) => println!("{:?}", v),
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("{:?}", e),
    }
}
