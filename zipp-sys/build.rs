use cmake::Config;

fn main() {
    let mut dst = Config::new("libzip")
//        .define("BUILD_SHARED_LIBS", "OFF")
        .build();
    dst.push("lib");
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=dylib=zip");
}
