# NVIDIA Video Codec SDK

[![crates.io](https://img.shields.io/crates/v/nvidia-video-codec-sdk?style=for-the-badge)](https://crates.io/crates/nvidia-video-codec-sdk)
[![docs.rs](https://img.shields.io/docsrs/nvidia-video-codec-sdk?label=docs.rs%20latest&style=for-the-badge)](https://docs.rs/nvidia-video-codec-sdk)

Rust bindings for [NVIDIA Video Codec SDK](https://developer.nvidia.com/video-codec-sdk).

The documentation is also hosted on GitHub Pages
[here](https://viliamvadocz.github.io/nvidia-video-codec-sdk/nvidia_video_codec_sdk/).

Versions:
- NVIDIA Video Codec SDK 12.1.14
- CUDA 12.2 (older CUDA versions should also work)

## Installation

The build script will try to automatically locate your NVIDIA Video Codec SDK installation.
You can help it by setting the environment variable `NVIDIA_VIDEO_CODEC_SDK_PATH` to the directory containing the library files. 
- `nvEncodeAPI.lib` and `nvcuvid.lib` on Windows,
- `libnvidia-encode.so` and `libnvcuvid.so` on Linux.

With the `dynamic-loading` feature nothing is linked at build time, so the SDK libraries
aren't needed to build. The encoder library is opened when it is first used, and
`library_available` says whether it can be, so a program can start on a machine without the
NVIDIA driver and fall back to another encoder. The raw NVDEC bindings in `sys` aren't usable
with this feature.
