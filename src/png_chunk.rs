use png::{BitDepth, ColorType, OutputInfo};
use std::borrow::Cow;
use ColorType::*;
use crate::unpack::unpack;

pub struct PngChunk<'a> {
    data: &'a [u8],
    png_info: &'a OutputInfo,
    png_palette: &'a Option<Cow<'a, [u8]>>,
    pub pixel_count: usize,
}

impl PngChunk<'_> {
    pub fn new<'a>(data: &'a [u8], png_info: &'a OutputInfo, png_palette: &'a Option<Cow<'a, [u8]>>) -> PngChunk<'a> {
        let pixel_count = (png_info.width * png_info.height) as usize;
        PngChunk { data, png_info, png_palette, pixel_count }
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

    pub fn chunks(&self, pixel_count: usize) -> Vec<PngChunk> {
        let channels = self.png_info.color_type.samples();
        self.data
            .chunks(pixel_count * channels)
            .map(|c| PngChunk {
                data: c,
                png_info: self.png_info,
                png_palette: self.png_palette,
                pixel_count: c.len() / channels,
            })
            .collect()
    }

    fn indexed_to_rgb(&self) -> Vec<u8> {
        let input = unpack(self.data, self.pixel_count, self.png_info.bit_depth);
        let mut output = vec![0; self.pixel_count * 3];
        let palette = self.png_palette.as_ref().unwrap();

        for pixel in 0..self.pixel_count {
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
        let input = unpack(self.data, self.pixel_count, self.png_info.bit_depth);
        let mut output = vec![0; self.pixel_count * 4];
        let palette = self.png_palette.as_ref().unwrap();

        for pixel in 0..self.pixel_count {
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
        let input = unpack(self.data, self.pixel_count, self.png_info.bit_depth);
        let mut output = vec![0; self.pixel_count * 3];

        let scale = match self.png_info.bit_depth {
            BitDepth::One => 255,
            BitDepth::Two => 85,
            BitDepth::Four => 17,
            _ => 1
        };

        for pixel in 0..self.pixel_count {
            let grey = input[pixel] * scale;

            let output_offset = pixel * 3;
            output[output_offset + 0] = grey;
            output[output_offset + 1] = grey;
            output[output_offset + 2] = grey;
        }

        output
    }

    fn grayscale_to_rgba(&self) -> Vec<u8> {
        let input = unpack(self.data, self.pixel_count, self.png_info.bit_depth);
        let mut output = vec![0; self.pixel_count * 4];

        let scale = match self.png_info.bit_depth {
            BitDepth::One => 255,
            BitDepth::Two => 85,
            BitDepth::Four => 17,
            _ => 1
        };

        for pixel in 0..self.pixel_count {
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
        let input = unpack(self.data, self.pixel_count * 2, self.png_info.bit_depth);
        let mut output = vec![0; self.pixel_count * 3];

        let scale = match self.png_info.bit_depth {
            BitDepth::One => 255,
            BitDepth::Two => 85,
            BitDepth::Four => 17,
            _ => 1
        };

        for pixel in 0..self.pixel_count {
            let input_offset = pixel * 2;
            let grey = input[input_offset] * scale;

            let output_offset = input_offset * 3;
            output[output_offset + 0] = grey;
            output[output_offset + 1] = grey;
            output[output_offset + 2] = grey;
        }

        output
    }

    fn grayscale_alpha_to_rgba(&self) -> Vec<u8> {
        let input = unpack(self.data, self.pixel_count * 2, self.png_info.bit_depth);
        let mut output = vec![0; self.pixel_count * 4];

        let scale = match self.png_info.bit_depth {
            BitDepth::One => 255,
            BitDepth::Two => 85,
            BitDepth::Four => 17,
            _ => 1
        };

        for pixel in 0..self.pixel_count {
            let input_offset = pixel * 2;
            let grey = input[input_offset] * scale;
            let alpha = input[input_offset + 1] * scale;

            let output_offset = input_offset * 4;
            output[output_offset + 0] = grey;
            output[output_offset + 1] = grey;
            output[output_offset + 2] = grey;
            output[output_offset + 3] = alpha;
        }

        output
    }

    fn rgb_to_rgb(&self) -> Vec<u8> {
        unpack(self.data, self.pixel_count * 3, self.png_info.bit_depth)
    }

    fn rgb_to_rgba(&self) -> Vec<u8> {
        let input = unpack(self.data, self.pixel_count * 3, self.png_info.bit_depth);
        let mut output = vec![0; self.pixel_count * 4];

        for pixel in 0..self.pixel_count {
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
        let input = unpack(self.data, self.pixel_count * 4, self.png_info.bit_depth);
        let mut output = vec![0; self.pixel_count * 3];

        for pixel in 0..self.pixel_count {
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
        unpack(self.data, self.pixel_count * 4, self.png_info.bit_depth)
    }
}
