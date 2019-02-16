extern crate bincode;
extern crate serde;
#[macro_use] extern crate serde_derive;

use serde::{Deserialize};

pub mod wire_protocol;

pub trait Versioned<'a> {
  type VersionPrev: Versioned<'a> + Deserialize<'a>;
  type RevisionPrev: Versioned<'a> + Deserialize<'a>;

  fn version() -> u32;
  fn revision() -> u32;

  fn migrate_from_version(_prev_ver: Self::VersionPrev) -> Option<Self> where Self: Sized {
    None
  }

  fn migrate_from_revision(_prev_rev: Self::RevisionPrev) -> Option<Self> where Self: Sized {
    None
  }
}

#[derive(Serialize, Deserialize)]
pub enum NoPrev {}

impl<'a> Versioned<'a> for NoPrev {
  type VersionPrev = NoPrev;
  type RevisionPrev = NoPrev;

  fn version() -> u32 {
    0
  }

  fn revision() -> u32 {
    0
  }
}

pub fn versioned_deserialize<'a, T>(ver: u32, rev: u32, bytes: &'a [u8]) -> bincode::Result<T>
where T: Versioned<'a> + Deserialize<'a> {
  if ver == T::version() && rev == T::revision() {
    bincode::deserialize(bytes)
  } else if ver == T::version() && rev < T::revision() {
    let payload_prev_rev: T::RevisionPrev = versioned_deserialize(ver, rev, bytes)?;
    T::migrate_from_revision(payload_prev_rev)
      .ok_or_else(|| Box::new(
          bincode::ErrorKind::Custom("Failed revision migration".to_string())
      ))
  } else if ver < T::version() {
    let payload_prev_ver: T::VersionPrev = versioned_deserialize(ver, rev, bytes)?;
    T::migrate_from_version(payload_prev_ver)
      .ok_or_else(|| Box::new(
          bincode::ErrorKind::Custom("Failed version migration".to_string())
      ))
  } else {
    Err(Box::new(
        bincode::ErrorKind::Custom("Failed migration".to_string())
    ))
  }
}
