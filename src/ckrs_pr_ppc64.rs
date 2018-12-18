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


ckrs_pr_fence_strict_emit!(atomic, "lwsync");
ckrs_pr_fence_strict_emit!(atomic_store, "lwsync");
ckrs_pr_fence_strict_emit!(atomic_load, "sync");
ckrs_pr_fence_strict_emit!(store_atomic, "lwsync");
ckrs_pr_fence_strict_emit!(load_atomic, "lwsync");
ckrs_pr_fence_strict_emit!(store, "lwsync");
ckrs_pr_fence_strict_emit!(store_load, "sync");
ckrs_pr_fence_strict_emit!(load, "lwsync");
ckrs_pr_fence_strict_emit!(load_store, "lwsync");
ckrs_pr_fence_strict_emit!(memory, "sync");
ckrs_pr_fence_strict_emit!(acquire, "lwsync");
ckrs_pr_fence_strict_emit!(release, "lwsync");
ckrs_pr_fence_strict_emit!(acqrel, "lwsync");
ckrs_pr_fence_strict_emit!(lock, "lwsync");
ckrs_pr_fence_strict_emit!(unlock, "lwsync");
