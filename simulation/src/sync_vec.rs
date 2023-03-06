use std::ops::{Deref, DerefMut};
use crate::physic_engine::VerletObject;

pub struct SyncVec{
    vec: Vec<VerletObject>,
}

impl SyncVec {
    pub fn new(vec:Vec<VerletObject>) -> Self{
        Self {
            vec,
        }
    }
}

// unsafe impl Send for SyncVec{}
// unsafe impl Sync for SyncVec{}
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
