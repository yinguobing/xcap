# xcap

A command-line tool for extracting, trimming, and visualizing ROS 2 messages from [MCAP](https://mcap.dev/) log files.

MCAP is the standard container format for robotics log data. xcap reads MCAP files recorded by ROS 2 nodes and lets you pull out specific topics — images, point clouds, compressed video, odometry — to standard file formats, trim them by time range, or stream them live into a 3D scene with [Rerun](https://rerun.io/).

## Features

- **Extract messages by topic** — dump `sensor_msgs/Image`, `sensor_msgs/PointCloud2`, `nav_msgs/Odometry`, H.264-compressed video, and `geometry_msgs/PointStamped` to disk
- **Output formats** — images (PNG), point clouds (PCD or raw binary), video frames (JPEG or other formats), odometry logs
- **Trim by time range** — slice an MCAP file to a given window using RFC 3339 timestamps
- **3D visualization via Rerun** — view point clouds, camera images, odometry trajectories, and tracking markers in a live 3D scene with an ego-vehicle 3D model
- **Text log overlay** — parse timestamped text logs into the Rerun timeline alongside ROS data
- **Remote input via MinIO / S3** — download MCAP files from any S3-compatible object store before processing
- **Transparent Zstandard decompression** — automatically handles ZSTD-compressed messages
- **Progress reporting** — per-topic progress bars with graceful Ctrl‑C handling
- **Packaged for Debian** — `.deb` packages available via `cargo deb`

## Supported message types

| Topic / message type | Extraction output | Visualization |
|---|---|---|
| `sensor_msgs/Image` | PNG files | 2D image view |
| `custom_msgs/CompressedImage` (H.264) | JPEG / configurable image frames | 2D image view |
| `sensor_msgs/PointCloud2` | PCD or raw binary | 3D point cloud |
| `nav_msgs/Odometry` | JSON text log | 3D trajectory + latency scalar |
| `geometry_msgs/PointStamped` | text log | 3D tracking marker |
| `builtin_interfaces/Time` | — | timeline anchor |

## Installation

### From a `.deb` package

```bash
sudo dpkg -i xcap_<version>_amd64.deb
```

### From source

```bash
# Clone and build
git clone https://github.com/yinguobing/xcap.git
cd xcap
cargo build --release

# The binary is at target/release/xcap
```

#### System dependencies

The `openh264` and `ndarray-linalg` (OpenBLAS) crates require:

```bash
# Debian / Ubuntu
sudo apt install libssl-dev libopenblas-dev pkg-config
```

## Usage

```
xcap <COMMAND>

Commands:
  extract  Extract ROS messages from MCAP files
  show     Visualize ROS messages from MCAP files
  trim     Trim MCAP files
  help     Print this message or the help of the given subcommand(s)
```

### `xcap extract`

Extract messages by topic from MCAP files to disk.

```bash
# From a single MCAP file
xcap extract /path/to/recording.mcap -o /path/to/output --topics /lidar /image_raw

# From a directory of MCAP files (sliced recordings)
xcap extract /path/to/mcap_dir -o /path/to/output --topics /lidar /image_raw

# From a mix of files and directories
xcap extract /path/to/file.mcap /path/to/dir -o /path/to/output --topics /lidar /camera

# With live preview in Rerun
xcap extract /path/to/mcap_dir -o /path/to/output --topics /lidar --preview
```

**Notable flags:**

| Argument / Flag | Description |
|---|---|
| `<INPUT>...` | Positional — one or more `.mcap` files, directories, or remote S3 URLs |
| `-o`, `--output-dir` | Directory to write extracted files into |
| `--topics` | Space-separated topic names to extract |
| `--preview` | Open a Rerun viewer alongside extraction |
| `--time-off`, `--time-stop` | Filter by time range (RFC 3339, e.g. `"2025-10-01 10:00:00+08:00"`) |
| `--point-cloud-scale` | Scale point cloud coordinates (default `1.0`) |
| `--intensity-scale` | Scale point cloud intensity values (default `1.0`) |
| `--as-binary` | Dump raw binary instead of PCD for point clouds |
| `--video-header` | Override missing H.264 codec header bytes |
| `--video-frame-format` | Output format for video frames (default `jpeg`) |

### `xcap show`

Visualize data in Rerun without writing anything to disk.

```bash
xcap show /path/to/mcap_dir --topics /lidar /image_raw
```

In addition to the flags shared with `extract`, `show` supports:

| Flag | Description |
|---|---|
| `--log-files` | Space-separated text log files to overlay on the Rerun timeline |
| `--viewer-url` | Connect to an already-running Rerun viewer |

The Rerun viewer must be installed separately:

```bash
# Either via cargo
cargo install rerun-cli

# or via pip
pip install rerun-sdk
```

When built with the `native_viewer` feature (enabled by default), xcap embeds the viewer and opens it automatically. Otherwise it spawns an external `rerun` process.

### `xcap trim`

Trim an MCAP file to a time window.

```bash
xcap trim /path/to/mcap_dir \
    --time-off "2024-12-05 09:50:20" \
    --time-stop "2024-12-05 09:50:25" \
    -o /path/to/output
```

| Argument / Flag | Description |
|---|---|
| `<INPUT>...` | Positional — one or more `.mcap` files, directories, or remote S3 URLs |
| `-o`, `--output-dir` | Directory for the trimmed MCAP file |
| `--time-off` | Start of the window (RFC 3339) |
| `--time-stop` | End of the window (RFC 3339) |

## Remote input (MinIO / S3)

xcap can pull MCAP files directly from an S3-compatible object store. Pass the HTTP endpoint of one file in the bucket as the input and set three environment variables:

```bash
export S3_ACCESS_KEY="YOUR_KEY"
export S3_SECRET_KEY="YOUR_SECRET"
export S3_REGION="YOUR_REGION"

xcap extract \
    "http://minio:9000/bucket-name/path/to/file.mcap" \
    -o /tmp/output \
    --topics /lidar /image
```

xcap will download all `.mcap` files in the same prefix, process them, and clean up the temporary download directory.

## Building

```bash
# Release binary
cargo build --release

# Create a .deb package
cargo deb --package xcap
```

Cross-compilation is configured in `Cross.toml` for `x86_64-unknown-linux-gnu`.

## Project structure

```
xcap/                        # Main crate (binary + library)
├── src/
│   ├── bin/xcap.rs          # CLI entry point (clap)
│   ├── lib.rs               # Core library: summary(), dump_n_visualize(), trim()
│   ├── extractor.rs         # Extractor trait for per-message-type parsers
│   ├── parser/              # Message-type parsers (image, pointcloud, h264, odom, …)
│   ├── storage.rs           # MinIO / S3 download agent
│   ├── visual.rs            # Visual trait for Rerun streaming
│   └── textlog.rs           # Text-log timeline parser
├── assets/                  # 3D models (ego vehicle, map pin)
└── Cargo.toml

ros2-message-traits/         # Shared Message / Service traits
ros2-interfaces-humble/      # Auto-generated ROS 2 Humble message structs
```

## License

[GPL-3.0](LICENSE) — Copyright 2026 Yin Guobing
