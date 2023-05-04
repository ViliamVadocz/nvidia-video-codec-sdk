#!/bin/bash
bindgen \
    --allowlist-type cudaVideo.* \
    --allowlist-type cuvid.* \
    --allowlist-type CUVID.* \
    --allowlist-function cuvid.* \
    --blocklist-file .*/cuda\.h \
    --blocklist-file .*/std.*\.h \
    \
    --default-enum-style=rust \
    --no-doc-comments \
    --with-derive-default \
    --with-derive-eq \
    --with-derive-hash \
    --with-derive-ord \
    --use-core \
    --merge-extern-blocks \
    --output cuviddec.rs ./headers/cuviddec.h
bindgen \
    --allowlist-type CU.* \
    --allowlist-type cudaVideo.* \
    --allowlist-type cudaAudio.* \
    --allowlist-type HEVC.* \
    --allowlist-function cuvid.* \
    --blocklist-file .*/cuda\.h \
    --blocklist-file .*/std.*\.h \
    --blocklist-file .*/cuviddec\.h \
    \
    --default-enum-style=rust \
    --no-doc-comments \
    --with-derive-default \
    --with-derive-eq \
    --with-derive-hash \
    --with-derive-ord \
    --use-core \
    --merge-extern-blocks \
    --output nvcuvid.rs ./headers/nvcuvid.h
bindgen \
    --allowlist-type NVENC.* \
    --allowlist-type NV_ENC.* \
    --allowlist-type NV_ENCODE.* \
    --allowlist-type GUID \
    --allowlist-type PENV.* \
    --allowlist-function NvEncodeAPI.* \
    --allowlist-function NvEnc.* \
    --blocklist-file .*/win.*\.h \
    --blocklist-file .*/cuda\.h \
    --blocklist-file .*/std.*\.h \
    --blocklist-file .*/cuviddec\.h \
    \
    --default-enum-style=rust \
    --no-doc-comments \
    --with-derive-default \
    --with-derive-eq \
    --with-derive-hash \
    --with-derive-ord \
    --use-core \
    --merge-extern-blocks \
    --output nvEncodeAPI.rs ./headers/nvEncodeAPI.h
