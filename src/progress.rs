use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

const PROGRESS_PREFIX: &str = "PROGRESS ";

pub struct Progress {
    pub pct: f64,
    pub speed: String,
    pub eta: String,
}

pub fn parse(line: &str) -> Option<Progress> {
    let rest = line.trim().strip_prefix(PROGRESS_PREFIX)?;
    let mut parts = rest.split_whitespace();
    let pct_str = parts.next()?;
    let speed = parts.next()?.to_string();
    let eta = parts.next()?.to_string();
    let pct = pct_str.trim_end_matches('%').parse::<f64>().ok()?;
    Some(Progress { pct, speed, eta })
}

pub fn make_bar(quiet: bool) -> ProgressBar {
    if quiet {
        return ProgressBar::hidden();
    }
    let bar = ProgressBar::new(1000);
    bar.set_style(
        ProgressStyle::with_template("{spinner:.green} [{bar:30.cyan/blue}] {percent:>3}% {msg}")
            .unwrap()
            .progress_chars("=> "),
    );
    bar.enable_steady_tick(Duration::from_millis(120));
    bar
}

pub fn apply(bar: &ProgressBar, p: &Progress) {
    let pos = (p.pct.clamp(0.0, 100.0) * 10.0) as u64;
    bar.set_position(pos);
    bar.set_message(format!("{} ETA {}", p.speed, p.eta));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_standard_line() {
        let p = parse("PROGRESS 42.3% 1.23MiB/s 00:15").unwrap();
        assert!((p.pct - 42.3).abs() < 1e-6);
        assert_eq!(p.speed, "1.23MiB/s");
        assert_eq!(p.eta, "00:15");
    }

    #[test]
    fn ignores_non_progress_lines() {
        assert!(parse("[download] Destination: foo.m4a").is_none());
        assert!(parse("").is_none());
        assert!(parse("PROGRESS").is_none());
    }

    #[test]
    fn tolerates_extra_whitespace() {
        let p = parse("  PROGRESS   10.0%  500KiB/s  01:00  ").unwrap();
        assert!((p.pct - 10.0).abs() < 1e-6);
    }
}
