use std::convert::TryInto;

/// Convert a signed isize into the nearest usize (rounding negatives to zero).
pub fn clamp_isize(i: isize) -> usize {
    if i.is_negative() {
        0usize
    } else {
        i.try_into().unwrap()  // Shouldn't fail
    }
}
