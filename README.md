# xcap
Single binary to extract ROS messages from MCAP files.

## Features
- Supported messages: CompressedImage, PointCloud2
- Support sliced MCAP files.
- Support MinIO as input source.
- Visualization with Rerun

## Usage
Xcap supports MCAP file extracting and visualizing.

### Extract
Extract from a local directory containing multiple MCAP files
```bash
xcap extract -i /path/to/mcap/dir -o /path/to/output --topics="/lidar,/image"
```

Extract from a MinIO bucket:
```bash
export S3_ACCESS_KEY="YOUR_KEY"
export S3_SECRET_KEY="YOUR_SECRET"
export S3_REGION="YOUR_REGION"
xcap extract -i "http://your_minio:port/bucket_name/path/to/one_of_the_mcap_file.mcap" -o /path/to/output --topics="/lidar,/image"
```

In case you want to preview the content during extracing, use `--preview` flag:
```bash
xcap extract -i /path/to/mcap/dir -o /path/to/output --topics="/lidar" --preview
```

### Visualize
A [Rerun](https://rerun.io/) viewer is required to visualize the data. Install it first:
```bash
# install with
cargo install rerun-cli
# or..
pip install rerun-sdk
```

Visualize the data, do not dump any to disk.
```bash
xcap visualize -i /path/to/mcap/dir --topics="/lidar"
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

## Installation
Download the latest release from the [releases page](https://github.com/yinguobing/xcap/releases).

Install using `dpkg`:
```bash
sudo dpkg -i xcap_<version>_amd64.deb
```
