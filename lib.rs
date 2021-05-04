use wasmer::*;

#[derive(Clone, Default, WasmerEnv)]
pub struct Env {
    #[wasmer(export)]
    memory: LazyInit<Memory>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            memory: LazyInit::new()
        }
    }
}

pub fn read_memory(env: &Env, guest_ptr: u32) -> u32 {
    dbg!(env.memory_ref());
    dbg!(guest_ptr);
    0
}
pub const WASM: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/wasm32-unknown-unknown/release/wasm.wasm"
));

#[test]
fn test() {
    let engine = JIT::new(wasmer_compiler_singlepass::Singlepass::new()).engine();
    let store = Store::new(&engine);
    let env = Env::new();
    let module = Module::new(&store, WASM).unwrap();
    let imports: ImportObject = imports! {
        "env" => {
            "__read_memory" => Function::new_native_with_env(
                &store,
                env.clone(),
                read_memory
            ),
        }
    };
    let instance = Instance::new(&module, &imports).unwrap();

    instance.exports.get_function("read_memory").unwrap().call(&[]).unwrap();
}