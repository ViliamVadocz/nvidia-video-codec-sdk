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

Nvidia library stubs are used, dynamic linking at runtime.
You should have these available for your dynamic loader to load:

- `nvEncodeAPI.lib` and `nvcuvid.lib` on Windows,
- `libnvidia-encode.so` and `libnvcuvid.so` on Linux.
