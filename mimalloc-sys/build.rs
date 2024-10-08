use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let is_debug = env::var("PROFILE")? == "debug";
    let os: String = {
        let target = env::var("TARGET")?;
        let parts = target.split("-").collect::<Vec<&str>>();

        if parts.len() >= 3 {
            parts[2].to_string()
        } else {
            "unknown".to_string()
        }
    };

    let mut path: PathBuf = PathBuf::from(&env::var("CARGO_MANIFEST_DIR")?);
    path.push("mimalloc");

    let mut build = cc::Build::new();

    build.include(path.join("include"));
    build.include(path.join("src"));
    build.file(path.join("src").join("static.c"));
    build.opt_level(3);

    if os == "linux" || os == "macos" {
        build.flag_if_supported("-ftls-model=initial-exec");
    }

    build.define("MI_DEBUG", "0");

    if !is_debug {
        build.define("MI_SECURE", "4");
    }

    if os == "linux" {
        build.define("MIMALLOC_ARENA_EAGER_COMMIT", "2");
    } else {
        build.define("MIMALLOC_ARENA_EAGER_COMMIT", "1");
    }

    build.define("MIMALLOC_ALLOW_LARGE_OS_PAGES", "0");
    build.define("MIMALLOC_RESERVE_HUGE_OS_PAGES", "2");
    build.define("MIMALLOC_PURGE_DELAY", "2500");

    if build.get_compiler().is_like_msvc() {
        build.cpp(true);
    }

    build.compile("mimalloc");

    Ok(())
}
