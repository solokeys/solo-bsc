fn main() {
    println!("cargo:rerun-if-env-changed=BOARD");
    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=build.rs");
}

