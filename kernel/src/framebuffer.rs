use core::fmt;

use bootloader_api::info::{FrameBuffer, PixelFormat};
use embedded_graphics::{
    Pixel,
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Size},
    mono_font::{MonoFont, MonoTextStyle, ascii::FONT_10X20},
    pixelcolor::{Rgb888, RgbColor},
    prelude::{Point, *},
    text::Text,
};

use crate::utils::Global;

const FONT: MonoFont = FONT_10X20;
const START_POINT: Point = start_point();
pub static DISPLAY: Global<Display> = Global::uninit();

pub fn init_display(framebuffer: &'static mut bootloader_api::info::FrameBuffer) {
    DISPLAY.set(Display::new(framebuffer));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub struct Display<'f> {
    framebuffer: &'f mut FrameBuffer,
    current_point: Point,
}

const fn start_point() -> Point {
    Point::new(
        FONT.character_size.width as i32,
        FONT.character_size.height as i32,
    )
}

impl<'f> Display<'f> {
    pub fn new<'a>(framebuffer: &'a mut FrameBuffer) -> Self
    where
        'a: 'f,
    {
        Display {
            framebuffer,
            current_point: START_POINT,
        }
    }

    pub fn clear(&mut self) {
        let buffer = self.framebuffer.buffer_mut();
        buffer.fill(0);
    }

    pub fn color<'a>(&'a mut self, color: Rgb888) -> ColoredDisplay<'f>
    where
        'a: 'f,
    {
        ColoredDisplay {
            display: self,
            color,
        }
    }

    fn print<'a>(&mut self, text: &'a str, color: Rgb888) {
        let info = self.framebuffer.info();

        if self.current_point.y >= self.framebuffer.info().height as i32 {
            // move the buffer up by one line
            // this is done by copying the buffer contents from the second line to the first line and filling the last line with zeros
            let line_size = info.stride * info.bytes_per_pixel;
            let buffer = self.framebuffer.buffer_mut();
            buffer.copy_within(line_size * (FONT.character_size.height as usize).., 0);
            self.current_point.y -= FONT.character_size.height as i32;
            let position_y = self.current_point.y as usize;
            buffer[position_y * line_size..].fill(0);
        }

        let style = MonoTextStyle::new(&FONT, color);
        let mut result = Text::new(text, self.current_point, style)
            .draw(self)
            .unwrap();
        if text.ends_with('\n') {
            result = Point::new(FONT.character_size.width as i32, result.y);
        }

        self.current_point = result;
    }

    fn draw_pixel(&mut self, Pixel(coordinates, color): Pixel<Rgb888>) {
        // ignore any out of bounds pixels
        let (width, height) = {
            let info = self.framebuffer.info();

            (info.width, info.height)
        };

        let (x, y) = {
            let c: (i32, i32) = coordinates.into();
            (c.0 as usize, c.1 as usize)
        };

        if (0..width).contains(&x) && (0..height).contains(&y) {
            let color = Color {
                red: color.r(),
                green: color.g(),
                blue: color.b(),
            };

            set_pixel_in(self.framebuffer, Position { x, y }, color);
        }
    }
}

pub fn set_pixel_in(framebuffer: &mut FrameBuffer, position: Position, color: Color) {
    let info = framebuffer.info();

    // calculate offset to first byte of pixel
    let byte_offset = {
        // use stride to calculate pixel offset of target line
        let line_offset = position.y * info.stride;
        // add x position to get the absolute pixel offset in buffer
        let pixel_offset = line_offset + position.x;
        // convert to byte offset
        pixel_offset * info.bytes_per_pixel
    };

    // set pixel based on color format
    let pixel_buffer = &mut framebuffer.buffer_mut()[byte_offset..];
    match info.pixel_format {
        PixelFormat::Rgb => {
            pixel_buffer[0] = color.red;
            pixel_buffer[1] = color.green;
            pixel_buffer[2] = color.blue;
        }
        PixelFormat::Bgr => {
            pixel_buffer[0] = color.blue;
            pixel_buffer[1] = color.green;
            pixel_buffer[2] = color.red;
        }
        PixelFormat::U8 => {
            // use a simple average-based grayscale transform
            let gray = color.red / 3 + color.green / 3 + color.blue / 3;
            pixel_buffer[0] = gray;
        }
        other => panic!("unknown pixel format {other:?}"),
    }
}

impl<'f> fmt::Write for Display<'f> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.print(s, Rgb888::WHITE);
        Ok(())
    }
}

impl<'f> DrawTarget for Display<'f> {
    type Color = Rgb888;

    /// Drawing operations can never fail.
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for pixel in pixels.into_iter() {
            self.draw_pixel(pixel);
        }

        Ok(())
    }
}

impl<'f> OriginDimensions for Display<'f> {
    fn size(&self) -> Size {
        let info = self.framebuffer.info();

        Size::new(info.width as u32, info.height as u32)
    }
}

pub struct ColoredDisplay<'f> {
    display: &'f mut Display<'f>,
    color: Rgb888,
}

impl<'f> fmt::Write for ColoredDisplay<'f> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.display.print(s, self.color);
        Ok(())
    }
}
