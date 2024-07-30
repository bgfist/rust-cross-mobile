/// import the preludes
use napi::bindgen_prelude::*;
use napi::module_init;
use napi_derive::napi;

use crate::QuestionDb;

/// module registration is done by the runtime, no need to explicitly do it now.
#[napi]
fn fibonacci(n: u32) -> u32 {
    match n {
        1 | 2 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

#[napi]
fn get_cwd<T: Fn(String) -> Result<()>>(callback: T) {
    // 获取当前目录下的文件列表
    let entries = std::fs::read_dir(".").unwrap();

    let res = entries
        .map(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();
            path.display().to_string()
        })
        .collect::<Vec<String>>()
        .join("\n");

    callback(res).unwrap();
}

#[cfg(target_env = "ohos")]
#[module_init]
fn napi_register_module_v1_ohos_init() {
    let mut modules = napi::sys::napi_module {
        nm_version: 1,
        nm_filename: std::ptr::null_mut(),
        nm_flags: 0,
        nm_modname: "jk_core_question".as_ptr().cast(),
        nm_priv: std::ptr::null_mut(),
        nm_register_func: Some(napi_register_module_v1),
        reserved: [std::ptr::null_mut(); 4],
    };
    unsafe {
        napi::sys::napi_module_register(&mut modules);
    }
}
