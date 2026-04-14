#!/usr/bin/env python3
# Requires: `pip install yt-dlp` and `ffmpeg` on PATH.
import argparse
import os
import sys
from pathlib import Path

from yt_dlp import YoutubeDL
from yt_dlp.utils import DownloadError


def resolve_output_dir(cli_arg: str | None) -> Path:
    if cli_arg:
        return Path(cli_arg).expanduser()
    env = os.environ.get("YTMP3_DIR")
    if env:
        return Path(env).expanduser()
    win_downloads = Path("/mnt/c/Users/Emil/Downloads")
    if win_downloads.exists():
        return win_downloads
    return Path.home() / "Downloads"


def progress_hook(d):
    status = d.get("status")
    if status == "downloading":
        pct = d.get("_percent_str", "").strip() or "?"
        speed = d.get("_speed_str", "").strip()
        eta = d.get("_eta_str", "").strip()
        sys.stdout.write(f"\rDownloading {pct}  {speed}  ETA {eta}   ")
        sys.stdout.flush()
    elif status == "finished":
        sys.stdout.write("\rDownload complete. Converting to MP3...      \n")
        sys.stdout.flush()


def main():
    parser = argparse.ArgumentParser(description="Download a YouTube video/short as a 128 kbps MP3.")
    parser.add_argument("url", help="YouTube video or Shorts URL")
    parser.add_argument("-o", "--output-dir", help="Directory to save the MP3 (overrides $YTMP3_DIR and ~/Downloads default)")
    args = parser.parse_args()

    out_dir = resolve_output_dir(args.output_dir)
    out_dir.mkdir(parents=True, exist_ok=True)

    base_opts = {
        "format": "bestaudio/best",
        "outtmpl": str(out_dir / "%(title)s.%(ext)s"),
        "restrictfilenames": True,
        "quiet": True,
        "no_warnings": True,
        "noprogress": True,
    }

    try:
        with YoutubeDL(base_opts) as ydl:
            info = ydl.extract_info(args.url, download=False)
            # prepare_filename honours restrictfilenames; swap ext for the post-processed output.
            src_path = ydl.prepare_filename(info)
            mp3_path = os.path.abspath(os.path.splitext(src_path)[0] + ".mp3")

        if os.path.exists(mp3_path):
            print(f"Already exists: {mp3_path}")
            return 0

        dl_opts = {
            **base_opts,
            "progress_hooks": [progress_hook],
            "postprocessors": [
                {"key": "FFmpegExtractAudio", "preferredcodec": "mp3", "preferredquality": "128"}
            ],
        }
        with YoutubeDL(dl_opts) as ydl:
            ydl.download([args.url])

        print(f"Saved: {mp3_path}")
        return 0

    except DownloadError as e:
        print(f"Error: {e}", file=sys.stderr)
        return 1
    except KeyboardInterrupt:
        print("\nCancelled.", file=sys.stderr)
        return 130
    except Exception as e:
        print(f"Unexpected error: {e}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    sys.exit(main())
