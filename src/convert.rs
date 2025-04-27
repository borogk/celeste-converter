use anyhow::{Result, anyhow};
use image::GenericImageView;
use std::io::{BufRead, Read, Seek, Write};

/// Converts DATA to PNG, uses 'png' library for better performance.
/// Output is limited to 24-bit RGB or 32-bit RGBA.
pub fn data_to_png<R: Read, W: Write>(input: &mut R, output: &mut W) -> Result<()> {
    let width = read_u32(input)?;
    let height = read_u32(input)?;
    let has_alpha = read_bool(input)?;

    println!("DATA image parameters: {width}x{height}, has alpha: {has_alpha}");

    let bytes_per_pixel: usize = if has_alpha { 4 } else { 3 };
    let mut png_data = Vec::with_capacity(bytes_per_pixel * (width * height) as usize);
    let mut i = 0;
    while i < width * height {
        // Read RLE count
        let count = read_u8(input)?;
        if count == 0 {
            return Err(anyhow!("Unexpected count value of 0"));
        }

        // Read individual channel values and compile them into RGBA
        let rgba = if has_alpha {
            let a = read_u8(input)?;
            if a != 0 {
                let b = read_u8(input)?;
                let g = read_u8(input)?;
                let r = read_u8(input)?;
                [r, g, b, a]
            } else {
                [0, 0, 0, a]
            }
        } else {
            let b = read_u8(input)?;
            let g = read_u8(input)?;
            let r = read_u8(input)?;
            [r, g, b, 0]
        };

        // Output the next span of same-colored pixels
        let rgba_slice = &rgba[0..bytes_per_pixel];
        for _ in 0..count {
            png_data.extend_from_slice(rgba_slice);
        }

        i += count as u32
    }

    let mut png_encoder = png::Encoder::new(output, width, height);
    png_encoder.set_depth(png::BitDepth::Eight);
    png_encoder.set_color(if has_alpha { png::ColorType::Rgba } else { png::ColorType::Rgb });

    let mut png_writer = png_encoder.write_header()?;
    png_writer.write_image_data(&png_data)?;

    Ok(())
}

/// Converts PNG into DATA, uses 'image' library for better compatibility.
pub fn png_to_data<R: BufRead + Seek, W: Write>(input: &mut R, output: &mut W) -> Result<()> {
    let png = image::ImageReader::with_format(input, image::ImageFormat::Png).decode()?;

    let width = png.width();
    let height = png.height();
    let has_alpha = png.color().has_alpha();

    println!("PNG image parameters: {width}x{height}, has alpha: {has_alpha}");

    // Write image headers (width, height and alpha channel flag)
    write_u32(output, width)?;
    write_u32(output, height)?;
    write_bool(output, has_alpha)?;

    let mut i = 0;
    while i < width * height {
        // Take color value of the current pixel
        let x = i % width;
        let y = i / width;
        let rgba = png.get_pixel(x, y);

        // Calculate RLE count by looking ahead at the next pixels
        let mut count = 1u8;
        loop {
            // Don't step out of bounds
            if i + count as u32 >= width * height {
                break;
            }

            // Compare with next pixel color
            let x2 = (i + count as u32) % width;
            let y2 = (i + count as u32) / width;
            let rgba2 = png.get_pixel(x2, y2);
            if rgba2 != rgba {
                break;
            }

            // Increment, but don't exceed maximum 8-bit value
            count += 1;
            if count == 0xFF {
                break;
            }
        }

        // Extract individual channel values
        let image::Rgba([r, g, b, a]) = rgba;

        // Write RLE count and RGB(A) channel values
        write_u8(output, count)?;
        if has_alpha {
            write_u8(output, a)?;
            if a != 0 {
                write_u8(output, b)?;
                write_u8(output, g)?;
                write_u8(output, r)?;
            }
        } else {
            write_u8(output, b)?;
            write_u8(output, g)?;
            write_u8(output, r)?;
        }

        i += count as u32;
    }

    Ok(())
}

fn read_bool<R: Read>(input: &mut R) -> Result<bool> {
    let mut buf = [0];
    input.read_exact(&mut buf)?;
    Ok(buf[0] != 0)
}

fn read_u8<R: Read>(input: &mut R) -> Result<u8> {
    let mut buf = [0];
    input.read_exact(&mut buf)?;
    Ok(buf[0])
}

fn read_u32<R: Read>(input: &mut R) -> Result<u32> {
    let mut buf = [0; 4];
    input.read_exact(&mut buf)?;
    Ok(u32::from_le_bytes(buf))
}

fn write_bool<W: Write>(output: &mut W, value: bool) -> Result<()> {
    let buf = [value as u8];
    output.write_all(&buf)?;
    Ok(())
}

fn write_u8<W: Write>(output: &mut W, value: u8) -> Result<()> {
    let buf = [value];
    output.write_all(&buf)?;
    Ok(())
}

fn write_u32<W: Write>(output: &mut W, value: u32) -> Result<()> {
    let buf = u32::to_le_bytes(value);
    output.write_all(&buf)?;
    Ok(())
}
