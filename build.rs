extern crate bindgen;

use std::env;
use std::path::PathBuf;

// https://docs.nvidia.com/video-technologies/video-codec-sdk/12.0/nvenc-video-encoder-api-prog-guide/index.html#basic-encoding-flow
#[cfg(target_os = "windows")]
const NVENC_LIB: &str = "nvencodeapi";
#[cfg(target_os = "linux")]
const NVENC_LIB: &str = "nvidia-encode";

// https://docs.nvidia.com/video-technologies/video-codec-sdk/12.0/nvdec-video-decoder-api-prog-guide/index.html#using-nvidia-video-decoder-nvdecode-api
const NVDEC_LIB: &str = "nvcuvid";

// Taken from https://github.com/coreylowman/cudarc/blob/main/build.rs
const CUDA_ROOT_ENV_VARS: [&str; 3] = ["CUDA_PATH", "CUDA_ROOT", "CUDA_TOOLKIT_ROOT_DIR"];
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
    let cuda_root = find_cuda_root()
        .canonicalize()
        .expect("Could not canonicalize path.");

    println!("cargo:rustc-link-lib={NVENC_LIB}");
    println!("cargo:rustc-link-lib={NVDEC_LIB}");

    for path in lib_candidates(cuda_root.clone()) {
        println!("cargo:rustc-link-search={}", path.display());
    }

    // generate_bindings(cuda_root);
}

fn rerun_if_changed() {
    for var in CUDA_ROOT_ENV_VARS {
        println!("cargo:rerun-if-env-changed={var}");
    }
    println!("cargo:rerun-if-changed=wrapper.h",);
}

fn cuda_root_candidates() -> impl Iterator<Item = PathBuf> {
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

/// We expect both `cuda.h` and all the NVIDIA Video Codec SDK headers to be in the same place.
fn find_cuda_root() -> PathBuf {
    let root = cuda_root_candidates()
        .find(|path| path.join("include").join("cuda.h").is_file())
        .unwrap_or_else(|| {
            panic!(
                "Could not find the CUDA header file `cuda.h`.\n\
                Try setting `CUDA_PATH` so that the header is located at `$CUDA_PATH/include/cuda.h`.\n"
            )
        });
    assert!(
        {
            let include = root.join("include");
            include.join("cuviddec.h").is_file()
                && include.join("nvcuvid.h").is_file()
                && include.join("nvEncodeAPI.h").is_file()
        },
        "Could not find the required NVIDIA Video Codec SDK headers.\n\
        Place the headers at the same location as your CUDA headers.\n\
        That means the headers are at located at `$CUDA_PATH/include/`."
    );
    root
}

fn generate_bindings(cuda_root: PathBuf) {
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", cuda_root.join("include").display()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Windows compatibility https://github.com/rust-lang/rust-bindgen/issues/1562#issuecomment-851038587
        .blocklist_type("_IMAGE_TLS_DIRECTORY64")
        .blocklist_type("IMAGE_TLS_DIRECTORY64")
        .blocklist_type("PIMAGE_TLS_DIRECTORY64")
        .blocklist_type("IMAGE_TLS_DIRECTORY")
        .blocklist_type("PIMAGE_TLS_DIRECTORY")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Could not write bindings");
}
