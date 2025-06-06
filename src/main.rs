use std::env;
use std::fs;
use std::path::Path;

use ir::IR;
use ts2ir::traits::IRFromTypeScript;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <typescript_file>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    
    // Validate file exists and has correct extension
    let path = Path::new(file_path);
    if !path.exists() {
        eprintln!("File not found: {}", file_path);
        std::process::exit(1);
    }

    if let Some(extension) = path.extension() {
        if extension != "ts" {
            println!("Warning: File doesn't have .ts extension");
        }
    }

    // Read file content
    let content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            std::process::exit(1);
        }
    };

    println!("Processing TypeScript file: {}", file_path);
    println!("Content:\n{}", content);

    // Convert TypeScript to IR
    let ir = IR::from_typescript(&content);

    // Print the IR structure
    println!("\nConverted to IR:");
    println!("{:#?}", ir);

    // Serialize to JSON
    match serde_json::to_string_pretty(&ir) {
        Ok(json) => {
            println!("\nJSON representation:");
            println!("{}", json);
        }
        Err(err) => {
            eprintln!("Error serializing to JSON: {}", err);
        }
    }
}
