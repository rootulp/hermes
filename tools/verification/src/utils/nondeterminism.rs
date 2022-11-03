use crate::types::aliases::Natural;

#[cfg(kani)]
pub fn any_bool() -> bool {
    kani::any()
}

#[cfg(not(kani))]
pub fn any_bool() -> bool {
    unimplemented!()
}

#[cfg(kani)]
pub fn any_natural() -> Natural {
    kani::any()
}

#[cfg(not(kani))]
pub fn any_natural() -> Natural {
    unimplemented!()
}

#[cfg(kani)]
pub fn assume(cond: bool) {
    kani::assume(cond)
}

#[cfg(not(kani))]
pub fn assume(_cond: bool) {
    unimplemented!()
}
