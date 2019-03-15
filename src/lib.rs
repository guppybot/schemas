extern crate bincode;
extern crate byteorder;
extern crate serde;
#[macro_use] extern crate serde_derive;

pub use crate::v1 as wire_protocol;

use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};
use serde::{Deserialize, Serialize};

use std::io::{Cursor};

pub mod v1;

pub trait Revise<'a> {
  type Previous: Revise<'a> + Deserialize<'a>;

  fn revision() -> u32;

  fn migrate_from_previous(_prev_rev: Self::Previous) -> Option<Self> where Self: Sized {
    None
  }
}

#[derive(Serialize, Deserialize)]
pub enum NoPrevious {}

impl<'a> Revise<'a> for NoPrevious {
  type Previous = NoPrevious;

  fn revision() -> u32 {
    0
  }
}

pub fn _deserialize_revision<'a, T>(rev: u32, bytes: &'a [u8]) -> bincode::Result<T>
where T: Revise<'a> + Deserialize<'a> {
  if rev == T::revision() {
    bincode::deserialize(bytes)
  } else if rev < T::revision() {
    let payload_prev_rev: T::Previous = _deserialize_revision(rev, bytes)?;
    T::migrate_from_previous(payload_prev_rev)
      .ok_or_else(|| Box::new(
          bincode::ErrorKind::Custom("Failed to migrate revision".to_string())
      ))
  } else {
    Err(Box::new(
        bincode::ErrorKind::Custom("Attempted to migrate a future revision".to_string())
    ))
  }
}

pub fn deserialize_revision<'a, T>(bytes: &'a [u8]) -> bincode::Result<T>
where T: Revise<'a> + Deserialize<'a> {
  let rev = {
    let mut reader = Cursor::new(bytes);
    let rev = match reader.read_u32::<LittleEndian>() {
      Err(_) => return Err(Box::new(
          bincode::ErrorKind::Custom("Failed to read revision number".to_string())
      )),
      Ok(r) => r,
    };
    rev
  };
  _deserialize_revision(rev, &bytes[4 .. ])
}

pub fn serialize_revision<'a, T>(value: &'a T) -> bincode::Result<Vec<u8>>
where T: Revise<'a> + Serialize {
  let mut buf = Vec::with_capacity(4);
  match buf.write_u32::<LittleEndian>(T::revision()) {
    Err(_) => return Err(Box::new(
        bincode::ErrorKind::Custom("Failed to write revision number".to_string())
    )),
    Ok(_) => {}
  }
  bincode::serialize_into(&mut buf, value).map(move |_| buf)
}
