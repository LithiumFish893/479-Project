use std::{
    env,
    path::Path,
    process::{exit, Command},
};

const AFL_URL: &str = "https://github.com/AFLplusplus/AFLplusplus";

fn main() {
    if cfg!(windows) {
        println!("cargo:warning=No support for windows yet.");
        exit(0);
    }

    let cwd = env::current_dir().unwrap().to_string_lossy().to_string();
                let xpdf_dir = format!("{}/xpdf", cwd);

                // clean doesn't know about the install directory we use to build, remove it as well
                Command::new("rm")
                    .arg("-r")
                    .arg("-v")
                    .arg("-f")
                    .arg(&format!("{}/install", xpdf_dir))
                    .current_dir(xpdf_dir.clone())
                    .status()
                    .expect("Couldn't clean xpdf's install directory");

                // export LLVM_CONFIG=llvm-config-11
                env::set_var("LLVM_CONFIG", "llvm-config-11");

                // configure with afl-clang-fast and set install directory to ./xpdf/install
                Command::new("./configure")
                    .arg(&format!("--prefix={}/install", xpdf_dir))
                    .env("CC", "/usr/local/bin/afl-clang-fast")
                    .env("CXX", "/usr/local/bin/afl-clang-fast++")
                    .current_dir(xpdf_dir.clone())
                    .status()
                    .expect("Couldn't configure xpdf to build using afl-clang-fast");

                // make && make install
                Command::new("make")
                    .current_dir(xpdf_dir.clone())
                    .status()
                    .expect("Couldn't make xpdf");

                Command::new("make")
                    .arg("install")
                    .current_dir(xpdf_dir)
                    .status()
                    .expect("Couldn't install xpdf");

    let afl = format!("{}/AFLplusplus", &cwd);
    let afl_gcc = format!("{}/AFLplusplus/afl-cc", &cwd);

    let afl_path = Path::new(&afl);
    let afl_gcc_path = Path::new(&afl_gcc);

    if !afl_path.is_dir() {
        println!("cargo:warning=AFL++ not found, downloading...");
        Command::new("git")
            .arg("clone")
            .arg(AFL_URL)
            .status()
            .unwrap();
    }

    if !afl_gcc_path.is_file() {
        Command::new("make")
            .arg("all")
            .current_dir(&afl_path)
            .status()
            .unwrap();
    }

    Command::new(afl_gcc_path)
        .args(&["src/program.c", "-o"])
        .arg(&format!("{}/target/release/program", &cwd))
        .status()
        .unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/");
}
