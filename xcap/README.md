# mcap-extractor
This is a tool to extract data from MCAP files.

## Features
- Extract raw H.264 data and frames from a MCAP file.
- Support sliced MCAP files.

## Usage
Extract raw H.264 data and frames from a MCAP file.
```bash
mcap-extractor -i /path/to/mcap/dir -o /path/to/output --topic="your_h264_topic"
```

## Build
Build the binary
```bash
cargo build --release
```

Build the deb package if you want to share it with others
```bash
cargo deb
```

## Installation
Download the latest release from the [releases page](https://github.com/yinguobing/mcap-extractor/releases).

Install using `dpkg`:
```bash
sudo dpkg -i mcap-extractor_<version>_amd64.deb
```
