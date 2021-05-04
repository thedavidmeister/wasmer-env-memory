use other::dispatch as other_dispatch;

extern "C" {
    pub fn __read_memory(ptr: u32) -> u32;
}

fn dispatch(
    f: unsafe extern "C" fn(u32) -> u32,
    i: u32
) -> u32 {
    unsafe { f(i) }
}

#[no_mangle]
pub extern "C" fn read_memory() {
    let _: u32 = dispatch(__read_memory, 1);
    let _: u32 = other_dispatch(__read_memory, 2);
}