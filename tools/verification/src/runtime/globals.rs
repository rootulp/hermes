static mut GLOBAL_STATE_CHANGED: bool = false;

pub fn is_global_state_modified() -> bool {
    unsafe { GLOBAL_STATE_CHANGED }
}

pub fn set_global_state_modified() {
    unsafe { GLOBAL_STATE_CHANGED = true }
}

pub fn clear_global_state_modified() {
    unsafe { GLOBAL_STATE_CHANGED = false }
}
