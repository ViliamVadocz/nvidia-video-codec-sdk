use std::{env, path::{Path, PathBuf}, process::Command};

/// <a href="https://docs.nvidia.com/video-technologies/video-codec-sdk/12.0/
/// nvenc-video-encoder-api-prog-guide/index.html#basic-encoding-flow"></a>
#[cfg(unix)]
const NVENC_LIB: (&str, &str) = ("nvidia-encode", "libnvidia-encode.so");
#[cfg(windows)]
const NVENC_LIB: (&str, &str) = ("nvencodeapi", "nvEncodeAPI.lib");

/// <a href="https://docs.nvidia.com/video-technologies/video-codec-sdk/12.0/
/// nvdec-video-decoder-api-prog-guide/index.html#
/// using-nvidia-video-decoder-nvdecode-api"></a>
#[cfg(unix)]
const NVDEC_LIB: (&str, &str) = ("nvcuvid", "libnvcuvid.so");
#[cfg(windows)]
const NVDEC_LIB: (&str, &str) = ("nvcuvid", "nvcuvid.lib");

/// Environment variables which might specify path to the libraries.
///
/// - <https://github.com/coreylowman/cudarc/blob/main/build.rs>
/// - <https://github.com/rust-av/nvidia-video-codec-rs/blob/master/nvidia-video-codec-sys/build.rs>
const ENVIRONMENT_VARIABLES: [&str; 5] = [
    "CUDA_PATH",
    "CUDA_ROOT",
    "CUDA_TOOLKIT_ROOT_DIR",
    "NVIDIA_VIDEO_CODEC_SDK_PATH",
    "NVIDIA_VIDEO_CODEC_INCLUDE_PATH",
];

/// Candidate paths which do not require an environment variable.
///
/// - <https://github.com/coreylowman/cudarc/blob/main/build.rs>
/// - <https://github.com/ViliamVadocz/nvidia-video-codec-sdk/issues/13>
const ROOT_CANDIDATES: [&str; 7] = [
    "/usr",
    "/usr/local/cuda",
    "/opt/cuda",
    "/usr/lib/cuda",
    "C:/Program Files/NVIDIA GPU Computing Toolkit",
    "C:/CUDA",
    "/usr/include/nvidia-sdk",
];

fn main() {
    if cfg!(feature = "ci-check") {
        return;
    }
    rerun_if_changed();

    let temp_dir = env::temp_dir();
    compile_library_stub("src/sys/stubs/nvcuvid.c",  NVDEC_LIB.1, temp_dir.to_str().unwrap());
    compile_library_stub("src/sys/stubs/nvEncodeAPI.c", NVENC_LIB.1, temp_dir.to_str().unwrap());

    println!("cargo:rustc-link-search=native={}", temp_dir.as_path().display());

    // Link to libraries.
    println!("cargo:rustc-link-lib=dylib={}", NVENC_LIB.0);
    println!("cargo:rustc-link-lib=dylib={}", NVDEC_LIB.0);
}

/// Rerun the build script if any of the listed environment variables changes.
fn rerun_if_changed() {
    for var in ENVIRONMENT_VARIABLES {
        println!("cargo:rerun-if-env-changed={var}");
    }
}

pub fn find_cuda() -> PathBuf {
    if let Ok(cuda_path) = env::var("CUDA_PATH") {
        let include_path = PathBuf::from(&cuda_path).join("include");
        if include_path.join("cuda.h").exists() {
            return include_path;
        }
    }

    if let Ok(cuda_home) = env::var("CUDA_HOME") {
        let include_path = PathBuf::from(&cuda_home).join("include");
        if include_path.join("cuda.h").exists() {
            return include_path;
        }
    }

    for root in ROOT_CANDIDATES.iter() {
        let include_path = Path::new(root).join("include");
        if include_path.join("cuda.h").exists() {
            return include_path;
        }
    }

    panic!(
        "Could not find CUDA include directory. Set CUDA_PATH or CUDA_HOME, or install CUDA in a standard location."
    );
}

pub fn compile_library_stub(source: &str, library_name_exact: &str, out_dir: &str) {
    let cuda_include_path = find_cuda();
    
    if !cuda_include_path.exists() {
        panic!("CUDA include path does not exist: {}", cuda_include_path.display());
    }
    let out_dir = PathBuf::from(out_dir);

    let lib_path = out_dir.join(library_name_exact);
    
    if cfg!(target_os = "windows") {
        // Windows with MSVC
        let status = Command::new("cl.exe")
            .args(&[
                "/nologo",
                "/LD", // Build as DLL
                "/W0", // No warnings
                &format!("/Fe:{}", lib_path.display()),
                &format!("/I{}", "src/sys/headers"),
                &format!("/I{}", cuda_include_path.display()),
                source,
            ])
            .status()
            .expect("Failed to run cl.exe");
        
        assert!(status.success(), "Failed to compile DLL");
    } else {
        // Unix-like systems (Linux, macOS)
        let status = Command::new("gcc")
            .args(&[
                "-shared",
                "-fPIC",
                "-o",
                lib_path.to_str().unwrap(),
                "-Isrc/sys/headers",
                &format!("-I{}", cuda_include_path.display()),
                "-w",
                source,
            ])
            .status()
            .expect("Failed to run gcc");
        
        assert!(status.success(), "Failed to compile shared library");
    }
}