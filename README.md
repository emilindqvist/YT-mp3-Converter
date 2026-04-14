# YT-mp3-Converter

Small Python CLI that downloads a YouTube video or Short and converts it to a
128 kbps MP3 — handy for building a Discord soundboard library, grabbing a
lecture as audio, or any case where you just want the sound.

## Features

- Downloads the best available audio via [`yt-dlp`](https://github.com/yt-dlp/yt-dlp)
- Transcodes to MP3 128 kbps via [`FFmpeg`](https://ffmpeg.org/)
- Works with regular videos *and* Shorts
- Live download progress + conversion status
- Skips re-downloading if the MP3 already exists
- Clean error messages for invalid/removed/private videos
- Saves to your **Downloads folder** by default — override with a flag or env var

## Requirements

- Python 3.9+
- `ffmpeg` on `PATH`
- `yt-dlp` Python package

## Installation

```bash
git clone https://github.com/emilindqvist/YT-mp3-Converter.git
cd YT-mp3-Converter

# create a virtual env (recommended)
python3 -m venv .venv
source .venv/bin/activate     # Windows: .venv\Scripts\activate

pip install -r requirements.txt
```

Install FFmpeg if you don't already have it:

| OS                | Command                         |
| ----------------- | ------------------------------- |
| Ubuntu / Debian / WSL | `sudo apt install ffmpeg`   |
| macOS (Homebrew)  | `brew install ffmpeg`           |
| Windows           | `winget install ffmpeg`         |

## Usage

```bash
python convert.py "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
```

The MP3 lands in `~/Downloads` by default. The final absolute path is printed
when the download finishes.

### Where the file is saved (priority order)

1. `-o / --output-dir <path>` — CLI flag, wins over everything
2. `$YTMP3_DIR` — environment variable
3. `~/Downloads` — default

Examples:

```bash
# one-off override
python convert.py "<url>" -o /tmp

# pin a default in your shell config (e.g. ~/.bashrc or ~/.zshrc)
export YTMP3_DIR="$HOME/Music/yt"
```

### Handy shell shortcut (optional)

If you'd rather type `convert <url>` from any folder, drop this into your
`~/.bashrc` (adjust the paths to where you cloned the repo):

```bash
convert() {
  /path/to/YT-mp3-Converter/.venv/bin/python \
    /path/to/YT-mp3-Converter/convert.py "$@"
}
```

Then:

```bash
source ~/.bashrc
convert "https://youtu.be/dQw4w9WgXcQ"
```

> **Tip:** always wrap the URL in quotes. YouTube playlist URLs contain `&`,
> which your shell will otherwise interpret as "run in background".

## Troubleshooting

- **`ERROR: ffmpeg not found`** — install FFmpeg and make sure `ffmpeg -version`
  works from the terminal you're running the script in.
- **`Video unavailable` / age-restricted / private** — yt-dlp will say so in the
  error message. Nothing this script can fix.
- **Filename has weird characters** — that's intentional (`restrictfilenames`)
  so the file is safe on Windows, Linux and macOS.

## License

MIT
