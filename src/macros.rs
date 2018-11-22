/// Retrieve a WebAssembly function given a Instance and a FuncIndex
/// Example:
/// let func: fn(i32) -> i32 = get_instance_function!(instance, func_index);
#[macro_export]
macro_rules! get_instance_function {
    ($instance:expr, $func_index:expr) => {{
        use crate::sighandler::install_sighandler;
        use std::mem;

        unsafe {
            install_sighandler();
        };
        let func_addr = $instance.get_function_pointer($func_index);
        unsafe { mem::transmute(func_addr) }
    }};
}

#[macro_export]
macro_rules! include_wast2wasm_bytes {
    ($x:expr) => {{
        use wabt::wat2wasm;
        const WAST_BYTES: &[u8] = include_bytes!($x);
        wat2wasm(WAST_BYTES.to_vec()).expect(&format!("Can't convert {} file to wasm", $x))
    }};
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug {
    ($fmt:expr) => (println!(concat!("Wasmer::", $fmt)));
    ($fmt:expr, $($arg:tt)*) => (println!(concat!("Wasmer::", $fmt, "\n"), $($arg)*));
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug {
    ($fmt:expr) => {};
    ($fmt:expr, $($arg:tt)*) => {};
}


// A macro to retrieve variadic arguments given a varargs offset
#[macro_export]
macro_rules! vararg {
    ($type:ident, $instance:ident, $varargs:ident) => (
        unsafe {
            use std::ptr;
            let ptr = $instance.memory_offset_addr(0, $varargs as usize);
            let ret = ptr::read(ptr as *const $type);
            let $varargs = $varargs + 4; // NOTE: 32-bit offsets
            ret
        };
    )
}
