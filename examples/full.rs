use std::{
    ffi::c_void,
    fs::{File, OpenOptions},
    io::Write,
    sync::Arc,
};

use cudarc::driver::{CudaDevice, DevicePtr};
#[allow(deprecated)]
use nvidia_video_codec_sdk::sys::nvEncodeAPI::NV_ENC_PRESET_LOW_LATENCY_HP_GUID;
use nvidia_video_codec_sdk::{
    safe::encoder::Encoder,
    sys::nvEncodeAPI::{
        NV_ENC_BUFFER_FORMAT::NV_ENC_BUFFER_FORMAT_ARGB,
        NV_ENC_CODEC_H264_GUID,
        NV_ENC_H264_PROFILE_HIGH_GUID,
        NV_ENC_INITIALIZE_PARAMS,
        NV_ENC_INPUT_RESOURCE_TYPE,
        NV_ENC_PIC_PARAMS,
        NV_ENC_PIC_STRUCT,
        NV_ENC_REGISTER_RESOURCE,
        NV_ENC_TUNING_INFO,
    },
};
use vulkano::{
    device::{
        physical::PhysicalDeviceType,
        Device,
        DeviceCreateInfo,
        DeviceExtensions,
        QueueCreateInfo,
    },
    instance::{Instance, InstanceCreateInfo},
    memory::{
        DeviceMemory,
        ExternalMemoryHandleType,
        ExternalMemoryHandleTypes,
        MappedDeviceMemory,
        MemoryAllocateInfo,
        MemoryPropertyFlags,
    },
    VulkanLibrary,
};

/// Returns the color `(r, g, b, alpha)` of a pixel on the screen relative to
/// its position on a screen:
///
/// Top right will be red,
/// bottom left will be green,
/// all colors will shift towards having more blue as `time` increases.
///
/// # Arguments
///
/// * `width`, `height` - Width and height of the screen.
/// * `x`, `y` - Coordinates of the pixel on the screen.
/// * time - Fraction indicating what part of the animation we are in [0,1]
fn get_color(width: u32, height: u32, x: u32, y: u32, time: f32) -> (u8, u8, u8, u8) {
    let alpha = 255;
    let red = (255 * x / width) as u8;
    let green = (255 * y / height) as u8;
    let blue = (255. * time) as u8;
    (blue, green, red, alpha)
}

/// Generates test frame inputs and sets `buf` to that input.
///
/// # Arguments
///
/// * `buf` - The buffer in which to put the generated input.
/// * `width`, `height` - The size of the frames to generate input for.
/// * `i`, `i_max` - The current frame and total amount of frames.
fn generate_test_input(buf: &mut [u8], width: u32, height: u32, i: u32, i_max: u32) {
    assert_eq!(buf.len(), (width * height * 4) as usize);
    for y in 0..height {
        for x in 0..width {
            let pixel = width * y + x;
            let index = (pixel * 4) as usize;
            let color = get_color(width, height, x, y, i as f32 / i_max as f32);
            buf[index] = color.0;
            buf[index + 1] = color.1;
            buf[index + 2] = color.2;
            buf[index + 3] = color.3;
        }
    }
}

