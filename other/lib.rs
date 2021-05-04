pub fn dispatch(
    f: unsafe extern "C" fn(u32) -> u32,
    i: u32
) -> u32 {
    unsafe { f(i) }
}