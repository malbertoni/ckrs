macro_rules! ckrs_pr_fence_strict_emit {
    ($t:ident, $i:tt) => (
        paste::item! {
            #[inline(always)]
            pub fn [<ckrs_pr_fence_strict_ $t>] () {
                unsafe {
                    asm!($i ::: "memory" : "volatile")
                }
            }
        }
    );
}

ckrs_pr_fence_strict_emit!(load, "lfence");
ckrs_pr_fence_strict_emit!(load_store, "lfence");
ckrs_pr_fence_strict_emit!(store, "sfence");
ckrs_pr_fence_strict_emit!(store_load, "mfence");
ckrs_pr_fence_strict_emit!(memory, "mfence");
