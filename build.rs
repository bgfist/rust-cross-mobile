fn main() {
    if std::env::var("CARGO_CFG_TARGET_ENV").unwrap() == "ohos".to_string() {
        println!("cargo::rustc-link-lib=dylib=ace_napi.z");
        println!("cargo::rustc-cfg=ffi_napi");
    }

    let os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    if os == "android".to_string() || os == "ios".to_string() {
        println!("cargo::rustc-cfg=ffi_uniffi");
    }
}
