use std::ops::{Deref, DerefMut};
use crate::physic_engine::VerletObject;

pub struct SyncVec {
    vec: Vec<VerletObject>,
}

impl SyncVec {
    pub fn new(vec: Vec<VerletObject>) -> Self {
        Self {
            vec,
        }
    }
}

unsafe impl Send for SyncVec {}

unsafe impl Sync for SyncVec {}

#[derive(Debug)]
pub struct SyncUniformGridSimple(pub uniform_grid_simple::UniformGridSimple);

unsafe impl Send for SyncUniformGridSimple {}

unsafe impl Sync for SyncUniformGridSimple {}

impl Deref for SyncUniformGridSimple {
    type Target = uniform_grid_simple::UniformGridSimple;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SyncUniformGridSimple {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone, Copy)]
pub struct WorkerData(pub *mut SyncVec,
                      pub *const SyncUniformGridSimple);

unsafe impl Send for WorkerData {}

unsafe impl Sync for WorkerData {}


impl Deref for SyncVec {
    type Target = Vec<VerletObject>;

    fn deref(&self) -> &Self::Target {
        &self.vec
    }
}

impl DerefMut for SyncVec {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.vec
    }
}
