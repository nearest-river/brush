
use crate::prelude::*;
use core::ops::{
  Index,
  IndexMut,
};


#[derive(Default)]
pub struct Scene {
  objects: Vec<Object>,
  next_id: usize,
}

#[derive(Clone,Copy,Debug,PartialEq,Eq,Hash)]
pub struct ObjectId(usize);

pub struct Object {
  pub id: ObjectId,
  pub style: Style,
  pub transform: Transform2,
  pub geometry: Box<dyn Geometry>,
}


impl Scene {
  #[inline(always)]
  pub const fn new()-> Self {
    Self {
      next_id: 0,
      objects: Vec::new(),
    }
  }

  #[inline(always)]
  pub fn insert<G: Into<Box<dyn Geometry>>>(&mut self,geometry: G)-> ObjectId {
    let id=ObjectId(self.next_id);

    self.next_id=self.next_id.checked_add(1)
    .expect("object space overflowed");
    self.objects.push(Object::new(id,geometry));

    id
  }

  #[inline(always)]
  pub fn get(&self,id: ObjectId)-> Option<&Object> {
    self.objects.get(id.0)
  }

  #[inline(always)]
  pub fn get_mut(&mut self,id: ObjectId)-> Option<&mut Object> {
    self.objects.get_mut(id.0)
  }

  #[inline(always)]
  pub unsafe fn get_unchecked(&self,id: ObjectId)-> &Object {
    // SAFETY(nate): atp im just trusting you bro.
    unsafe {
      self.objects.get_unchecked(id.0)
    }
  }

  #[inline(always)]
  pub unsafe fn get_unchecked_mut(&mut self,id: ObjectId)-> &mut Object {
    // SAFETY(nate): atp im just trusting you bro.
    unsafe {
      self.objects.get_unchecked_mut(id.0)
    }
  }

  #[inline(always)]
  pub fn objects(&self)-> impl ExactSizeIterator<Item=&Object> {
    self.objects.iter()
  }
}

impl Index<ObjectId> for Scene {
  type Output=Object;

  #[inline(always)]
  fn index(&self,index: ObjectId)-> &Self::Output {
    self.objects.index(index.0)
  }
}

impl IndexMut<ObjectId> for Scene {
  #[inline(always)]
  fn index_mut(&mut self,index: ObjectId)-> &mut Self::Output {
    self.objects.index_mut(index.0)
  }
}




impl Object {
  pub fn new<G: Into<Box<dyn Geometry>>>(id: ObjectId,geometry: G)-> Self {
    Self {
      id,
      style: Style::default(),
      geometry: geometry.into(),
      transform: Transform2::default(),
    }
  }
}



