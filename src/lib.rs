#![feature(asm)]
extern crate paste;

#[macro_use]
extern crate cfg_if;

pub mod ckrs_pr;
pub mod ckrs_pr_x86;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
