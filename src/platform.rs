use std::fmt;
use url::Url;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Platform {
    YouTube,
    Instagram,
    TikTok,
    Twitter,
    Unknown,
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Platform::YouTube => "YouTube",
            Platform::Instagram => "Instagram",
            Platform::TikTok => "TikTok",
            Platform::Twitter => "X/Twitter",
            Platform::Unknown => "Unknown",
        };
        f.write_str(s)
    }
}

pub fn detect(raw: &str) -> Platform {
    let Ok(u) = Url::parse(raw) else {
        return Platform::Unknown;
    };
    let host = u
        .host_str()
        .unwrap_or("")
        .trim_start_matches("www.")
        .to_ascii_lowercase();
    match host.as_str() {
        "youtube.com" | "m.youtube.com" | "music.youtube.com" | "youtu.be" => Platform::YouTube,
        "instagram.com" => Platform::Instagram,
        "tiktok.com" | "vm.tiktok.com" | "m.tiktok.com" => Platform::TikTok,
        "twitter.com" | "mobile.twitter.com" | "x.com" => Platform::Twitter,
        _ => Platform::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_youtube_variants() {
        assert_eq!(
            detect("https://www.youtube.com/watch?v=dQw4w9WgXcQ"),
            Platform::YouTube
        );
        assert_eq!(detect("https://youtu.be/dQw4w9WgXcQ"), Platform::YouTube);
        assert_eq!(
            detect("https://m.youtube.com/watch?v=abc"),
            Platform::YouTube
        );
        assert_eq!(
            detect("https://music.youtube.com/watch?v=abc"),
            Platform::YouTube
        );
    }

    #[test]
    fn detects_instagram() {
        assert_eq!(
            detect("https://www.instagram.com/reel/xyz/"),
            Platform::Instagram
        );
    }

    #[test]
    fn detects_tiktok_variants() {
        assert_eq!(
            detect("https://www.tiktok.com/@user/video/123"),
            Platform::TikTok
        );
        assert_eq!(detect("https://vm.tiktok.com/abc/"), Platform::TikTok);
    }

    #[test]
    fn detects_twitter_variants() {
        assert_eq!(
            detect("https://twitter.com/user/status/123"),
            Platform::Twitter
        );
        assert_eq!(detect("https://x.com/user/status/123"), Platform::Twitter);
        assert_eq!(
            detect("https://mobile.twitter.com/user/status/123"),
            Platform::Twitter
        );
    }

    #[test]
    fn unknown_for_other_hosts() {
        assert_eq!(detect("https://example.com/video"), Platform::Unknown);
    }

    #[test]
    fn unknown_for_unparseable() {
        assert_eq!(detect("not a url"), Platform::Unknown);
    }
}
