use std::env;
use std::path::PathBuf;

#[cfg(feature = "static")]
use std::process::Command;

fn main() {
  // TODO: cross compile
  // TODO: use variables for paths, etc.
  // TODO: make this stuff more portable (e.g. no `Command`)
  #[allow(unused_variables)] // for some reason rust doesn't recognize this is being used.
  let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

  #[cfg(feature = "static")]
  {
    Command::new("cp")
      .args(&[
        "-rf",
        "vendor/c-core",
        &format!("{}", out_path.join("c-core").display()),
      ])
      .status()
      .unwrap();
    Command::new("make")
      .current_dir(out_path.join("c-core"))
      .args(&["-f", "posix.mk"])
      .status()
      .unwrap();
    println!("cargo:rustc-link-search={}", out_path.join("c-core/posix").display());
  }

  #[cfg(feature = "callback")]
  {
    #[cfg(feature = "static")]
    {
      Command::new("cp")
        .args(&[
          &format!("{}", out_path.join("c-core/posix/pubnub_callback.a").display()),
          &format!("{}", out_path.join("c-core/posix/libpubnub_callback.a").display()),
        ])
        .status()
        .unwrap();
      println!("cargo:rustc-link-lib=static=pubnub_callback");
    }

    let callback_bindings = bindgen::Builder::default()
    .header("vendor/c-core/posix/pubnub_callback.h")
    .clang_arg("-Ivendor/c-core")
    .clang_arg("-Ivendor/c-core/posix")
    .clang_arg("-DPUBNUB_CALLBACK_API=1")
    .blacklist_function("strtold") // u128 is not ffi-safe
    .generate()
    .expect("Unable to generate callback bindings");

    callback_bindings
      .write_to_file(out_path.join("callback.rs"))
      .expect("Couldn't write bindings");

    #[cfg(feature = "dynamic")]
    println!("cargo:rustc-link-lib=pubnub_callback");
  }

  #[cfg(feature = "sync")]
  {
    #[cfg(feature = "static")]
    {
      Command::new("cp")
        .args(&[
          &format!("{}", out_path.join("c-core/posix/pubnub_sync.a").display()),
          &format!("{}", out_path.join("c-core/posix/libpubnub_sync.a").display()),
        ])
        .status()
        .unwrap();
      println!("cargo:rustc-link-lib=static=pubnub_sync");
    }

    let sync_bindings = bindgen::Builder::default()
    .header("vendor/c-core/posix/pubnub_sync.h")
    .clang_arg("-Ivendor/c-core")
    .clang_arg("-Ivendor/c-core/posix")
    .clang_arg("-DPUBNUB_CALLBACK_API=0")
    .blacklist_function("strtold") // u128 is not ffi-safe
    .generate()
    .expect("Unable to generate sync bindings");

    sync_bindings
      .write_to_file(out_path.join("sync.rs"))
      .expect("Couldn't write bindings");

    #[cfg(feature = "dynamic")]
    println!("cargo:rustc-link-lib=pubnub_sync");
  }
}
