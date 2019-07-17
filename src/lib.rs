extern crate rand;
#[macro_use]
extern crate nom;

pub mod types;
pub use types::*;
pub mod util;
pub mod dice;
pub mod space;
pub mod damage;
pub mod basetraits;
pub mod action;
pub mod creature;
pub mod combat;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
