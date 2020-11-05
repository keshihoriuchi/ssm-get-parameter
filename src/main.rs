#![warn(clippy::all)]

use getopts::Options;
use rusoto_core::Region;
use rusoto_ssm::{GetParameterRequest, GetParameterResult, Parameter, Ssm, SsmClient};
use std::env;
use std::str::FromStr;
use std::string::String;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("d", "decrypt", "with decryption");
    opts.optopt("r", "region", "AWS region", "REGION");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    let with_decryption = matches.opt_present("d");
    let region = match matches.opt_str("r") {
        Some(r) => Region::from_str(&r).unwrap(),
        None => Region::default(),
    };

    let key = matches.free[0].clone();

    let client = SsmClient::new(region);
    match client
        .get_parameter(GetParameterRequest {
            name: key,
            with_decryption: Some(with_decryption),
        })
        .await
    {
        Ok(GetParameterResult {
            parameter: Some(Parameter { value: Some(v), .. }),
            ..
        }) => println!("{}", v),
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("{:?}", e),
    }
}
