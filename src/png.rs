use crate::math::make_divisible_by;
use crate::unpack::unpack;
use anyhow::{bail, Result};
use png::{BitDepth, ColorType};
use std::io::Read;
use BitDepth::*;
use ColorType::*;

pub struct Png {
    pub width: usize,
    pub height: usize,
    pub color_type: ColorType,
    pub bit_depth: BitDepth,
    data: Vec<u8>,
    palette: Option<Vec<u8>>,
    bpp: usize,
    divisor: usize,
}

impl Png {
    pub fn new(
        width: usize,
        height: usize,
        color_type: ColorType,
        bit_depth: BitDepth,
        data: Vec<u8>,
        palette: Option<Vec<u8>>,
    ) -> Result<Png> {
        if color_type == Indexed && palette.is_none() {
            bail!("Image with indexed color type is missing a palette");
        }

        // Divisor must account for BPP (bits-per-pixel) and picture width
        let bpp = bit_depth as usize * color_type.samples();
        let divisor = if bpp < 8 {
            // Sub-byte encoded data may consist of indivisible picture-wide spans
            // Even if not, make sure the data may only be split between whole bytes
            let usable_line_bits = width * bpp;
            if usable_line_bits % 8 > 0 { width } else { 8 / bpp }
        } else {
            // Full-byte encoded data is allowed to be subdivided freely
            1
        };

        Ok(Png { width, height, color_type, bit_depth, data, palette, bpp, divisor })
    }

    pub fn load<R: Read>(input: &mut R) -> Result<Png> {
        let decoder = png::Decoder::new(input);
        let mut reader = decoder.read_info()?;
        let mut data = vec![0; reader.output_buffer_size()];

        let frame_info = reader.next_frame(&mut data)?;
        let width = frame_info.width as usize;
        let height = frame_info.height as usize;
        let color_type = frame_info.color_type;
        let bit_depth = frame_info.bit_depth;
        let palette = reader.info().palette.as_ref().map(|p| p.to_vec());

        // Leave only the first frame data
        data.truncate(frame_info.buffer_size());

        Self::new(width, height, color_type, bit_depth, data, palette)
    }

    /// Convert into a single chunk for further processing. 
    pub fn as_chunk(&self) -> PngChunk {
        let len = self.width * self.height;
        PngChunk { data: &self.data, len, span: len, png: self }
    }

    /// Split into multiple chunks, useful for further parallel processing.
    pub fn chunks(&self, target_len: usize) -> Vec<PngChunk> {
        // Adjust target chunk length to be divisible by the divisor 
        let len = make_divisible_by(target_len, self.divisor);

        // Calculate corresponding chunk length in bytes
        let bits_divisor = make_divisible_by(self.divisor * self.bpp, 8);
        let data_len = len / self.divisor * bits_divisor / 8;

        // Calculate span, which is needed later for unpacking
        let span = if self.bpp < 8 && self.divisor != 8 / self.bpp { self.divisor } else { len };

        let data_chunks: Vec<&[u8]> = self.data
            .chunks(data_len)
            .collect();

        let mut chunks = Vec::with_capacity(data_chunks.len());
        for i in 0..data_chunks.len() - 1 {
            let data_chunk = data_chunks[i];
            chunks.push(PngChunk { data: data_chunk, len, span, png: self })
        }
        if data_chunks.len() > 0 {
            let data_chunk = data_chunks[data_chunks.len() - 1];
            let remainder = (self.width * self.height) % len;
            let len = if remainder > 0 { remainder } else { len };
            chunks.push(PngChunk { data: data_chunk, len, span, png: self })
        }
        chunks
    }
}

pub struct PngChunk<'a> {
    pub data: &'a [u8],
    pub len: usize,
    span: usize,
    png: &'a Png,
}

