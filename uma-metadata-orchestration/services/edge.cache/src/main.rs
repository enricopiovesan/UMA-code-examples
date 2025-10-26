use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

#[derive(Deserialize)]
pub struct ImageAnalyzed { pub id: String, pub tags: Vec<String> }

#[derive(Serialize)]
pub struct Status { pub source: String, pub event: String, pub status: String, pub reason: Option<String> }

fn persist(evt: &ImageAnalyzed) -> std::io::Result<()> {
    // simple deterministic KV file path for demo
    let path = format!("cache-{}.json", evt.id);
    std::fs::write(path, serde_json::to_string(evt).unwrap())
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let evt: ImageAnalyzed = serde_json::from_str(&buf).unwrap();
    let res = persist(&evt);
    let out = match res {
        Ok(_) => Status { source: "edge.cache".into(), event: "image.analyzed.v1".into(), status: "passed".into(), reason: None },
        Err(e) => Status { source: "edge.cache".into(), event: "image.analyzed.v1".into(), status: "failed".into(), reason: Some(e.to_string()) }
    };
    std::io::stdout().write_all(serde_json::to_string(&out).unwrap().as_bytes()).unwrap();
}
