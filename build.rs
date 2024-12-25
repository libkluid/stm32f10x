fn main() {
    const LINKER: &'static str = "linker/stm32f103rbt6.ld";

    println!("cargo:return-if-changed=build.rs");
    println!("cargo:return-if-changed={linker}", linker = LINKER);
    println!("cargo:rustc-link-arg=-T{linker}", linker = LINKER);
}
