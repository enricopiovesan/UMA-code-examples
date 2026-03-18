use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

#[derive(Deserialize)]
pub struct Input { pub id: String, pub bytes: Vec<u8> }

#[derive(Serialize)]
pub struct Output { pub id: String, pub tags: Vec<String> }

pub fn analyze(input: Input) -> Output {
    let sum: u64 = input.bytes.iter().map(|b| *b as u64).sum();
    let tags = if sum % 2 == 0 { vec!["even".to_string(), "low-entropy".to_string()] }
               else { vec!["odd".to_string(), "low-entropy".to_string()] };
    Output { id: input.id, tags }
}

// WASI entry via stdin/stdout so we can run with wasmtime
fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let input: Input = serde_json::from_str(&buf).unwrap();
    let out = analyze(input);
    let json = serde_json::to_string(&out).unwrap();
    std::io::stdout().write_all(json.as_bytes()).unwrap();
}
