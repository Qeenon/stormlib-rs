/// Check ffi function call result, propagates the error
macro_rules! unsafe_try_call {
  ($r:expr) => {
    #[allow(unused_unsafe)]
    unsafe {
      if !$r {
        return Err(crate::error::StormError::from(stormlib_sys::GetLastError()));
      }
    }
  };
}
