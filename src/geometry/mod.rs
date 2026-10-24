
use glam::prelude::*;


pub trait Geometry {
  fn bounds(&self)-> Aabb;
}

#[derive(Clone,Copy,Debug,PartialEq)]
pub struct Aabb {
  pub min: Vec3,
  pub max: Vec3,
}

#[derive(Clone,Debug,PartialEq)]
pub struct Circle {
  pub radius: f32,
}

#[derive(Clone,Debug,PartialEq)]
pub struct Line(pub Vec3,pub Vec3);

#[derive(Clone,Debug,PartialEq)]
pub struct Rectangle {
  pub width: f32,
  pub height: f32,
}

#[derive(Clone,Debug,PartialEq)]
pub struct PolyLine(Vec<Vec3>);




























