use anyhow::{Context, Result};
use serde::Serialize;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn thresholds_from_contract(c: &contract::Contract) -> (f32, f32) {
    let mut dark = 0.4f32;
    let mut bright = 0.6f32;
    if let Some(params) = c.parameters() {
        if let Some(tagging) = params.get("tagging").and_then(|v| v.as_object()) {
            if let Some(v) = tagging.get("avg_dark_threshold").and_then(|v| v.as_f64()) {
                dark = v as f32;
            }
            if let Some(v) = tagging.get("avg_bright_threshold").and_then(|v| v.as_f64()) {
                bright = v as f32;
            }
        }
    }
    (dark, bright)
}

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct ImageMetrics {
    pub width: usize,
    pub height: usize,
    pub avg: f32,
    pub contrast: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AnalysisResult {
    pub tags: Vec<String>,
    pub metrics: ImageMetrics,
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

pub fn analyze_image_data(path: &str, contract: &contract::Contract) -> Result<AnalysisResult> {
    let (w, h, px, maxval) = load_pgm_ascii(path)?;
    let sum: u64 = px.iter().map(|&v| v as u64).sum();
    let avg = sum as f32 / (px.len() as f32);
    let avg_norm = if maxval > 0 { avg / maxval as f32 } else { 0.0 };

    let min = *px.iter().min().unwrap_or(&0) as f32;
    let max = *px.iter().max().unwrap_or(&0) as f32;
    let contrast = if maxval > 0 {
        (max - min) / maxval as f32
    } else {
        0.0
    };
    let (dark_threshold, bright_threshold) = thresholds_from_contract(contract);

    let mut tags = Vec::new();
    if avg_norm < dark_threshold {
        tags.push("mostly_dark".to_string());
    }
    if avg_norm > bright_threshold {
        tags.push("mostly_bright".to_string());
    }
    if contrast > 0.8 {
        tags.push("high_contrast".to_string());
    }
    if tags.is_empty() {
        tags.push("neutral".to_string());
    }

    let metrics = ImageMetrics {
        width: w,
        height: h,
        avg: avg_norm,
        contrast,
    };
    Ok(AnalysisResult { tags, metrics })
}

pub fn analyze_image(path: &str, service_name: &str, contract: &contract::Contract) -> Result<()> {
    let result = analyze_image_data(path, contract)?;

    #[derive(Serialize)]
    struct Payload<'a> {
        service: &'a str,
        path: &'a str,
        tags: Vec<String>,
        metrics: ImageMetrics,
    }

    let payload = Payload {
        service: service_name,
        path,
        tags: result.tags,
        metrics: result.metrics,
    };
    bus::publish_validated(contract, "image.analyzed", &payload)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};

    static TEMP_FILE_COUNTER: AtomicU64 = AtomicU64::new(0);

    fn write_temp_pgm(contents: &str) -> PathBuf {
        let mut p = std::env::temp_dir();
        p.push(format!("uma_test_{}.pgm", uuid()));
        fs::write(&p, contents).expect("write temp pgm");
        p
    }

    fn uuid() -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let counter = TEMP_FILE_COUNTER.fetch_add(1, Ordering::Relaxed);
        format!("{}-{}", nanos, counter)
    }

    #[test]
    fn parses_p2_pgm() {
        let pgm = "P2\n# t\n2 2\n255\n0 255 255 0\n";
        let path = write_temp_pgm(pgm);
        let (w, h, px, maxv) = load_pgm_ascii(path.to_str().unwrap()).unwrap();
        assert_eq!((w, h, maxv), (2, 2, 255));
        assert_eq!(px.len(), 4);
        assert!(px.contains(&0) && px.contains(&255));
    }

    #[test]
    fn analysis_is_deterministic() {
        let pgm = "P2\n2 2\n10\n0 10 10 0\n";
        let path = write_temp_pgm(pgm);
        let contract = contract::Contract::load_from("../../../CONTRACT.json").unwrap();
        let result = analyze_image_data(path.to_str().unwrap(), &contract).unwrap();
        assert_eq!(result.metrics.width, 2);
        assert_eq!(result.metrics.height, 2);
        assert_eq!(result.tags, vec!["high_contrast".to_string()]);
    }

    #[test]
    fn contract_thresholds_change_tags() {
        let pgm = "P2\n2 2\n10\n3 3 3 3\n";
        let path = write_temp_pgm(pgm);
        let mut contract = contract::Contract::load_from("../../../CONTRACT.json").unwrap();
        contract.parameters = serde_json::json!({
            "tagging": {
                "avg_dark_threshold": 0.35,
                "avg_bright_threshold": 0.9
            }
        });

        let result = analyze_image_data(path.to_str().unwrap(), &contract).unwrap();
        assert_eq!(result.tags, vec!["mostly_dark".to_string()]);
    }
}
