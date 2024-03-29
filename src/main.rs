use clap::Parser;
use gevulot_shim::{Task, TaskResult};
// use prover::shared_state::verify;
use serde::{Deserialize, Serialize};
use serde_json::json;
// use std::fs::write;
use std::time::SystemTime;
use std::{error::Error, fs, io, result::Result};
#[derive(Serialize, Deserialize)]
struct PolygonVerifierResult {
    config_file: String,
    is_success: bool,
    message: String,
    timestamp: u64,
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("polygon_verifier main()");
    gevulot_shim::run(run_task)
}

// The main function that executes the prover program.
fn run_task(task: Task) -> Result<TaskResult, Box<dyn Error>> {
    println!("polygon_verifier run_task()");

    // to synchronize argument parsing
    let mut args = vec!["dummy".to_string()];
    for a in task.args.clone() {
        args.push(a);
    }
    println!("polygon_verifier args: {:?}", &args);

    let (is_success, config_file) = polygon_verifier(&args)?;

    let message: String = match is_success {
        true => "Polygon verifier result: success".to_string(),
        false => "Polygon verifier result: fail".to_string(),
    };

    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    let polygon_result = PolygonVerifierResult {
        config_file,
        is_success,
        message,
        timestamp,
    };

    let jresult = json!(polygon_result).to_string();
    let data = jresult.as_bytes().to_vec();
    println!("polygon_verifier jresult: {:?}", &jresult);

    // let verifier_path = "polygon_verifier.json".to_string();
    // write(verifier_path.clone(), jresult).unwrap();
    // task.result(data, vec![verifier_path])
    task.result(data, vec![])
}

#[derive(Parser, Debug)]
#[clap(author = "Polygon Verifier", version, about, long_about = None)]
pub struct PolygonVerifierConfig {
    /// Required for offline_prover, legacy_prover, and verifier
    #[clap(short, long, value_parser, verbatim_doc_comment)]
    pub config_path: Option<String>,
}

// #[tokio::main]
fn polygon_verifier(args: &Vec<String>) -> Result<(bool, String), Box<dyn Error>> {
    println!("polygon_verifier");
    let arg_conf = PolygonVerifierConfig::parse_from(args);

    let config_path = arg_conf.config_path;

    let entries = fs::read_dir(".")
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();

    println!("file entries in root directory :: {:?}", entries);

    let entries = fs::read_dir("/workspace")
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();

    println!("file entries in /workspace :: {:?}", entries);

    if config_path.is_none() {
        return Err(String::from("no config file parameter").into());
    }

    let _jproof = std::fs::read_to_string(config_path.clone().unwrap())?;
    // let proofs: Proofs = serde_json::from_str(&jproof)?;
    // let result = verify(proofs);
    let result = true;

    Ok((result, config_path.unwrap()))
}