/// Creates an encoded bitstream for a 128 frame, 1920x1080 video.
/// This bitstream will be written to ./test.bin
/// To view this bitstream use a decoder like ffmpeg.
///
/// For ffmpeg use `ffmpeg -i test.bin -vcodec copy test.mp4` to
/// decode the video.
fn main() {
    const WIDTH: u32 = 1920;
    const HEIGHT: u32 = 1080;
    const FRAMES: u32 = 128;

    // Initialize Vulkan.
    let vulkan_library = VulkanLibrary::new().expect("Vulkan should be installed correctly");
    let instance = Instance::new(
        vulkan_library,
        InstanceCreateInfo::application_from_cargo_toml(),
    )
    .expect("Vulkan should be installed correctly");

    let (memory_type_index, physical_device) = instance
        .enumerate_physical_devices()
        .expect("There should be some device capable of encoding")
        .filter_map(|pd| {
            matches!(pd.properties().device_type, PhysicalDeviceType::DiscreteGpu)
                .then_some(())
                .and_then(|()| {
                    pd.memory_properties()
                        .memory_types
                        .iter()
                        .position(|mt| {
                            mt.property_flags
                                .contains(MemoryPropertyFlags::HOST_VISIBLE)
                        })
                        .map(|index| (index as u32, pd))
                })
        })
        .next()
        .expect(
            "There should be at least one GPU which supports a memory type that is `HOST_VISIBLE`",
        );

    // Create a Vulkan device.
    let (vulkan_device, _queues) = Device::new(physical_device, DeviceCreateInfo {
        queue_create_infos: vec![QueueCreateInfo::default()],
        enabled_extensions: DeviceExtensions {
            khr_external_memory_fd: true,
            ..Default::default()
        },
        ..Default::default()
    })
    .expect(
        "Vulkan should be installed correctly and `Device` should support `khr_external_memory_fd`",
    );

    // Create a new CudaDevice to interact with cuda.
    let cuda_device = CudaDevice::new(0).expect("Cuda should be installed correctly");

    let encoder = Encoder::initialize_with_cuda(cuda_device.clone()).expect(
        "NVENC API initialization should succeed given that the NVIDIA Video Codec SDK has been \
         installed correctly",
    );

    // Get all encode guids supported by the GPU.
    let encode_guids = encoder
        .get_encode_guids()
        .expect("The encoder should be able to get the supported guids");
    let encode_guid = NV_ENC_CODEC_H264_GUID;
    assert!(encode_guids.contains(&encode_guid));

    // Get available preset guids based on encode guid.
    let preset_guids = encoder
        .get_preset_guids(encode_guid)
        .expect("The encoder should have a preset for H.264");
    #[allow(deprecated)]
    let preset_guid = NV_ENC_PRESET_LOW_LATENCY_HP_GUID;
    assert!(preset_guids.contains(&preset_guid));

    // Get available profiles based on encode guid.
    let profile_guids = encoder
        .get_profile_guids(encode_guid)
        .expect("The encoder should have a profile for H.264");
    let profile_guid = NV_ENC_H264_PROFILE_HIGH_GUID;
    assert!(profile_guids.contains(&profile_guid));

    // Get input formats based on the encode guid.
    let input_formats = encoder
        .get_supported_input_formats(encode_guid)
        .expect("The encoder should be able to receive input buffer formats");
    let buffer_format = NV_ENC_BUFFER_FORMAT_ARGB;
    assert!(input_formats.contains(&buffer_format));

    // Get the preset config based on the selected encode guid (H.264), selected
    // preset (`LOW_LATENCY`), and tuning info (`ULTRA_LOW_LATENCY`).
    let mut preset_config = encoder
        .get_preset_config(
            encode_guid,
            preset_guid,
            NV_ENC_TUNING_INFO::NV_ENC_TUNING_INFO_ULTRA_LOW_LATENCY,
        )
        .expect("Encoder should be able to create config based on presets");

    // Initialize a new encoder session based on the `preset_config`
    // we generated before.
    let session = encoder
        .initialize_encoder_session(
            NV_ENC_INITIALIZE_PARAMS::new(encode_guid, WIDTH, HEIGHT)
                .display_aspect_ratio(16, 9)
                .framerate(30, 1)
                .enable_picture_type_decision()
                .encode_config(&mut preset_config.presetCfg),
        )
        .expect("Encoder should be initialized correctly");

    // Calculate the number of buffers we need based on the interval of P frames and
    // the look ahead depth.
    let num_bufs = usize::try_from(preset_config.presetCfg.frameIntervalP)
        .expect("frame intervalP should always be positive")
        + usize::try_from(preset_config.presetCfg.rcParams.lookaheadDepth)
            .expect("lookahead depth should always be positive");

    let mut output_buffers: Vec<_> = (0..num_bufs)
        .map(|_| {
            session
                .create_output_bitstream()
                .expect("The encoder should be able to create bitstreams")
        })
        .collect();

    // Write result to output file "test.bin".
    let mut out_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("test.bin")
        .expect("Permissions and available space should allow creating a new file");

    // Generate each of the frames with Vulkan.
    let file_descriptors = (0..FRAMES)
        .map(|f| {
            create_buffer(
                vulkan_device.clone(),
                memory_type_index,
                WIDTH,
                HEIGHT,
                f,
                FRAMES,
            )
        })
        .collect::<Vec<_>>();

    // Encode each of the frames.
    for (i, file_descriptor) in file_descriptors.into_iter().enumerate() {
        let output_buffer = &mut output_buffers[i % num_bufs];

        // Import file descriptor using CUDA.
        let mut external_memory = unsafe {
            cuda_device.import_external_memory(file_descriptor, (WIDTH * HEIGHT * 4) as u64)
        }
        .expect("File descriptor should be valid for importing.");
        let mapped_buffer = external_memory
            .map_all()
            .expect("External memory should be mappable.");

        // Register and map with NVENC.
        let (mut input_buffer, buf_fmt) = session
            .register_and_map_input_resource(
                NV_ENC_REGISTER_RESOURCE::new(
                    NV_ENC_INPUT_RESOURCE_TYPE::NV_ENC_INPUT_RESOURCE_TYPE_CUDADEVICEPTR,
                    WIDTH,
                    HEIGHT,
                    *mapped_buffer.device_ptr() as *mut c_void,
                    buffer_format,
                )
                .pitch(WIDTH * 4),
            )
            .expect("Should be able to register buffer with right size as nvenc resource");
        assert_eq!(buffer_format, buf_fmt);

        session
            .encode_picture(NV_ENC_PIC_PARAMS::new(
                WIDTH,
                HEIGHT,
                &mut input_buffer,
                output_buffer,
                buffer_format,
                NV_ENC_PIC_STRUCT::NV_ENC_PIC_STRUCT_FRAME,
            ))
            .expect("Encoder should be able to encode valid pictures");

        let out = output_buffer
            .lock_and_read(true)
            .expect("Bitstream lock should be available.");
        out_file
            .write_all(out)
            .expect("Writing should succeed because `out_file` was opened with write permissions");

        // Drop registered resource before dropping the CUDA mapped buffer.
        // TODO: `MappedResource` should store a reference or `PhantomData`
        // to enforce drop order.
        drop(input_buffer);
    }
}

