use std::fs::File;
use std::io::Write;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
struct Output {
    url: String,
    status: String,
    duration: u128
}

pub fn format_to_json(
    url: String, 
    status: String, 
    duration: u128
) {
    let output = json!({
        "url": url,
        "status": status,
        "duration": duration,
    });
    
    let mut file = File::create("output.json")
        .expect("Failed to create new output file");
    let _ = file.write(output.to_string().as_bytes());
}
