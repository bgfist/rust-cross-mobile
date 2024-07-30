fn main() {
    if std::env::var("CARGO_CFG_TARGET_ENV").unwrap() == "ohos".to_string() {
        println!("cargo::rustc-link-lib=dylib=ace_napi.z");
    }

    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "macos".to_string() {
        napi_build::setup();
    }
}
