use ckrs_pr;
use ckrs_stack::*;

const CKRS_EPOCH_LENGTH: usize = 4;

const CKRS_EPOCH_SENSE: usize = 2;


type CkrsEpochCb = fn(&mut CkrsEpochEntry);

pub struct CkrsEpochEntry {
    function: CkrsEpochCb,
}

pub struct CkrsEpochRef {
    epoch: u32,
    count: u32,
}

pub struct CkrsEpochRecord {
    record_next: CkrsStackEntry,
    global: *mut CkrsEpoch,
    epoch: u32,
    active: u32,
    bucket: [CkrsEpochRef; CKRS_EPOCH_SENSE],
    n_pending: u32,
    n_peak: u32,
    n_dispatch: u32,
    pending: [CkrsStack; CKRS_EPOCH_LENGTH]
}

pub struct CkrsEpoch {
    epoch: u32,
    n_free: u32,
}


