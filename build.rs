extern crate bindgen;

use std::env;
use std::path::PathBuf;

// https://docs.nvidia.com/video-technologies/video-codec-sdk/12.0/nvenc-video-encoder-api-prog-guide/index.html#basic-encoding-flow
#[cfg(target_os = "windows")]
const NVENC_LIB: &str = "nvEncode";
#[cfg(target_os = "linux")]
const NVENC_LIB: &str = "nvidia-encode";

// https://docs.nvidia.com/video-technologies/video-codec-sdk/12.0/nvdec-video-decoder-api-prog-guide/index.html#using-nvidia-video-decoder-nvdecode-api
const NVDEC_LIB: &str = "nvcuvid";

// Taken from https://github.com/coreylowman/cudarc/blob/main/build.rs
const CUDA_ROOT_ENV_VARS: [&str; 4] = [
    "CUDA_PATH",
    "CUDA_ROOT",
    "CUDA_TOOLKIT_ROOT_DIR",
    "NVIDIA_VIDEO_CODEC_SDK_PATH",
];
const CUDA_ROOT_CANDIDATES: [&str; 6] = [
    "/usr",
    "/usr/local/cuda",
    "/opt/cuda",
    "/usr/lib/cuda",
    "C:/Program Files/NVIDIA GPU Computing Toolkit",
    "C:/CUDA",
];
const LIBRARY_CANDIDATES: [&str; 10] = [
    "lib",
    "lib/x64",
    "lib/Win32",
    "lib/x86_64",
    "lib/x86_64-linux-gnu",
    "lib64",
    "lib64/stubs",
    "targets/x86_64-linux",
    "targets/x86_64-linux/lib",
    "targets/x86_64-linux/lib/stubs",
];

fn main() {
    rerun_if_changed();
    add_directories_to_library_search_path();
    link_to_libraries();
    generate_bindings();
}

fn rerun_if_changed() {
    for var in CUDA_ROOT_ENV_VARS {
        println!("cargo:rerun-if-env-changed={var}");
    }
    println!("cargo:rerun-if-changed=wrapper.h",);
}

fn add_directories_to_library_search_path() {
    let roots = root_candidates().filter(|path| {
        let join_include = path.join("include");
        join_include.join("cuviddec.h").is_file()
            || join_include.join("nvcuvid.h").is_file()
            || join_include.join("nvEncodeAPI.h").is_file()
    });

    let mut at_least_one = false;
    for path in roots.flat_map(lib_candidates) {
        let Ok(canonical) = path.canonicalize() else { continue };
        println!("cargo:rustc-link-search={}", canonical.display());
        at_least_one = true;
    }
    if !at_least_one {
        eprintln!(
            "Could not find the required headers.\n\
            Set the `NVIDIA_VIDEO_CODEC_SDK_PATH` environment variable\n\
            so that the headers are at `$NVIDIA_VIDEO_CODEC_SDK_PATH/include/`.\n\
            Make sure the libraries are at `$NVIDIA_VIDEO_CODEC_SDK_PATH/lib/`.\n"
        );
    }
}

fn link_to_libraries() {
    println!("cargo:rustc-link-lib={NVENC_LIB}");
    println!("cargo:rustc-link-lib={NVDEC_LIB}");
}

fn generate_bindings() {
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Could not write bindings");
}

fn root_candidates() -> impl Iterator<Item = PathBuf> {
    let env_vars = CUDA_ROOT_ENV_VARS
        .into_iter()
        .map(std::env::var)
        .filter_map(Result::ok);
    let roots = CUDA_ROOT_CANDIDATES.into_iter().map(Into::into);
    env_vars.chain(roots).map(Into::<PathBuf>::into)
}

fn lib_candidates(root: PathBuf) -> impl Iterator<Item = PathBuf> {
    LIBRARY_CANDIDATES
        .into_iter()
        .map(move |p| root.join(p))
        .filter(|p| p.is_dir())
}