/// Allocates memory on a Vulkan [`Device`] and returns a [`File`] (file
/// descriptor) to that data.
///
/// Will be used to create file descriptors for the invidual frames.
///
/// # Arguments
///
/// * `vulkan_device` - The device where the data should be allocated.
/// * `memory_type_index` - The index of the memory type that should be
///   allocated.
/// * `width`, `height` - The size of data to store.
/// * `i`, `i_max`: - The current frame and maximum frame index.
fn create_buffer(
    vulkan_device: Arc<Device>,
    memory_type_index: u32,
    width: u32,
    height: u32,
    i: u32,
    i_max: u32,
) -> File {
    let size = (width * height * 4) as u64;

    // Allocate memory with Vulkan.
    let memory = DeviceMemory::allocate(vulkan_device, MemoryAllocateInfo {
        allocation_size: size,
        memory_type_index,
        export_handle_types: ExternalMemoryHandleTypes::OPAQUE_FD,
        ..Default::default()
    })
    .expect("There should be space to allocate vulkan memory on the device");

    // Map and write to the memory.
    let mapped_memory = MappedDeviceMemory::new(memory, 0..size)
        .expect("There should be memory available to map and write to");
    unsafe {
        let content = mapped_memory
            .write(0..size)
            .expect("The physical device and memory type is HOST_VISIBLE");
        generate_test_input(content, width, height, i, i_max);
        mapped_memory.flush_range(0..size).expect(
            "There should be no other devices writing to this memory and size should also fit \
             within the size",
        );
    }

    // Export the memory.
    mapped_memory
        .unmap()
        .export_fd(ExternalMemoryHandleType::OpaqueFd)
        .expect("The memory should be able to be turned into a file handle if we are on UNIX")
}
