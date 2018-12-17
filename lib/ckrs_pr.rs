#![feature(asm)]
extern crate paste;

#[macro_use]
extern crate cfg_if;


pub mod ckrs_pr_x86;


#[macro_use]
#[cfg(not(CKRS_USE_CC_BUILTINS))]
cfg_if! {

    if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {

        use ckrs_pr_x86::*;
        macro_rules! if_rmo { ($($x:tt)*) => () }
        macro_rules! if_pso { ($($x:tt)*) => () }
        macro_rules! if_tso { ($($x:tt)*) => ($($x)*) }
    } else if #[cfg(target_arch = "powerpc64")] {
    use ckrs_pr_ppc64::*;
        macro_rules! if_rmo { ($($x:tt)*) => ($($x)*) }
        macro_rules! if_pso { ($($x:tt)*) => () }
        macro_rules! if_tso { ($($x:tt)*) => () }
    }
}

pub fn ckrs_pr_barrier() {
    unsafe {
        asm!("" ::: "memory" : "volatile")
    }
}


macro_rules! ckrs_pr_fence_emit {
    ($t:ident) => (
        paste::item! {
            #[inline(always)]
            pub fn [<ckrs_pr_fence_ $t>] () {
                [<ckrs_pr_fence_strict_ $t>]();
            }
        }
    );
}

macro_rules! ckrs_pr_fence_noop {
    ($t:ident) => (
        #[inline(always)]
        paste::item! {
            pub fn [<ckrs_pr_fence_ $t>] () {
                ckrs_pr_barrier();
            }
        }
    );
}

if_rmo! {
    ckrs_pr_fence_emit!(atomic);
    ckrs_pr_fence_emit!(atomic_load);
    ckrs_pr_fence_emit!(atomic_store);
    ckrs_pr_fence_emit!(store_atomic);
    ckrs_pr_fence_emit!(load_atomic);
    ckrs_pr_fence_emit!(load_store);
    ckrs_pr_fence_emit!(store_load);
    ckrs_pr_fence_emit!(load);
    ckrs_pr_fence_emit!(store);
    ckrs_pr_fence_emit!(memory);
    ckrs_pr_fence_emit!(acquire);
    ckrs_pr_fence_emit!(release);
    ckrs_pr_fence_emit!(acqrel);
    ckrs_pr_fence_emit!(lock);
    ckrs_pr_fence_emit!(unlock);
}

if_pso! {
    ckrs_pr_fence_emit!(atomic);
    ckrs_pr_fence_noop!(atomic_load);
    ckrs_pr_fence_emit!(atomic_store);
    ckrs_pr_fence_emit!(store_atomic);
    ckrs_pr_fence_noop!(load_atomic);
    ckrs_pr_fence_emit!(load_store);
    ckrs_pr_fence_emit!(store_load);
    ckrs_pr_fence_noop!(load);
    ckrs_pr_fence_emit!(store);
    ckrs_pr_fence_emit!(memory);
    ckrs_pr_fence_emit!(acquire);
    ckrs_pr_fence_emit!(release);
    ckrs_pr_fence_emit!(acqrel);
    ckrs_pr_fence_emit!(lock);
    ckrs_pr_fence_emit!(unlock);
}

if_tso! {
    ckrs_pr_fence_noop!(atomic);
    ckrs_pr_fence_noop!(atomic_load);
    ckrs_pr_fence_noop!(atomic_store);
    ckrs_pr_fence_noop!(store_atomic);
    ckrs_pr_fence_noop!(load_atomic);
    ckrs_pr_fence_noop!(load_store);
    ckrs_pr_fence_emit!(store_load);
    ckrs_pr_fence_noop!(load);
    ckrs_pr_fence_noop!(store);
    ckrs_pr_fence_emit!(memory);
    ckrs_pr_fence_noop!(acquire);
    ckrs_pr_fence_noop!(release);
    ckrs_pr_fence_noop!(acqrel);
    ckrs_pr_fence_noop!(lock);
    ckrs_pr_fence_noop!(unlock);
}



