pub struct CkrsStackEntry {
    next: *mut CkrsStackEntry,
}

pub struct CkrsStack {
    head: *mut CkrsStackEntry,
    generation: *mut char,
}
