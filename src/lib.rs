include!("../target/bindings.rs");

pub type HookFn = extern "C" fn(
    num: i64,
    a0: i64,
    a1: i64,
    a2: i64,
    a3: i64,
    a4: i64,
    a5: i64,
    result: *mut i64,
) -> i32;


pub unsafe fn set_hook_fn(f: HookFn) {
    intercept_hook_point = Some(f);
}

pub unsafe fn unset_hook_fn() {
    intercept_hook_point = None;
}
#[repr(i32)]
pub enum InterceptResult {
    Hook = 0,
    Forward = 1,
}
