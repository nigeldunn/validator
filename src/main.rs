use jsonschema::JSONSchema;
use serde_json::json;
use clap::Parser;
use std::fs;

/// JSON Schema Validator
#[derive(Parser,Debug)]
#[clap(author="Nigel Dunn <nigel.dunn@gmail.com>", version="0.1.0", about="JSON Schema Validation CLI", long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    schema: String,

    #[clap(short, long, value_parser)]
    instance: String,
}

fn main() {
    let args = Args::parse();

    let schema_file = fs::read_to_string(&args.schema).unwrap();
    let schema: serde_json::Value = serde_json::from_str(&schema_file).expect("Schema JSON was not well-formatted");

    let instance_file = fs::read_to_string(&args.instance).unwrap();
    let instance: serde_json::Value = serde_json::from_str(&instance_file).expect("Instance JSON was not well-formatted");

    let compiled = JSONSchema::compile(&schema).expect("A valid schema");
    let result = compiled.validate(&instance);
    if let Err(errors) = result {
        for error in errors {
            println!("Validation error: {}", error);
        }
    }
}