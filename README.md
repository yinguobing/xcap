# mcap-extractor
Single binary to extract ROS messages from MCAP files.

## Features
- Messages: CompressedImage, PointCloud2
- Support sliced MCAP files.
- Support MinIO as input source.

## Usage
Extract from a local directory containing multiple MCAP files
```bash
mcap-extractor -i /path/to/mcap/dir -o /path/to/output --topics="/h264,/lidar"
```

Extract from a MinIO bucket:
```bash
export S3_ACCESS_KEY="YOUR_KEY"
export S3_SECRET_KEY="YOUR_SECRET"
export S3_REGION="YOUR_REGION"
mcap-extractor -i "http://your_minio:port/bucket_name/path/to/one_of_the_mcap_file.mcap" -o /path/to/output --topics="/h264,/lidar"
```

## Build
Build the binary
```bash
cargo build --release
```

Build the deb package if you want to share it with others
```bash
cargo deb --package mcap-extractor --install
```

## Installation
Download the latest release from the [releases page](https://github.com/yinguobing/mcap-extractor/releases).

Install using `dpkg`:
```bash
sudo dpkg -i mcap-extractor_<version>_amd64.deb
```
