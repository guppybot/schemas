extern crate bincode;
extern crate serde;
#[macro_use] extern crate serde_derive;

pub use crate::v1 as wire_protocol;

use serde::{Deserialize};

pub mod v1;

pub trait Revise<'a> {
  type RevisionPrev: Revise<'a> + Deserialize<'a>;

  fn revision() -> u32;

  fn migrate_from_revision(_prev_rev: Self::RevisionPrev) -> Option<Self> where Self: Sized {
    None
  }
}

#[derive(Serialize, Deserialize)]
pub enum NoPrev {}

impl<'a> Revise<'a> for NoPrev {
  type RevisionPrev = NoPrev;

  fn revision() -> u32 {
    0
  }
}

pub fn deserialize_revision<'a, T>(rev: u32, bytes: &'a [u8]) -> bincode::Result<T>
where T: Revise<'a> + Deserialize<'a> {
  if rev == T::revision() {
    bincode::deserialize(bytes)
  } else if rev < T::revision() {
    let payload_prev_rev: T::RevisionPrev = deserialize_revision(rev, bytes)?;
    T::migrate_from_revision(payload_prev_rev)
      .ok_or_else(|| Box::new(
          bincode::ErrorKind::Custom("Failed to migrate revision".to_string())
      ))
  } else {
    Err(Box::new(
        bincode::ErrorKind::Custom("Attempted to migrate a future revision".to_string())
    ))
  }
}
