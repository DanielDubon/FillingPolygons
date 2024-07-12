use std::fs::File;
use std::io::{self, Write};

pub struct Bitmap {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

impl Bitmap {
    pub fn new(width: u32, height: u32) -> Self {
        let data = vec![0; (width * height * 3) as usize];
        Self { width, height, data }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: (u8, u8, u8)) {
        if x >= self.width || y >= self.height {
            return;
        }
        let index = ((y * self.width + x) * 3) as usize;
        self.data[index] = color.2;     // Blue
        self.data[index + 1] = color.1; // Green
        self.data[index + 2] = color.0; // Red
    }

    pub fn save(&self, filename: &str) -> io::Result<()> {
        let mut file = File::create(filename)?;

        let file_header_size = 14;
        let info_header_size = 40;
        let file_size = file_header_size + info_header_size + self.data.len();

        // File header
        file.write_all(&[
            0x42, 0x4D, // Signature "BM"
            (file_size & 0xFF) as u8,
            ((file_size >> 8) & 0xFF) as u8,
            ((file_size >> 16) & 0xFF) as u8,
            ((file_size >> 24) & 0xFF) as u8,
            0, 0, 0, 0, // Reserved
            (file_header_size + info_header_size) as u8, 0, 0, 0,
        ])?;

        // Info header
        file.write_all(&[
            info_header_size as u8, 0, 0, 0, // Header size
            (self.width & 0xFF) as u8,
            ((self.width >> 8) & 0xFF) as u8,
            ((self.width >> 16) & 0xFF) as u8,
            ((self.width >> 24) & 0xFF) as u8,
            (self.height & 0xFF) as u8,
            ((self.height >> 8) & 0xFF) as u8,
            ((self.height >> 16) & 0xFF) as u8,
            ((self.height >> 24) & 0xFF) as u8,
            1, 0, // Planes
            24, 0, // Bits per pixel
            0, 0, 0, 0, // Compression (none)
            (self.data.len() & 0xFF) as u8,
            ((self.data.len() >> 8) & 0xFF) as u8,
            ((self.data.len() >> 16) & 0xFF) as u8,
            ((self.data.len() >> 24) & 0xFF) as u8,
            0, 0, 0, 0, // X pixels per meter
            0, 0, 0, 0, // Y pixels per meter
            0, 0, 0, 0, // Total colors
            0, 0, 0, 0, // Important colors
        ])?;

        // Pixel data
        file.write_all(&self.data)?;

        Ok(())
    }
}