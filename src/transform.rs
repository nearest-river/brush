
use glam::prelude::*;
use crate::geometry::Geometry;

pub trait Transformation<G: Geometry> {
  type Output: Geometry;
  fn transfrom(&self,geometry: G)-> Self::Output;
}


#[derive(Clone,Copy,Debug)]
pub struct Transform2 {
  pub translation: Vec2,
  pub angle: f32,
  pub scale: Vec2,
}

impl Transform2 {
  #[inline(always)]
  pub fn transformation(&self)-> Mat3A {
    Mat3A::from_scale_angle_translation(self.scale,self.angle,self.translation)
  }
}

impl Default for Transform2 {
  fn default()-> Self {
    Self {
      angle: 0.0,
      scale: Vec2::ONE,
      translation: Vec2::ZERO,
    }
  }
}




























