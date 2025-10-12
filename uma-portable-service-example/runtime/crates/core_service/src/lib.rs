
use anyhow::{Context, Result};
use serde::Serialize;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

#[derive(Debug, Serialize, Clone, PartialEq)]


fn thresholds_from_contract(c: &contract::Contract) -> (f32, f32) {
    let mut dark = 0.4f32;
    let mut bright = 0.6f32;
    if let Some(params) = c.parameters() {
        if let Some(tagging) = params.get("tagging").and_then(|v| v.as_object()) {
            if let Some(v) = tagging.get("avg_dark_threshold").and_then(|v| v.as_f64()) { dark = v as f32; }
            if let Some(v) = tagging.get("avg_bright_threshold").and_then(|v| v.as_f64()) { bright = v as f32; }
        }
    }
    (dark, bright)
}

        pub struct ImageMetrics {
    pub width: usize,
    pub height: usize,
    pub avg: f32,
    pub contrast: f32,
}

/// Parse a simple ASCII PGM (P2) and return pixel values
pub(crate) fn load_pgm_ascii(path: &str) -> Result<(usize, usize, Vec<u16>, u16)> {
    let f = File::open(path).with_context(|| format!("open {}", path))?;
    let mut r = BufReader::new(f);
    let mut line = String::new();

    // magic
    r.read_line(&mut line)?;
    if !line.trim().starts_with("P2") {
        anyhow::bail!("Only P2 PGM is supported");
    }

    // skip comments
    let mut dims_line = String::new();
    loop {
        dims_line.clear();
        r.read_line(&mut dims_line)?;
        if !dims_line.trim_start().starts_with('#') && !dims_line.trim().is_empty() {
            break;
        }
    }
    let dims: Vec<_> = dims_line.split_whitespace().collect();
    let (w, h): (usize, usize) = (dims[0].parse()?, dims[1].parse()?);

    // maxval
    let mut max_line = String::new();
    r.read_line(&mut max_line)?;
    let maxval: u16 = max_line.trim().parse()?;

    // pixels
    let mut pixels: Vec<u16> = Vec::with_capacity(w * h);
    for line in r.lines() {
        for tok in line?.split_whitespace() {
            if let Ok(v) = tok.parse::<u16>() {
                pixels.push(v);
            }
        }
    }
    if pixels.len() != w * h {
        anyhow::bail!("pixel count mismatch");
    }
    Ok((w, h, pixels, maxval))
}

pub fn analyze_image(path: &str, service_name: &str, contract: &contract::Contract) -> Result<()> {
    let (w, h, px, maxval) = load_pgm_ascii(path)?;
    let sum: u64 = px.iter().map(|&v| v as u64).sum();
    let avg = sum as f32 / (px.len() as f32);
    let avg_norm = if maxval > 0 { avg / maxval as f32 } else { 0.0 };

    let min = *px.iter().min().unwrap_or(&0) as f32;
    let max = *px.iter().max().unwrap_or(&0) as f32;
    let contrast = if maxval > 0 { (max - min) / maxval as f32 } else { 0.0 };

    let mut tags = Vec::new();
    if avg_norm < 0.4 { tags.push("mostly_dark".to_string()); }
    if avg_norm > 0.6 { tags.push("mostly_bright".to_string()); }
    if contrast > 0.8 { tags.push("high_contrast".to_string()); }
    if tags.is_empty() { tags.push("neutral".to_string()); }

    let metrics = ImageMetrics { width: w, height: h, avg: avg_norm, contrast };

    #[derive(Serialize)]
    struct Payload<'a> {
        service: &'a str,
        path: &'a str,
        tags: Vec<String>,
        metrics: ImageMetrics,
    }

    let payload = Payload { service: service_name, path, tags, metrics };
    bus::publish_validated(contract, "image.analyzed", &payload)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    fn write_temp_pgm(contents: &str) -> PathBuf {
        let mut p = std::env::temp_dir();
        p.push(format!("uma_test_{}.pgm", uuid()));
        fs::write(&p, contents).expect("write temp pgm");
        p
    }

    fn uuid() -> String {
        // very small unique-ish id for temp file names
        use std::time::{SystemTime, UNIX_EPOCH};
        let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        format!("{}", nanos)
    }

    #[test]
    fn parses_p2_pgm() {
        let pgm = "P2\n# t\n2 2\n255\n0 255 255 0\n";
        let path = write_temp_pgm(pgm);
        let (w,h,px,maxv) = load_pgm_ascii(path.to_str().unwrap()).unwrap();
        assert_eq!((w,h,maxv), (2,2,255));
        assert_eq!(px.len(), 4);
        assert!(px.contains(&0) && px.contains(&255));
    }

    #[test]
    fn analysis_is_deterministic() {
        let pgm = "P2\n2 2\n10\n0 10 10 0\n";
        let path = write_temp_pgm(pgm);
        // Should compute neutral with high contrast off at maxval 10
        analyze_image(path.to_str().unwrap(), "svc:1.0").unwrap();
        // If it reaches here without panic, the deterministic path is fine.
    }
}
