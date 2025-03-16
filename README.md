# xcap
Single binary to extract ROS messages from MCAP files.

## Features
- Supported messages: `Image`, `CompressedImage`, `PointCloud2`
- Support sliced MCAP files
- Visualization with Rerun
- Support MinIO as input source

## Installation
Download the latest release from the [releases page](https://github.com/yinguobing/xcap/releases).

Install using `dpkg`:
```bash
sudo dpkg -i xcap_<version>_amd64.deb
```

**Optional** Install Rerun viewer if you need realtime visualization:
```bash
# install with
cargo install rerun-cli
# or..
pip install rerun-sdk
```

## Usage
Xcap supports MCAP file extracting, trimming and visualization.

### Extract
Extract from a local directory containing multiple MCAP files
```bash
xcap extract -i /path/to/mcap/dir -o /path/to/output --topics /lidar /image
```

Extract from a MinIO bucket:
```bash
export S3_ACCESS_KEY="YOUR_KEY"
export S3_SECRET_KEY="YOUR_SECRET"
export S3_REGION="YOUR_REGION"
xcap extract -i "http://your_minio:port/bucket_name/path/to/one_of_the_mcap_file.mcap" -o /path/to/output --topics /lidar /image
```

### Trim
Trim the mcap file.
```bash
xcap trim -i /path/to/mcap/dir --time-off "2024-12-05 09:50:20" --time-stop "2024-12-05 09:50:25"
```

### Visualize
Visualize the data, do not dump any to disk.
```bash
xcap show -i /path/to/mcap/dir --topics /lidar
```

## Build
Build the binary
```bash
cargo build --release
```

Build the deb package if you want to share it with others
```bash
cargo deb --package xcap --install
```

