#![feature(asm)]
extern crate paste;

#[macro_use]
extern crate cfg_if;

cfg_if! {
    if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
        pub mod ckrs_pr_x86;
    } else if #[cfg(target_arch = "powerpc64")] {
        pub mod ckrs_pr_ppc64;
    }
}
mod ckrs_stack;
pub mod ckrs_pr;
pub mod ckrs_epoch;



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
