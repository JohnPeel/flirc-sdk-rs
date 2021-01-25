use std::{env, path::PathBuf};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn get_lib_dir() -> Result<&'static str> {
    if cfg!(all(target_os = "macos", target_arch = "x86_64")) {
        Ok("sdk/libs/Darwin_x86_64")
    } else if cfg!(all(target_os = "linux", target_arch = "x86_64")) {
        Ok("sdk/libs/Linux_x86_64")
    } else if cfg!(all(target_os = "linux", target_arch = "x86")) {
        Ok("sdk/libs/Linux_i686")
    } else if cfg!(all(target_os = "linux", target_arch = "arm")) {
        if cfg!(feature = "armv6l") {
            Ok("sdk/libs/Linux_armv6l")
        } else if cfg!(feature = "armv7l") {
            Ok("sdk/libs/Linux_armv7l")
        } else {
            Err("Please specify an arm version using feature armv6l or armv7l.".into())
        }
    } else if cfg!(target_os = "windows") {
        Ok("sdk/libs/Win")
    } else {
        Err("Unsupported target.".into())
    }
}

fn get_link_lib() -> &'static str {
    if cfg!(feature = "dynamic") {
        "dynamic=flirc"
    } else if cfg!(feature = "static") {
        "static=flirc"
    } else {
        "flirc"
    }
}

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=sdk/libs/flirc/flirc.h");
    println!("cargo:rustc-link-search={}", get_lib_dir()?);
    println!("cargo:rustc-link-lib={}", get_link_lib());

    let mut builder = bindgen::Builder::default()
        .header("sdk/libs/flirc/flirc.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .clang_arg("-Isdk/cli/include")
        .whitelist_recursively(false)
        .whitelist_type("flirc_.*")
        .whitelist_type("usb_iface_type")
        .whitelist_type("error_type")
        .whitelist_type("sensitivity")
        .whitelist_type("fl_.*")
        .whitelist_type("size_t")
        .whitelist_type("ll_.*")
        .whitelist_type("hl_.*")
        .whitelist_type("list_.*")
        .whitelist_type("hlist_.*")
        .whitelist_var("BOOTLOADER")
        .whitelist_var("FIRMWARE_FLIRC.*")
        .whitelist_var("FIRMWARE")
        .whitelist_var("FL_.*")
        .whitelist_var("MAX_TIMEOUT")
        .whitelist_var("RM_.*")
        .whitelist_var("FUNK_SUCCESS")
        .whitelist_var("ERR_.*")
        .whitelist_var("list_.*")
        .whitelist_var("hlist_.*")
        .whitelist_function("fl_.*")
        .whitelist_function("strerr")
        .whitelist_function("delay_ms")
        .whitelist_function("list_.*")
        .whitelist_function("hlist_.*")
        .whitelist_function("__list_.*")
        .whitelist_function("__hlist_.*");

    if cfg!(target_os = "windows") {
        builder = builder.clang_arg("-DWINDOWS");
    }

    let bindings = builder
        .generate()
        .map_err(|_| "Unable to generate bindings.rs")?;
    
    let out_path = PathBuf::from(env::var("OUT_DIR")?);
    bindings.write_to_file(out_path.join("bindings.rs"))?;

    Ok(())
}
