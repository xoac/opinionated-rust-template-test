//! # Summary
//!
//!
//!
//! # Overview
//! TODO: Add Overview or remove section
//!
//! # Examples
//! TODO: Add examples or remove section

#![deny(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

/// return 2
pub fn return2() -> i32 {
    2_i32
}

#[cfg(test)]
mod tests {
    use crate::return2;

    #[test]
    fn it_works() {
        let x = return2();

        assert_eq!(2 + x, 4);
    }
}
