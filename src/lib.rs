pub mod safe;
pub mod sys;

#[macro_use]
extern crate lazy_static;

#[cfg(test)]
mod tests {
    use std::{ffi::c_void, fs::OpenOptions, io::Write, os::fd::AsRawFd};

    use cudarc::driver::{
        sys::{cuMemImportFromShareableHandle, CUmemAllocationHandleType, CUresult},
        CudaDevice,
    };
    use vulkano::{
        device::{Device, DeviceCreateInfo, QueueCreateInfo},
        instance::{Instance, InstanceCreateInfo},
        memory::{DeviceMemory, MemoryAllocateInfo},
        VulkanLibrary,
    };

    #[allow(deprecated)]
    use crate::sys::nvEncodeAPI::NV_ENC_PRESET_LOW_LATENCY_HP_GUID;
    use crate::{
        safe::api::ENCODE_API,
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

    fn get_color(width: u32, height: u32, x: u32, y: u32, t: f32) -> (u8, u8, u8, u8) {
        let alpha = 255;
        let red = (255 * x / width) as u8;
        let green = (255 * y / height) as u8;
        let blue = (255. * t) as u8;
        (blue, green, red, alpha) // order might be dependant on endianness?
    }

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

    #[allow(non_snake_case)]
    #[test]
    fn example() {
        const WIDTH: u32 = 1920;
        const HEIGHT: u32 = 1080;

        let cuda_device = CudaDevice::new(0).unwrap();

        let encoder = ENCODE_API
            .open_encode_session_with_cuda(cuda_device)
            .unwrap();

        let encode_guids = encoder.get_encode_guids().unwrap();
        let encode_guid = NV_ENC_CODEC_H264_GUID;
        assert!(encode_guids.contains(&encode_guid));

        let preset_guids = encoder.get_preset_guids(encode_guid).unwrap();
        #[allow(deprecated)]
        let preset_guid = NV_ENC_PRESET_LOW_LATENCY_HP_GUID;
        assert!(preset_guids.contains(&preset_guid));

        let profile_guids = encoder.get_profile_guids(encode_guid).unwrap();
        let profile_guid = NV_ENC_H264_PROFILE_HIGH_GUID;
        assert!(profile_guids.contains(&profile_guid));

        let input_formats = encoder.get_supported_input_formats(encode_guid).unwrap();
        let buffer_format = NV_ENC_BUFFER_FORMAT_ARGB;
        assert!(input_formats.contains(&buffer_format));

        let mut preset_config = encoder
            .get_preset_config(
                encode_guid,
                preset_guid,
                NV_ENC_TUNING_INFO::NV_ENC_TUNING_INFO_ULTRA_LOW_LATENCY,
            )
            .unwrap();

        encoder
            .initialize_encoder_session(
                NV_ENC_INITIALIZE_PARAMS::new(encode_guid, WIDTH, HEIGHT)
                    .display_aspect_ratio(16, 9)
                    .framerate(30, 1)
                    .enable_picture_type_decision()
                    .encode_config(&mut preset_config.presetCfg),
            )
            .unwrap();

        // ---

        // 4.1.2. Input buffers allocated externally

        // Allocate memory with Vulkan and export it as a POSIX Fd.
        let vulkan_library = VulkanLibrary::new().unwrap();
        let instance = Instance::new(
            vulkan_library,
            InstanceCreateInfo::application_from_cargo_toml(),
        )
        .unwrap();
        let physical_device = instance
            .enumerate_physical_devices()
            .unwrap()
            .next()
            .unwrap();
        let (vulkan_device, mut queues) = Device::new(physical_device, DeviceCreateInfo {
            queue_create_infos: vec![QueueCreateInfo::default()],
            ..Default::default()
        })
        .unwrap();
        let _queue = queues.next().unwrap();
        let memory = DeviceMemory::allocate(vulkan_device, MemoryAllocateInfo {
            allocation_size: (WIDTH * HEIGHT * 4) as u64,
            ..Default::default()
        })
        .unwrap();
        let file = memory
            .export_fd(vulkano::memory::ExternalMemoryHandleType::OpaqueFd)
            .unwrap();

        // Import POSIX Fd with CUDA.
        let mut handle = 0;
        assert_eq!(CUresult::CUDA_SUCCESS, unsafe {
            cuMemImportFromShareableHandle(
                &mut handle,
                file.as_raw_fd() as *mut c_void,
                CUmemAllocationHandleType::CU_MEM_HANDLE_TYPE_POSIX_FILE_DESCRIPTOR,
            )
        });

        let (_input_resource, buf_fmt) = encoder
            .register_and_map_input_resource(NV_ENC_REGISTER_RESOURCE::new(
                NV_ENC_INPUT_RESOURCE_TYPE::NV_ENC_INPUT_RESOURCE_TYPE_CUDADEVICEPTR,
                WIDTH,
                HEIGHT,
                handle as *mut c_void,
                buffer_format,
            ))
            .unwrap();
        assert_eq!(buffer_format, buf_fmt);

        // ---

        // TODO: In the samples they add a constant "extra output delay" to this,
        // investigate?
        let num_bufs = preset_config.presetCfg.frameIntervalP as u32
            + preset_config.presetCfg.rcParams.lookaheadDepth as u32;

        let mut input_buffers: Vec<_> = (0..num_bufs)
            .map(|_| {
                encoder
                    .create_input_buffer(WIDTH, HEIGHT, buffer_format)
                    .unwrap()
            })
            .collect();

        let mut output_buffers: Vec<_> = (0..num_bufs)
            .map(|_| encoder.create_output_bitstream().unwrap())
            .collect();

        let mut out_file = OpenOptions::new()
            .write(true)
            .create(true)
            .open("test.bin")
            .unwrap();

        let mut input_data = vec![0; (WIDTH * HEIGHT * 4) as usize];
        const FRAMES: u32 = 128;
        for i in 0..FRAMES {
            let input_buffer = &mut input_buffers[(i % num_bufs) as usize];
            let output_buffer = &mut output_buffers[(i % num_bufs) as usize];

            generate_test_input(&mut input_data, WIDTH, HEIGHT, i, FRAMES);
            input_buffer.lock_and_write(false, &input_data).unwrap();

            // TODO: Timestamps?
            encoder
                .encode_picture(NV_ENC_PIC_PARAMS::new(
                    WIDTH,
                    HEIGHT,
                    input_buffer,
                    output_buffer,
                    buffer_format,
                    NV_ENC_PIC_STRUCT::NV_ENC_PIC_STRUCT_FRAME,
                ))
                .unwrap();

            // TODO: only read if encode_picture was Ok().
            // It could also ask for more input data!
            let out = output_buffer.lock_and_read().unwrap();
            out_file.write_all(out).unwrap();
        }

        // 5.1. Notifying the End of Input Stream
        // Note that output is still generated here

        let output_buffer = &mut output_buffers[0];
        encoder
            .encode_picture(
                NV_ENC_PIC_PARAMS::new(
                    WIDTH,
                    HEIGHT,
                    &mut input_buffers[0],
                    output_buffer,
                    buffer_format,
                    NV_ENC_PIC_STRUCT::NV_ENC_PIC_STRUCT_FRAME,
                )
                .end_of_stream(),
            )
            .unwrap();

        let out = output_buffer.lock_and_read().unwrap();
        out_file.write_all(out).unwrap();
    }
}
