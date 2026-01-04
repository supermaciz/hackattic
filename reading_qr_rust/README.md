# reading_qr_rust

Small Rust GTK app for the Hackattic “Reading QR” challenge.

Fetches a QR code image from the Hackattic API, decodes it locally, and submits the decoded text back.

## Requirements

- Rust toolchain (stable)
- GTK4 + libadwaita development headers installed (for the GUI build)
- `HACKATTIC_TOKEN` environment variable set to your Hackattic access token (runtime)

## Setup

Install system deps (examples):

- Debian/Ubuntu:

```bash
sudo apt install libgtk-4-dev libadwaita-1-dev
```

- Fedora:

```bash
sudo dnf install gtk4-devel libadwaita-devel
```

## Build

```bash
cargo build
```

## Run

Set the token and start the app:

```bash
export HACKATTIC_TOKEN=your_token_here
cargo run
```

The window opens with the QR image. Click **Decode** to read it, then **Submit solution** to send it back.

## Screenshots

Initial QR prompt:

![Hackattic Reading QR prompt](Screenshot1.png)

Decoded + submitted successfully:

![Hackattic Reading QR result](Screenshot3.png)

## Notes

- Expects the challenge payload directly from Hackattic; no local assets required.
- Network access is required to fetch the QR and submit the result.
