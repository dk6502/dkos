fn main() {
  cc::Build::new()
    .file("src/arch/x86_64/interrupt_stub.s")
    .compile("interrupt_stub");
  let arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
  // Tell cargo to pass the linker script to the linker..
  println!("cargo:rustc-link-arg=-Tkernel/linker-{arch}.ld");
  // ..and to re-run if it changes.
  println!("cargo:rerun-if-changed=kernel/linker-{arch}.ld");
}
