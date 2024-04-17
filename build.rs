fn main() {
    cxx_build::bridge("src/lib.rs")
        .file("src/interface.cc")
        .std("c++17")
        .compile("voronota-rs");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/interface.cc");
    println!("cargo:rerun-if-changed=src/interface.h");
    println!("cargo:rerun-if-changed=src/voronotalt.h");
}
