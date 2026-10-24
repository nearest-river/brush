
mod srgb;

pub use srgb::*;

use glam::prelude::*;
use core::ops::{
  Add,
  Sub,
  Mul,
  Div,
  Rem,
  AddAssign,
  SubAssign,
  MulAssign,
  DivAssign,
  RemAssign,
};


#[derive(Clone,Debug,PartialEq)]
pub struct Style {
  pub fill: Option<Color>,
  pub stroke: Option<Color>,
  pub stroke_width: f32,
  pub opacity: f32,
}

#[derive(Clone,Copy,Debug,PartialEq)]
pub struct Color {
  pub r: f32,
  pub g: f32,
  pub b: f32,
  pub a: f32,
}

pub trait IntoColor: Sized {
  fn into_color(self)-> Color;
}

impl Default for Style {
  fn default()-> Self {
    Self {
      fill: None,
      stroke: Some(Color::BLACK),
      stroke_width: 0.025,
      opacity: 1.0,
    }
  }
}

impl Color {
  pub const BLACK: Self=color(0.0,0.0,0.0,1.0);
  pub const WHITE: Self=color(1.0,1.0,1.0,1.0);
  pub const TRANSPARENT: Self=color(0.0,0.0,0.0,0.0);
  pub const RED: Self=color(1.0,0.0,0.0,1.0);
  pub const GREEN: Self=color(0.0,1.0,0.0,1.0);
  pub const BLUE: Self=color(0.0,0.0,1.0,1.0);
  pub const CYAN: Self=color(0.0,1.0,1.0,1.0);
  pub const MAGENTA: Self=color(1.0,0.0,1.0,1.0);
  pub const YELLOW: Self=color(1.0,1.0,0.0,1.0);

  #[inline(always)]
  pub const fn new(r: f32,g: f32,b: f32,a: f32)-> Self {
    assert!(is_normalized(r) && is_normalized(g) && is_normalized(b) && is_normalized(a));
    Self { r, g, b, a }
  }

  #[inline(always)]
  pub const unsafe fn new_unchecked(r: f32,g: f32,b: f32,a: f32)-> Self {
    Self { r, g, b, a }
  }

  #[inline(always)]
  pub const fn from_array([r,g,b,a]: [f32;4])-> Self {
    Self::new(r,g,b,a)
  }

  #[inline(always)]
  pub fn from_vec4(v: Vec4)-> Self {
    Self::new(v.x,v.y,v.z,v.w)
  }

  #[inline(always)]
  pub fn from_vec3(v: Vec3)-> Self {
    Self::from_vec4(v.extend(1.0))
  }

  #[inline(always)]
  pub const fn as_vec4(self)-> Vec4 {
    let Self { r, g, b, a }=self;
    vec4(r,g,b,a)
  }

  #[inline(always)]
  pub const fn as_rgba(self)-> Rgba {
    Rgba::from_color(self)
  }

  #[inline(always)]
  pub const fn as_rgb(self)-> Rgb {
    Rgb::from_color(self)
  }

  #[inline(always)]
  pub(crate) const fn into_wgpu_color(self)-> wgpu::Color {
    let Color { r, g, b, a }=self;
    let r=r as f64;
    let g=g as f64;
    let b=b as f64;
    let a=a as f64;
    wgpu::Color { r, g, b, a }
  }
}

impl Default for Color {
  #[inline(always)]
  fn default()-> Self {
    Self::BLACK
  }
}

#[inline(always)]
pub const fn color(r: f32,g: f32,b: f32,a: f32)-> Color {
  Color::new(r,g,b,a)
}

macro_rules! impl_ops {
  (
    $name:ident:
      $($trait_name:ident -> $op:ident)*
  )=> {
    $(
      impl $trait_name for $name {
        type Output=Self;
        #[inline(always)]
        fn $op(self,other: Self)-> Self::Output {
          Self::from_vec4(self.as_vec4().$op(other.as_vec4()))
        }
      }
    )*
  };
}

macro_rules! impl_ops_assign {
  (
    $name:ident:
      $($trait_name:ident -> $op:ident)*
  )=> {
    $(
      impl $trait_name for $name {
        #[inline(always)]
        fn $op(&mut self,other: Self) {
          let mut this_vec=self.as_vec4();
          this_vec.$op(other.as_vec4());

          *self=Self::from_vec4(this_vec);
        }
      }
    )*
  };
}

impl_ops! {
  Color:
    Add -> add
    Sub -> sub
    Mul -> mul
    Div -> div
    Rem -> rem
}

impl_ops_assign! {
  Color:
    AddAssign -> add_assign
    SubAssign -> sub_assign
    MulAssign -> mul_assign
    DivAssign -> div_assign
    RemAssign -> rem_assign
}

impl IntoColor for Color {
  #[inline(always)]
  fn into_color(self)-> Color {
    self
  }
}

impl Into<wgpu::Color> for Color {
  #[inline(always)]
  fn into(self)-> wgpu::Color {
    self.into_wgpu_color()
  }
}


#[inline(always)]
const fn is_normalized(x: f32)-> bool {
  x.is_finite() && x>=0.0 && x<=1.0
}

