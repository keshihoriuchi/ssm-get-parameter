#![warn(clippy::all)]

use getopts::Options;
use rusoto_core::Region;
use rusoto_ssm::{
    GetParameterRequest, GetParameterResult, GetParametersRequest, GetParametersResult, Parameter,
    Ssm, SsmClient,
};
use std::env;
use std::process;
use std::str::FromStr;
use std::string::String;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("d", "decrypt", "with decryption");
    opts.optflag("s", "get-parameters", "Using get-parameter*s*");
    opts.optopt("r", "region", "AWS region", "REGION");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!("{}", f.to_string()),
    };
    let with_decryption = matches.opt_present("d");
    let region = match matches.opt_str("r") {
        Some(r) => Region::from_str(&r).unwrap(),
        None => Region::default(),
    };
    let use_get_parameters = matches.opt_present("s");

    let key = matches.free[0].clone();

    let client = SsmClient::new(region);
    if use_get_parameters {
        match client
            .get_parameters(GetParametersRequest {
                names: vec![key],
                with_decryption: Some(with_decryption),
            })
            .await
        {
            Ok(GetParametersResult {
                parameters: Some(vs),
                ..
            }) => {
                if vs.len() != 1 {
                    eprintln!("length of parameters vector is {}", vs.len());
                    process::exit(1);
                }
                match &vs[0] {
                    Parameter { value: Some(v), .. } => {
                        println!("{}", v)
                    }
                    v => {
                        eprintln!("{:?}", v);
                        process::exit(1);
                    }
                }
            }
            Ok(v) => {
                eprintln!("{:?}", v);
                process::exit(1);
            }
            Err(e) => {
                eprintln!("{:?}", e);
                process::exit(1);
            }
        }
    } else {
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
            Ok(v) => {
                eprintln!("{:?}", v);
                process::exit(1);
            }
            Err(e) => {
                eprintln!("{:?}", e);
                process::exit(1);
            }
        }
    }
}
