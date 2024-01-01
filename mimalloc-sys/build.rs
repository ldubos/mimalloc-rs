use std::env;
use std::path::PathBuf;

fn main() {
    let mut path: PathBuf = PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap());
    path.push("mimalloc");

    let mut build = cc::Build::new();

    build.include(path.join("include"));
    build.include(path.join("src"));
    build.file(path.join("src").join("static.c"));
    build.opt_level(3);

    if cfg!(any(target_family = "unix", target_os = "haiku")) {
        build.flag_if_supported("-ftls-model=initial-exec");
    }

    build.define("MI_DEBUG", "0");

    if build.get_compiler().is_like_msvc() {
        build.cpp(true);
    }

    build.compile("mimalloc");
}
