pub mod safe;
pub mod sys;

#[cfg(test)]
mod tests {
    use std::{fs::OpenOptions, io::Write};

    use cudarc::driver::CudaDevice;

    use crate::{
        safe::api::EncodeAPI,
        sys::nvEncodeAPI::{
            NV_ENC_BUFFER_FORMAT::NV_ENC_BUFFER_FORMAT_ABGR,
            NV_ENC_CODEC_H264_GUID,
            NV_ENC_H264_PROFILE_HIGH_GUID,
            NV_ENC_INITIALIZE_PARAMS,
            NV_ENC_PIC_PARAMS,
            NV_ENC_PIC_STRUCT,
            NV_ENC_PRESET_LOW_LATENCY_HP_GUID,
            NV_ENC_TUNING_INFO,
        },
    };

    fn checkerboard(width: u32, height: u32, x: u32, y: u32) -> (u8, u8, u8) {
        // Arbitrary value controlling checkerboard size.
        const SKIP: u32 = 160;
        let mut color = (255, 255, 0); // Blue always 0
        if (y % SKIP < SKIP / 2) == (x % SKIP < SKIP / 2) {
            color.0 = 0;
            color.1 = 0;
        } else {
            // Red
            if (x / SKIP) < (width / SKIP / 2) {
                color.0 = 127;
            }
            // Green
            if (y / SKIP) < (height / SKIP / 2) {
                color.1 = 127;
            }
        }
        color
    }

    fn generate_test_input(width: u32, height: u32, i: u32, i_max: u32) -> Vec<u8> {
        let mut buf = vec![0; (width * height * 4) as usize];
        let f = 1.0 - (i as f32 / i_max as f32); // Used for fade out.
        for x in 0..width {
            for y in 0..height {
                let pixel = width * y + x;
                let index = (pixel * 4) as usize;
                let color = checkerboard(width, height, x, y);
                buf[index] = (255.0 * f) as u8;
                buf[index + 1] = (color.0 as f32 * f) as u8;
                buf[index + 2] = (color.1 as f32 * f) as u8;
                buf[index + 3] = (color.2 as f32 * f) as u8;
            }
        }
        buf
    }

    #[allow(non_snake_case)]
    #[test]
    fn example() {
        const WIDTH: u32 = 1920;
        const HEIGHT: u32 = 1080;

        let cuda_device = CudaDevice::new(0).unwrap();

        let encode_api = EncodeAPI::new().unwrap();
        let encoder = encode_api
            .open_encode_session_with_cuda(cuda_device)
            .unwrap();

        let encode_guids = encoder.get_encode_guids().unwrap();
        let encode_guid = NV_ENC_CODEC_H264_GUID;
        assert!(encode_guids.contains(&encode_guid));

        let preset_guids = encoder.get_preset_guids(encode_guid).unwrap();
        let preset_guid = NV_ENC_PRESET_LOW_LATENCY_HP_GUID;
        assert!(preset_guids.contains(&preset_guid));

        let profile_guids = encoder.get_profile_guids(encode_guid).unwrap();
        let profile_guid = NV_ENC_H264_PROFILE_HIGH_GUID;
        assert!(profile_guids.contains(&profile_guid));

        let input_formats = encoder.get_supported_input_formats(encode_guid).unwrap();
        let buffer_format = NV_ENC_BUFFER_FORMAT_ABGR;
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

        for i in 0..128 {
            let input_buffer = &mut input_buffers[(i % num_bufs) as usize];
            let output_buffer = &mut output_buffers[(i % num_bufs) as usize];

            let input = generate_test_input(WIDTH, HEIGHT, i, 128);
            input_buffer.write(false, &input).unwrap();

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
            let out = output_buffer.read().unwrap();
            out_file.write_all(out).unwrap();
            println!("Wrote {} bytes to file", out.len());
        }

        // 5.1. Notifying the End of Input Stream
        // Note that output is still generated here

        let output_buffer = &mut output_buffers[0];
        encoder
            .encode_picture(
                NV_ENC_PIC_PARAMS::new(
                    WIDTH,
                    HEIGHT,
                    &input_buffers[0],
                    output_buffer,
                    buffer_format,
                    NV_ENC_PIC_STRUCT::NV_ENC_PIC_STRUCT_FRAME,
                )
                .end_of_stream(),
            )
            .unwrap();

        let out = output_buffer.read().unwrap();
        out_file.write_all(out).unwrap();
        println!("Wrote {} bytes to file", out.len());
    }
}
