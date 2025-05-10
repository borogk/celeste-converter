use crate::log;
use crate::png_chunk::PngChunk;
use anyhow::{anyhow, Result};
use png::ColorType;
use rayon::prelude::*;
use std::io::{Cursor, Read, Write};

const CHUNK_SIZE: usize = 65536;

/// Converts DATA to PNG. Output is limited to 24-bit RGB or 32-bit RGBA.
pub fn data_to_png<R: Read, W: Write>(input: &mut R, output: &mut W) -> Result<()> {
    log!("Converting DATA into PNG...");

    let width = read_u32(input)?;
    let height = read_u32(input)?;
    let has_alpha = read_bool(input)?;

    log!("DATA image parameters: {width}x{height}, has alpha: {has_alpha}");

    let mut input_data = Vec::new();
    input.read_to_end(&mut input_data)?;

    let output_data = if has_alpha {
        data_to_png_chunk_rgba(input_data.as_slice())?
    } else {
        let input_chunks: Vec<&[u8]> = input_data.chunks(CHUNK_SIZE * 4).collect();
        let output_chunks: Vec<Vec<u8>> = input_chunks
            .par_iter()
            .map(|c| data_to_png_chunk_rgb(c).unwrap())
            .collect();

        let capacity = output_chunks.iter().map(|c| c.len()).sum();
        let mut output_data = Vec::with_capacity(capacity);
        for chunk in output_chunks {
            output_data.extend_from_slice(&chunk);
        }
        output_data
    };

    let mut png_encoder = png::Encoder::new(output, width, height);
    png_encoder.set_depth(png::BitDepth::Eight);
    png_encoder.set_color(if has_alpha { ColorType::Rgba } else { ColorType::Rgb });

    let mut png_writer = png_encoder.write_header()?;
    png_writer.write_image_data(&output_data)?;

    Ok(())
}

/// Converts PNG into DATA.
pub fn png_to_data<R: Read, W: Write>(input: &mut R, output: &mut W) -> Result<()> {
    log!("Converting PNG into DATA...");

    let mut reader = png::Decoder::new(input).read_info()?;
    let mut all_frames_data = vec![0; reader.output_buffer_size()];

    // Only read the first frame
    let png_info = &reader.next_frame(&mut all_frames_data)?;
    let png_palette = &reader.info().palette;
    let png_data = PngChunk::new(
        &all_frames_data[..png_info.buffer_size()],
        png_info,
        png_palette,
    );

    let width = png_info.width;
    let height = png_info.height;
    let has_alpha = png_info.color_type == ColorType::Rgba || png_info.color_type == ColorType::GrayscaleAlpha;
    let color_type_str = match png_info.color_type {
        ColorType::Indexed => "Indexed",
        ColorType::Grayscale => "Grayscale",
        ColorType::GrayscaleAlpha => "Grayscale alpha",
        ColorType::Rgb => "RGB",
        ColorType::Rgba => "RGBA",
    };
    log!("PNG input: {}x{}, color type: {}, bit depth: {}", width, height, color_type_str, png_info.bit_depth as u8);

    // Write image headers (width, height and alpha channel flag)
    write_u32(output, width)?;
    write_u32(output, height)?;
    write_bool(output, has_alpha)?;

    let output_chunks: Vec<Vec<u8>> = if has_alpha {
        png_data
            .chunks(CHUNK_SIZE)
            .par_iter()
            .map(|c| png_to_data_chunk_rgba(c))
            .collect()
    } else {
        png_data
            .chunks(CHUNK_SIZE)
            .par_iter()
            .map(|c| png_to_data_chunk_rgb(c))
            .collect()
    };

    for chunk in output_chunks {
        output.write_all(chunk.as_slice())?;
    }

    Ok(())
}

fn data_to_png_chunk_rgb(input: &[u8]) -> Result<Vec<u8>> {
    let mut output = Vec::new();

    for i in 0..input.len() / 4 {
        let input_offset = i * 4;

        // Read RLE count
        let rle_count = input[input_offset];
        if rle_count == 0 {
            return Err(anyhow!("Unexpected RLE count value of 0"));
        }

        // Read individual channel values
        let b = input[input_offset + 1];
        let g = input[input_offset + 2];
        let r = input[input_offset + 3];

        // Output the next span of same-colored pixels
        for _ in 0..rle_count {
            output.push(r);
            output.push(g);
            output.push(b);
        }
    }

    Ok(output)
}

