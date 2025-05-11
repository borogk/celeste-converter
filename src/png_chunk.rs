use png::{BitDepth, ColorType, OutputInfo};
use std::borrow::Cow;
use BitDepth::*;
use ColorType::*;
use crate::unpack::unpack;

pub struct PngChunk<'a> {
    data: &'a [u8],
    png_info: &'a OutputInfo,
    png_palette: &'a Option<Cow<'a, [u8]>>,
    pub width: usize,
    pub height: usize,
}

impl PngChunk<'_> {
    pub fn new<'a>(data: &'a [u8], png_info: &'a OutputInfo, png_palette: &'a Option<Cow<'a, [u8]>>) -> PngChunk<'a> {
        PngChunk { data, png_info, png_palette, width: png_info.width as usize, height: png_info.height as usize }
    }

    pub fn rgb(&self) -> Vec<u8> {
        match self.png_info.color_type {
            Indexed => self.indexed_to_rgb(),
            Grayscale => self.grayscale_to_rgb(),
            GrayscaleAlpha => self.grayscale_alpha_to_rgb(),
            Rgb => self.rgb_to_rgb(),
            Rgba => self.rgba_to_rgb(),
        }
    }

    pub fn rgba(&self) -> Vec<u8> {
        match self.png_info.color_type {
            Indexed => self.indexed_to_rgba(),
            Grayscale => self.grayscale_to_rgba(),
            GrayscaleAlpha => self.grayscale_alpha_to_rgba(),
            Rgb => self.rgb_to_rgba(),
            Rgba => self.rgba_to_rgba(),
        }
    }

    pub fn chunks(&self, target_pixel_count: usize) -> Vec<PngChunk> {
        let width = self.png_info.width as usize;
        let line_size = self.png_info.line_size;

        let chunk_height = target_pixel_count / width + (target_pixel_count % width > 0) as usize;
        let bytes_in_chunk = chunk_height * line_size;

        let byte_chunks: Vec<&[u8]> = self.data
            .chunks(bytes_in_chunk)
            .collect();

        let mut result_chunks = Vec::with_capacity(byte_chunks.len());
        for i in 0..byte_chunks.len() - 1 {
            result_chunks.push(PngChunk {
                data: byte_chunks[i],
                png_info: self.png_info,
                png_palette: self.png_palette,
                width,
                height: chunk_height,
            })
        }
        if byte_chunks.len() > 0 {
            let height_remainder = self.height % chunk_height;
            result_chunks.push(PngChunk {
                data: byte_chunks[byte_chunks.len() - 1],
                png_info: self.png_info,
                png_palette: self.png_palette,
                width,
                height: if height_remainder > 0 { height_remainder } else { chunk_height },
            })
        }
        result_chunks
    }

    fn indexed_to_rgb(&self) -> Vec<u8> {
        let input = self.unpack();
        let mut output = vec![0; self.width * self.height * 3];
        let palette = self.png_palette.as_ref().unwrap();

        for pixel in 0..self.width * self.height {
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
        let mut output = vec![0; self.width * self.height * 4];
        let palette = self.png_palette.as_ref().unwrap();

        for pixel in 0..self.width * self.height {
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
        let mut output = vec![0; self.width * self.height * 3];

        let scale = match self.png_info.bit_depth {
            One => 255,
            Two => 85,
            Four => 17,
            _ => 1
        };

        for pixel in 0..self.width * self.height {
            let grey = input[pixel] * scale;

            let output_offset = pixel * 3;
            output[output_offset + 0] = grey;
            output[output_offset + 1] = grey;
            output[output_offset + 2] = grey;
        }

        output
    }

    fn grayscale_to_rgba(&self) -> Vec<u8> {
        let input = self.unpack();
        let mut output = vec![0; self.width * self.height * 4];

        let scale = match self.png_info.bit_depth {
            One => 255,
            Two => 85,
            Four => 17,
            _ => 1
        };

        for pixel in 0..self.width * self.height {
            let grey = input[pixel] * scale;

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
        let mut output = vec![0; self.width * self.height * 3];

        let scale = match self.png_info.bit_depth {
            One => 255,
            Two => 85,
            Four => 17,
            _ => 1
        };

        for pixel in 0..self.width * self.height {
            let input_offset = pixel * 2;
            let grey = input[input_offset] * scale;

            let output_offset = pixel * 3;
            output[output_offset + 0] = grey;
            output[output_offset + 1] = grey;
            output[output_offset + 2] = grey;
        }

        output
    }

    fn grayscale_alpha_to_rgba(&self) -> Vec<u8> {
        let input = self.unpack();
        let mut output = vec![0; self.width * self.height * 4];

        let scale = match self.png_info.bit_depth {
            One => 255,
            Two => 85,
            Four => 17,
            _ => 1
        };

        for pixel in 0..self.width * self.height {
            let input_offset = pixel * 2;
            let grey = input[input_offset] * scale;
            let alpha = input[input_offset + 1] * scale;

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
        let mut output = vec![0; self.width * self.height * 4];

        for pixel in 0..self.width * self.height {
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
        let mut output = vec![0; self.width * self.height * 3];

        for pixel in 0..self.width * self.height {
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

    fn unpack(&self) -> Vec<u8> {
        unpack(
            self.data,
            self.width,
            self.height,
            self.png_info.line_size,
            self.png_info.bit_depth,
        )
    }
}