impl PngChunk<'_> {
    /// Convert the chunk into RGB 8-bit format (3 bytes per pixel).
    pub fn rgb(&self) -> Vec<u8> {
        match self.png.color_type {
            Indexed => self.indexed_to_rgb(),
            Grayscale => self.grayscale_to_rgb(),
            GrayscaleAlpha => self.grayscale_alpha_to_rgb(),
            Rgb => self.rgb_to_rgb(),
            Rgba => self.rgba_to_rgb(),
        }
    }

    /// Convert the chunk into RGBA 8-bit format (4 bytes per pixel).
    pub fn rgba(&self) -> Vec<u8> {
        match self.png.color_type {
            Indexed => self.indexed_to_rgba(),
            Grayscale => self.grayscale_to_rgba(),
            GrayscaleAlpha => self.grayscale_alpha_to_rgba(),
            Rgb => self.rgb_to_rgba(),
            Rgba => self.rgba_to_rgba(),
        }
    }

    fn indexed_to_rgb(&self) -> Vec<u8> {
        let input = self.unpack();
        let mut output = vec![0; self.len * 3];
        let palette = self.png.palette.as_ref().unwrap();

        for pixel in 0..self.len {
            let palette_index = input[pixel] as usize;
            let palette_offset = palette_index * 3;

            let offset = pixel * 3;
            output[offset + 0] = palette[palette_offset + 0];
            output[offset + 1] = palette[palette_offset + 1];
            output[offset + 2] = palette[palette_offset + 2];
        }

        output
    }

    fn indexed_to_rgba(&self) -> Vec<u8> {
        let input = self.unpack();
        let mut output = vec![0; self.len * 4];
        let palette = self.png.palette.as_ref().unwrap();

        for pixel in 0..self.len {
            let palette_index = input[pixel] as usize;
            let palette_offset = palette_index * 3;

            let offset = pixel * 4;
            output[offset + 0] = palette[palette_offset + 0];
            output[offset + 1] = palette[palette_offset + 1];
            output[offset + 2] = palette[palette_offset + 2];
            output[offset + 3] = 255;
        }

        output
    }

    fn grayscale_to_rgb(&self) -> Vec<u8> {
        let input = self.unpack();
        let mut output = vec![0; self.len * 3];

        let multiplier = self.grayscale_multiplier();
        for pixel in 0..self.len {
            let grey = input[pixel] * multiplier;

            let output_offset = pixel * 3;
            output[output_offset + 0] = grey;
            output[output_offset + 1] = grey;
            output[output_offset + 2] = grey;
        }

        output
    }

    fn grayscale_to_rgba(&self) -> Vec<u8> {
        let input = self.unpack();
        let mut output = vec![0; self.len * 4];

        let multiplier = self.grayscale_multiplier();
        for pixel in 0..self.len {
            let grey = input[pixel] * multiplier;

            let output_offset = pixel * 4;
            output[output_offset + 0] = grey;
            output[output_offset + 1] = grey;
            output[output_offset + 2] = grey;
            output[output_offset + 3] = 255;
        }

        output
    }

    fn grayscale_alpha_to_rgb(&self) -> Vec<u8> {
        let input = self.unpack();
        let mut output = vec![0; self.len * 3];

        let multiplier = self.grayscale_multiplier();
        for pixel in 0..self.len {
            let input_offset = pixel * 2;
            let grey = input[input_offset] * multiplier;

            let output_offset = pixel * 3;
            output[output_offset + 0] = grey;
            output[output_offset + 1] = grey;
            output[output_offset + 2] = grey;
        }

        output
    }

    fn grayscale_alpha_to_rgba(&self) -> Vec<u8> {
        let input = self.unpack();
        let mut output = vec![0; self.len * 4];

        let multiplier = self.grayscale_multiplier();
        for pixel in 0..self.len {
            let input_offset = pixel * 2;
            let grey = input[input_offset] * multiplier;
            let alpha = input[input_offset + 1] * multiplier;

            let output_offset = pixel * 4;
            output[output_offset + 0] = grey;
            output[output_offset + 1] = grey;
            output[output_offset + 2] = grey;
            output[output_offset + 3] = alpha;
        }

        output
    }

    fn rgb_to_rgb(&self) -> Vec<u8> {
        self.unpack()
    }

    fn rgb_to_rgba(&self) -> Vec<u8> {
        let input = self.unpack();
        let mut output = vec![0; self.len * 4];

        for pixel in 0..self.len {
            let input_offset = pixel * 3;
            let r = input[input_offset];
            let g = input[input_offset + 1];
            let b = input[input_offset + 2];

            let output_offset = pixel * 4;
            output[output_offset + 0] = r;
            output[output_offset + 1] = g;
            output[output_offset + 2] = b;
            output[output_offset + 3] = 255;
        }

        output
    }

    fn rgba_to_rgb(&self) -> Vec<u8> {
        let input = self.unpack();
        let mut output = vec![0; self.len * 3];

        for pixel in 0..self.len {
            let input_offset = pixel * 4;
            let r = input[input_offset];
            let g = input[input_offset + 1];
            let b = input[input_offset + 2];

            let output_offset = pixel * 3;
            output[output_offset + 0] = r;
            output[output_offset + 1] = g;
            output[output_offset + 2] = b;
        }

        output
    }

    fn rgba_to_rgba(&self) -> Vec<u8> {
        self.unpack()
    }

    fn grayscale_multiplier(&self) -> u8 {
        match self.png.bit_depth {
            One => 255,
            Two => 85,
            Four => 17,
            _ => 1
        }
    }

    fn unpack(&self) -> Vec<u8> {
        unpack(self.data, self.len, self.span, self.png.bit_depth)
    }
}
