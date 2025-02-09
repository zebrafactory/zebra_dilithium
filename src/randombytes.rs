use rand;
use rand::prelude::*;

// FIXME: This slicing inside x feels weird and risky
pub fn randombytes(x: &mut [u8], len: usize) {
  rand::rng().fill_bytes(&mut x[..len])
}
