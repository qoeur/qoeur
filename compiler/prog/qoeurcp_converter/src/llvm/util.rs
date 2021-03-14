use llvm_sys::core::{LLVMGetTypeKind, LLVMTypeOf};
use llvm_sys::prelude::{LLVMTypeRef, LLVMValueRef};
use llvm_sys::LLVMTypeKind;

pub macro cstring($s:expr) {
  format!("{}{}", $s, "\0").as_ptr() as *const i8
}

pub fn type_of(val: LLVMValueRef) -> LLVMTypeRef {
  unsafe { LLVMTypeOf(val) }
}

pub fn get_type_kind(ty: LLVMTypeRef) -> LLVMTypeKind {
  unsafe { LLVMGetTypeKind(ty) }
}
