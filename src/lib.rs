#[macro_use] extern crate group_attr;

group_attr! {
    #[cfg(unix)]

    extern crate libc;

    mod wrapper;
}

