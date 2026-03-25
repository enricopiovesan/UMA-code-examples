use anyhow::{Context, Result};
use serde::Serialize;
use std::fs;

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
    let contents = fs::read_to_string(path).with_context(|| format!("open {}", path))?;
    let mut lines = contents.lines();

    // magic
    let line = lines.next().unwrap_or_default();
    if !line.trim().starts_with("P2") {
        anyhow::bail!("Only P2 PGM is supported");
    }

    // skip comments
    let dims_line = lines
        .find(|line| !line.trim_start().starts_with('#') && !line.trim().is_empty())
        .context("missing dimensions line")?;
    let dims: Vec<_> = dims_line.split_whitespace().collect();
    anyhow::ensure!(dims.len() == 2, "invalid dimensions line");
    let w: usize = dims[0].parse()?;
    let h: usize = dims[1].parse()?;

    // maxval
    let max_line = lines.next().context("missing max value line")?;
    let maxval: u16 = max_line.trim().parse()?;

    // pixels
    let mut pixels: Vec<u16> = Vec::with_capacity(w * h);
    for line in lines {
        for tok in line.split_whitespace() {
            match tok.parse::<u16>() {
                Ok(v) => pixels.push(v),
                Err(_) => continue,
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

    let payload = serde_json::json!({
        "service": service_name,
        "path": path,
        "tags": result.tags,
        "metrics": result.metrics,
    });
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

    #[test]
    fn thresholds_fall_back_without_tagging_config() {
        let contract = contract::Contract::load_from("../../../CONTRACT.json").unwrap();
        assert_eq!(thresholds_from_contract(&contract), (0.4, 0.6));

        let mut missing_tagging = contract.clone();
        missing_tagging.parameters = serde_json::json!({});
        assert_eq!(thresholds_from_contract(&missing_tagging), (0.4, 0.6));

        let mut null_params = contract.clone();
        null_params.parameters = serde_json::Value::Null;
        assert_eq!(thresholds_from_contract(&null_params), (0.4, 0.6));
    }

    #[test]
    fn thresholds_can_override_each_boundary_independently() {
        let mut contract = contract::Contract::load_from("../../../CONTRACT.json").unwrap();
        contract.parameters = serde_json::json!({
            "tagging": {
                "avg_dark_threshold": 0.2
            }
        });
        assert_eq!(thresholds_from_contract(&contract), (0.2, 0.6));

        contract.parameters = serde_json::json!({
            "tagging": {
                "avg_bright_threshold": 0.8
            }
        });
        assert_eq!(thresholds_from_contract(&contract), (0.4, 0.8));
    }

    #[test]
    fn rejects_non_p2_images() {
        let path = write_temp_pgm("P5\n2 2\n255\n0 255 255 0\n");
        let err = load_pgm_ascii(path.to_str().unwrap()).unwrap_err();
        assert!(err.to_string().contains("Only P2 PGM is supported"));
    }

    #[test]
    fn missing_file_is_reported_with_path_context() {
        let err = load_pgm_ascii("/definitely/missing/file.pgm").unwrap_err();
        assert!(err.to_string().contains("open /definitely/missing/file.pgm"));
    }

    #[test]
    fn rejects_pixel_count_mismatch() {
        let path = write_temp_pgm("P2\n2 2\n255\n0 255 255\n");
        let err = load_pgm_ascii(path.to_str().unwrap()).unwrap_err();
        assert!(err.to_string().contains("pixel count mismatch"));
    }

    #[test]
    fn rejects_invalid_dimensions_and_maxval() {
        let bad_dims = write_temp_pgm("P2\nx 2\n255\n0 1\n");
        let dims_err = load_pgm_ascii(bad_dims.to_str().unwrap()).unwrap_err();
        assert!(!dims_err.to_string().is_empty());

        let short_dims = write_temp_pgm("P2\n2\n255\n0 1\n");
        let short_dims_err = load_pgm_ascii(short_dims.to_str().unwrap()).unwrap_err();
        assert!(short_dims_err.to_string().contains("invalid dimensions line"));

        let bad_second_dim = write_temp_pgm("P2\n2 x\n255\n0 1\n");
        let second_dim_err = load_pgm_ascii(bad_second_dim.to_str().unwrap()).unwrap_err();
        assert!(!second_dim_err.to_string().is_empty());

        let bad_max = write_temp_pgm("P2\n1 1\nabc\n0\n");
        let max_err = load_pgm_ascii(bad_max.to_str().unwrap()).unwrap_err();
        assert!(!max_err.to_string().is_empty());
    }

    #[test]
    fn invalid_pixel_tokens_are_ignored_until_count_mismatch() {
        let path = write_temp_pgm("P2\n2 2\n255\n0 255 oops 0\n");
        let err = load_pgm_ascii(path.to_str().unwrap()).unwrap_err();
        assert!(err.to_string().contains("pixel count mismatch"));
    }

    #[test]
    fn rejects_missing_dimensions_and_max_lines() {
        let missing_dims = write_temp_pgm("P2\n# comment only\n");
        let dims_err = load_pgm_ascii(missing_dims.to_str().unwrap()).unwrap_err();
        assert!(dims_err.to_string().contains("missing dimensions line"));

        let missing_max = write_temp_pgm("P2\n2 2\n");
        let max_err = load_pgm_ascii(missing_max.to_str().unwrap()).unwrap_err();
        assert!(max_err.to_string().contains("missing max value line"));
    }

    #[test]
    fn zero_maxval_yields_zero_metrics_and_dark_tag() {
        let pgm = "P2\n2 2\n0\n0 0 0 0\n";
        let path = write_temp_pgm(pgm);
        let contract = contract::Contract::load_from("../../../CONTRACT.json").unwrap();
        let result = analyze_image_data(path.to_str().unwrap(), &contract).unwrap();
        assert_eq!(result.metrics.avg, 0.0);
        assert_eq!(result.metrics.contrast, 0.0);
        assert_eq!(result.tags, vec!["mostly_dark".to_string()]);
    }

    #[test]
    fn bright_images_are_tagged_as_bright() {
        let pgm = "P2\n2 2\n10\n9 9 9 9\n";
        let path = write_temp_pgm(pgm);
        let contract = contract::Contract::load_from("../../../CONTRACT.json").unwrap();
        let result = analyze_image_data(path.to_str().unwrap(), &contract).unwrap();
        assert_eq!(result.tags, vec!["mostly_bright".to_string()]);
    }

    #[test]
    fn analyze_image_data_propagates_missing_file_errors() {
        let contract = contract::Contract::load_from("../../../CONTRACT.json").unwrap();
        let err = analyze_image_data("/definitely/missing/file.pgm", &contract).unwrap_err();
        assert!(err.to_string().contains("open /definitely/missing/file.pgm"));
    }

    #[test]
    fn neutral_images_are_tagged_when_no_other_rule_matches() {
        let pgm = "P2\n2 2\n10\n5 5 5 5\n";
        let path = write_temp_pgm(pgm);
        let contract = contract::Contract::load_from("../../../CONTRACT.json").unwrap();
        let result = analyze_image_data(path.to_str().unwrap(), &contract).unwrap();
        assert_eq!(result.tags, vec!["neutral".to_string()]);
    }

    #[test]
    fn analyze_image_publishes_validated_event() {
        let pgm = "P2\n2 2\n10\n0 10 10 0\n";
        let path = write_temp_pgm(pgm);
        let contract = contract::Contract::load_from("../../../CONTRACT.json").unwrap();

        analyze_image(path.to_str().unwrap(), "core-service", &contract).unwrap();
    }

    #[test]
    fn analyze_image_fails_when_contract_event_is_missing() {
        let pgm = "P2\n2 2\n10\n0 10 10 0\n";
        let path = write_temp_pgm(pgm);
        let mut contract = contract::Contract::load_from("../../../CONTRACT.json").unwrap();
        contract.events.clear();

        let err = analyze_image(path.to_str().unwrap(), "core-service", &contract).unwrap_err();
        assert!(err.to_string().contains("schema not found"));
    }

    #[test]
    fn analyze_image_propagates_analysis_errors() {
        let contract = contract::Contract::load_from("../../../CONTRACT.json").unwrap();
        let err = analyze_image("/definitely/missing/file.pgm", "core-service", &contract).unwrap_err();
        assert!(err.to_string().contains("open /definitely/missing/file.pgm"));
    }
}
