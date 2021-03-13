#![feature(box_patterns)]
#![feature(box_syntax)]
#![feature(decl_macro)]
#![recursion_limit = "256"]

mod cranelift;
mod llvm;

pub use BackendKind::*;

pub enum BackendKind {
  CraneLift,
  Llvm,
}

pub fn compile(mode: BackendKind) -> Result<(), String> {
  match mode {
    CraneLift => cranelift::compile(),
    Llvm => llvm::compile(),
  }
}
