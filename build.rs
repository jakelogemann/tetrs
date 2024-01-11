fn main() {
  println!("cargo:rerun-if-changed=build.rs");
  build_info_build::build_script();
  // let out_dir = std::env::var("OUT_DIR").unwrap();
  // println!("cargo:warning=Hello, world!");
  // println!("cargo:rustc-env=FOO=bar");
}