fn data_to_png_chunk_rgba(input: &[u8]) -> Result<Vec<u8>> {
    let max_position = input.len() as u64;
    let mut input = Cursor::new(input);
    let mut output = Vec::new();

    while input.position() < max_position {
        // Read RLE count
        let rle_count = read_u8(&mut input)? as usize;
        if rle_count == 0 {
            return Err(anyhow!("Unexpected RLE count value of 0"));
        }

        // Read individual channel values
        let r: u8;
        let g: u8;
        let b: u8;
        let a = read_u8(&mut input)?;
        if a != 0 {
            b = read_u8(&mut input)?;
            g = read_u8(&mut input)?;
            r = read_u8(&mut input)?;
        } else {
            b = 0;
            g = 0;
            r = 0;
        }

        // Output the next span of same-colored pixels
        for _ in 0..rle_count {
            output.push(r);
            output.push(g);
            output.push(b);
            output.push(a);
        }
    }

    Ok(output)
}

fn png_to_data_chunk_rgb(input: &PngChunk) -> Vec<u8> {
    let rgb = input.rgb();
    let mut output = Vec::new();

    let mut pixel = 0;
    while pixel < input.pixel_count {
        let offset = pixel * 3;
        let pixel_rgb = &rgb[offset..offset + 3];

        // Calculate RLE count by looking ahead at the next pixels
        let mut rle_count = 1;
        loop {
            // Don't step out of bounds
            if pixel + rle_count >= rle_count {
                break;
            }

            // Compare with next pixel color
            let next_offset = (pixel + rle_count) * 3;
            let next_pixel_rgb = &rgb[next_offset..next_offset + 3];
            if next_pixel_rgb != pixel_rgb {
                break;
            }

            // Increment, but don't exceed maximum 8-bit value
            rle_count += 1;
            if rle_count == 0xFF {
                break;
            }
        }

        // Extract individual channel values
        let r = pixel_rgb[0];
        let g = pixel_rgb[1];
        let b = pixel_rgb[2];

        // Write RLE count and RGB channel values
        write_u8(&mut output, rle_count as u8).unwrap();
        write_u8(&mut output, b).unwrap();
        write_u8(&mut output, g).unwrap();
        write_u8(&mut output, r).unwrap();

        pixel += rle_count;
    }

    output
}

fn png_to_data_chunk_rgba(input: &PngChunk) -> Vec<u8> {
    let rgba = input.rgba();
    let mut output = Vec::new();

    let mut pixel = 0;
    while pixel < input.pixel_count {
        let offset = pixel * 4;
        let pixel_rgba = &rgba[offset..offset + 4];

        // Calculate RLE count by looking ahead at the next pixels
        let mut rle_count = 1;
        loop {
            // Don't step out of bounds
            if pixel + rle_count >= rle_count {
                break;
            }

            // Compare with next pixel color
            let next_offset = (pixel + rle_count) * 4;
            let next_pixel_rgba = &rgba[next_offset..next_offset + 4];
            if next_pixel_rgba != pixel_rgba {
                break;
            }

            // Increment, but don't exceed maximum 8-bit value
            rle_count += 1;
            if rle_count == 0xFF {
                break;
            }
        }

        // Extract individual channel values
        let r = pixel_rgba[0];
        let g = pixel_rgba[1];
        let b = pixel_rgba[2];
        let a = pixel_rgba[3];

        // Write RLE count and RGBA channel values
        write_u8(&mut output, rle_count as u8).unwrap();
        write_u8(&mut output, a).unwrap();
        if a != 0 {
            write_u8(&mut output, b).unwrap();
            write_u8(&mut output, g).unwrap();
            write_u8(&mut output, r).unwrap();
        }

        pixel += rle_count;
    }

    output
}

#[inline]
fn read_bool<R: Read>(input: &mut R) -> Result<bool> {
    let mut buf = [0];
    input.read_exact(&mut buf)?;
    Ok(buf[0] != 0)
}

#[inline]
fn read_u8<R: Read>(input: &mut R) -> Result<u8> {
    let mut buf = [0];
    input.read_exact(&mut buf)?;
    Ok(buf[0])
}

#[inline]
fn read_u32<R: Read>(input: &mut R) -> Result<u32> {
    let mut buf = [0; 4];
    input.read_exact(&mut buf)?;
    Ok(u32::from_le_bytes(buf))
}

#[inline]
fn write_bool<W: Write>(output: &mut W, value: bool) -> Result<()> {
    let buf = [value as u8];
    output.write_all(&buf)?;
    Ok(())
}

#[inline]
fn write_u8<W: Write>(output: &mut W, value: u8) -> Result<()> {
    let buf = [value];
    output.write_all(&buf)?;
    Ok(())
}

#[inline]
fn write_u32<W: Write>(output: &mut W, value: u32) -> Result<()> {
    let buf = u32::to_le_bytes(value);
    output.write_all(&buf)?;
    Ok(())
}
