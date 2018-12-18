use ckrs_pr;
use ckrs_stack::*;

const CKRS_EPOCH_LENGTH: usize = 4;

const CKRS_EPOCH_SENSE: usize = 2;


type CkrsEpochCb = fn(&mut CkrsEpochEntry);

pub struct CkrsEpochEntry {
    function: CkrsEpochCb,
}
pub struct CkrsEpochSection {
    bucket: u32,
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

impl CkrsEpochRecord {
    fn _ckrs_epoch_addref(&mut self, _section: &mut CkrsEpochSection) {

    }
    fn _ckrs_epoch_delref(&mut self, _section: &mut CkrsEpochSection) -> bool {
        return false;
    }
    pub fn ckrs_epoch_begin(&mut self, _section: Option<&mut CkrsEpochSection>) {

        if let Some(mut s) = _section {
            self._ckrs_epoch_addref(s);
        }
    }

}
