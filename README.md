# syscall-intercept-rs

## Requirements

Install dependencies in Ubuntu.

```sh
sudo apt install cmake libcapstone-dev
```

lib of syscall_intercept also are required.[FIXME]
github addr of syscall_intercept 
```
https://github.com/pmem/syscall_intercept
```

Add the following lines to your Cargo.toml:

```
[dependencies]
libsyscall-intercept = "0.1.1"
```

## Usage 
lib.rs in your code
```
use std::cell::Cell;
#[macro_use]
extern crate ctor;
use libsyscall_intercept::{set_hook_fn,InterceptResult};


#[ctor]
fn init_preload() {
    unsafe { set_hook_fn(hook) };
}

extern "C" fn hook(
    num: i64,
    _a0: i64,
    _a1: i64,
    _a2: i64,
    _a3: i64,
    _a4: i64,
    _a5: i64,
    result: *mut i64,
) -> i32 {
    // detect and avoid recursive interception
    let _guard = match InterceptGuard::try_lock() {
        Some(g) => g,
        None => return InterceptResult::Forward as i32,
    };
    if num == libc::SYS_getdents64 || num == libc::SYS_getdents {
        unsafe {
            *result =  -libc::ENOTSUP as i64;
        } 
        return InterceptResult::Hook as i32;
    }
    InterceptResult::Forward as i32
}

thread_local! {
    static INTERCEPTED: Cell<bool> = Cell::new(false);
}

struct InterceptGuard;

impl InterceptGuard {
    fn try_lock() -> Option<Self> {
        INTERCEPTED.with(|x| {
            if x.get() {
                None
            } else {
                x.set(true);
                Some(InterceptGuard)
            }
        })
    }
}

impl Drop for InterceptGuard {
    fn drop(&mut self) {
        INTERCEPTED.with(|x| x.set(false));
    }
}
```

Cargo.toml as below
```
[package]
name = "my-lib-test"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html


[dependencies]
ctor = "0.2.0"
libc = "0.2"
libsyscall-intercept = "0.1.1"


[lib]
name = "rintercept"
crate-type = ["dylib"]

```

and then run command : 
```
cargo build
LD_PRELOAD=./target/debug/librintercept.so ls
```
will see command ***ls is not support***.


## [blog](https://blog.csdn.net/gzhu_flyingbird/article/details/131882648?spm=1001.2014.3001.5502)
