use std::iter::{repeat, repeat_with};
use std::ops::{Index, IndexMut};
use crate::Float;


#[derive(Clone, Debug)]
pub struct Texture2D<T> {
    width: u32,
    height: u32,
    pixels: Vec<T>,
}
impl<T> Texture2D<T> {
    pub fn new(width: u32, height: u32) -> Self
        where T: Default {
        let pixels = Vec::from_iter(repeat_with(|| T::default()).take(width as usize * height as usize));
        
        Self {
            width,
            height,
            pixels
        }
    }
    pub fn new_from(width: u32, height: u32, value: T) -> Self
        where T: Clone {
        let pixels = Vec::from_iter(repeat(value).take(width as usize * height as usize));

        Self {
            width,
            height,
            pixels
        }
    }
    pub fn new_from_pixels(width: u32, height: u32, pixels: Vec<T>) -> Self {
        assert_eq!(width as usize * height as usize, pixels.len());
        
        Self {
            width,
            height,
            pixels,
        }
    }
    
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn read_pixel(&self, i: PixelCoord2D) -> T
        where T: Clone {
        self[i].clone()
    }
    pub fn write_pixel(&mut self, i: PixelCoord2D, value: T) {
        let i = i.to_pixel_index(self.width);

        self.pixels[i] = value;
    }
    
    
    pub fn pixels(&self) -> impl Iterator<Item = &T> {
        self.pixels.iter()
    }
    pub fn into_pixels(self) -> impl Iterator<Item = T> {
        self.pixels.into_iter()
    }
}
impl<T> Index<PixelCoord2D> for Texture2D<T> {
    type Output = T;

    fn index(&self, index: PixelCoord2D) -> &Self::Output {
        let i = index.to_pixel_index(self.width);

        &self.pixels[i]
    }
}
impl<T> IndexMut<PixelCoord2D> for Texture2D<T> {
    fn index_mut(&mut self, index: PixelCoord2D) -> &mut Self::Output {
        let i = index.to_pixel_index(self.width);
        &mut self.pixels[i]
    }
}
impl<T> Index<TextureCoord2D> for Texture2D<T> {
    type Output = T;

    fn index(&self, index: TextureCoord2D) -> &Self::Output {
        let i = index.to_pixel_coord(self.width, self.height);
        &self[i]
    }
}
impl<T> IndexMut<TextureCoord2D> for Texture2D<T> {
    fn index_mut(&mut self, index: TextureCoord2D) -> &mut Self::Output {
        let i = index.to_pixel_coord(self.width, self.height);
        &mut self[i]
    }
}


/// A 2d pixel coordinate.
/// Upper left corner is [0, 0].
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PixelCoord2D {
    pub x: u32,
    pub y: u32,
}
impl PixelCoord2D {
    pub fn to_pixel_index(&self, width: u32) -> usize {
        (self.y * width + self.x) as usize
    }
}


/// A 2d normalized texture coordinate.
/// Lower left corner is [0.0, 0.0].
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TextureCoord2D {
    pub x: Float,
    pub y: Float,
}
impl TextureCoord2D {
    pub fn to_pixel_coord(&self, width: u32, height: u32) -> PixelCoord2D {
        let fwidth = width as Float;
        let fheight = height as Float;

        let x = self.x * fwidth;
        let y = self.y * fheight;

        let x = x.floor() as u32;
        let mut y = y.floor() as u32;

        y = width - 1 - y;


        PixelCoord2D {
            x,
            y,
        }
    }
}
