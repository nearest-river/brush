
use glam::*;
use super::{
  Color,
  IntoColor,
};



#[derive(Clone,Copy,Debug,PartialEq)]
pub struct Rgba {
  pub r: u8,
  pub g: u8,
  pub b: u8,
  pub a: u8,
}

#[derive(Clone,Copy,Debug,PartialEq)]
pub struct Rgb {
  pub r: u8,
  pub g: u8,
  pub b: u8,
}


impl Rgba {
  #[inline(always)]
  pub const fn new(r: u8,g: u8,b: u8,a: u8)-> Self {
    Self { r, g, b, a }
  }

  #[inline(always)]
  pub const fn from_u8vec4(U8Vec4 { x, y, z, w }: U8Vec4)-> Self {
    Self::new(x,y,z,w)
  }

  #[inline(always)]
  pub const fn from_u8vec3(U8Vec3 { x, y, z }: U8Vec3)-> Self {
    Self::new(x,y,z,0xff)
  }

  #[inline(always)]
  pub const fn from_array([r,g,b,a]: [u8;4])-> Self {
    Self::new(r,g,b,a)
  }

  #[inline(always)]
  pub const fn from_color(Color { r, g, b, a }: Color)-> Self {
    let r=(r*255.0).round() as u8;
    let g=(g*255.0).round() as u8;
    let b=(b*255.0).round() as u8;
    let a=(a*255.0).round() as u8;

    Self::new(r,g,b,a)
  }

  #[inline(always)]
  pub const fn as_color(self)-> Color {
    let r=self.r as f32 / 255.0;
    let g=self.g as f32 / 255.0;
    let b=self.b as f32 / 255.0;
    let a=self.a as f32 / 255.0;

    // SAFETY(nate): trust me bro.
    unsafe {
      Color::new_unchecked(r,g,b,a)
    }
  }

  #[inline(always)]
  pub const fn as_u8vec4(self)-> U8Vec4 {
    let Rgba { r, g, b, a }=self;
    U8Vec4::new(r,g,b,a)
  }
}

impl Rgb {
  #[inline(always)]
  pub const fn new(r: u8,g: u8,b: u8)-> Self {
    Self { r, g, b }
  }

  #[inline(always)]
  pub const fn from_u8vec3(U8Vec3 { x, y, z }: U8Vec3)-> Self {
    Self::new(x,y,z)
  }

  #[inline(always)]
  pub const fn from_array([r,g,b]: [u8;3])-> Self {
    Self::new(r,g,b)
  }

  #[inline(always)]
  pub const fn from_color(Color { r, g, b, a: _ }: Color)-> Self {
    let r=(r*255.0).round() as u8;
    let g=(g*255.0).round() as u8;
    let b=(b*255.0).round() as u8;

    Self::new(r,g,b)
  }

  #[inline(always)]
  pub const fn as_color(self)-> Color {
    let r=self.r as f32 / 255.0;
    let g=self.g as f32 / 255.0;
    let b=self.b as f32 / 255.0;

    // SAFETY(nate): trust me bro.
    unsafe {
      Color::new_unchecked(r,g,b,1.0)
    }
  }

  #[inline(always)]
  pub const fn as_u8vec3(self)-> U8Vec3 {
    let Rgb { r, g, b }=self;
    U8Vec3::new(r,g,b)
  }
}





impl IntoColor for Rgba {
  #[inline(always)]
  fn into_color(self)-> Color {
    self.as_color()
  }
}

impl IntoColor for Rgb {
  #[inline(always)]
  fn into_color(self)-> Color {
    self.as_color()
  }
}



