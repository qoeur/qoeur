#![feature(box_patterns)]
#![feature(box_syntax)]
#![feature(decl_macro)]
#![recursion_limit = "256"]

mod cranelift;

pub use self::cranelift::Jit;

pub fn compile() -> Result<(), String> {
  Ok(())
}
