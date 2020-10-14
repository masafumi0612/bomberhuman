use crate::geometry::Point;
use crate::models::{Vector};

/// Optimized version of `Vec::retain`
///
/// We achieve better performance by renouncing to keep the original order of the `Vec`
pub fn fast_retain<T, F>(vec: &mut Vec<T>, mut f: F)
where
    F: FnMut(&T) -> bool,
{
    let mut i = 0;
    while i < vec.len() {
        if !f(&vec[i]) {
            vec.swap_remove(i);
        }

        i += 1;
    }
}

