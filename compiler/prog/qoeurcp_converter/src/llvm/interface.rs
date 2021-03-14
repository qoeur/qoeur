use super::util::cstring;

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::ffi::CString;
use std::os::raw::c_uint;
use std::ptr;

use llvm_sys::core::*;
use llvm_sys::prelude::*;
use llvm_sys::target::*;
use llvm_sys::target_machine::*;
use llvm_sys::transforms::pass_manager_builder::*;
use llvm_sys::*;

pub fn make_build_call(
  builder: LLVMBuilderRef,
  llvm_fun: LLVMValueRef,
  param_tys: &mut Vec<LLVMValueRef>,
) -> LLVMValueRef {
  unsafe {
    LLVMBuildCall(
      builder,
      llvm_fun,
      param_tys.as_mut_ptr(),
      param_tys.len() as u32,
      cstring!(""),
    )
  }
}

pub fn make_build_load(
  builder: LLVMBuilderRef,
  pointer_value: LLVMValueRef,
  name: &str,
) -> LLVMValueRef {
  unsafe { LLVMBuildLoad(builder, pointer_value, cstring!(name)) }
}

pub fn make_build_store(
  builder: LLVMBuilderRef,
  value: LLVMValueRef,
  pointer: LLVMValueRef,
) -> LLVMValueRef {
  unsafe { LLVMBuildStore(builder, value, pointer) }
}

pub fn make_build_binop_add_value(
  builder: LLVMBuilderRef,
  lhs: LLVMValueRef,
  rhs: LLVMValueRef,
) -> LLVMValueRef {
  unsafe {
    LLVMBuildAdd(builder, lhs, rhs, cstring!("addtmp"))
  }
}

pub fn make_build_binop_div_value(
  builder: LLVMBuilderRef,
  lhs: LLVMValueRef,
  rhs: LLVMValueRef,
) -> LLVMValueRef {
  unsafe {
    LLVMBuildUDiv(builder, lhs, rhs, cstring!("divtmp"))
  }
}

pub fn make_build_binop_mul_value(
  builder: LLVMBuilderRef,
  lhs: LLVMValueRef,
  rhs: LLVMValueRef,
) -> LLVMValueRef {
  unsafe {
    LLVMBuildMul(builder, lhs, rhs, cstring!("multmp"))
  }
}

pub fn make_build_binop_sub_value(
  builder: LLVMBuilderRef,
  lhs: LLVMValueRef,
  rhs: LLVMValueRef,
) -> LLVMValueRef {
  unsafe {
    LLVMBuildSub(builder, lhs, rhs, cstring!("subtmp"))
  }
}

pub fn make_build_ret(
  builder: LLVMBuilderRef,
  value: LLVMValueRef,
) -> LLVMValueRef {
  unsafe { LLVMBuildRet(builder, value) }
}

pub fn make_build_ret_void(
  builder: LLVMBuilderRef,
  value: LLVMValueRef,
) -> LLVMValueRef {
  unsafe { LLVMBuildRetVoid(builder) }
}

pub fn make_const_int(ty: LLVMTypeRef, int: &i64) -> LLVMValueRef {
  unsafe { LLVMConstInt(ty, *int as u64, 0) }
}

pub fn make_const_int_value(
  context: LLVMContextRef,
  int: &i64,
) -> LLVMValueRef {
  unsafe {
    let int_ty = LLVMInt64TypeInContext(context);
    LLVMConstInt(int_ty, *int as u64, 0)
  }
}

pub fn make_const_real(ty: LLVMTypeRef, val: &f64) -> LLVMValueRef {
  unsafe { LLVMConstReal(ty, *val) }
}

pub fn make_const_real_value(
  context: LLVMContextRef,
  real: &f64,
) -> LLVMValueRef {
  unsafe {
    let real_ty = LLVMFloatTypeInContext(context);
    LLVMConstReal(real_ty, *real)
  }
}

pub fn make_context_double_ty(context: LLVMContextRef) -> LLVMTypeRef {
  unsafe { LLVMDoubleTypeInContext(context) }
}

pub fn make_context_float_ty(context: LLVMContextRef) -> LLVMTypeRef {
  unsafe { LLVMFloatTypeInContext(context) }
}

pub fn make_context_int1_ty(context: LLVMContextRef) -> LLVMTypeRef {
  unsafe { LLVMInt1TypeInContext(context) }
}

pub fn make_context_int8_ty(context: LLVMContextRef) -> LLVMTypeRef {
  unsafe { LLVMInt8TypeInContext(context) }
}

pub fn make_context_int32_ty(context: LLVMContextRef) -> LLVMTypeRef {
  unsafe { LLVMInt32TypeInContext(context) }
}

pub fn make_context_int64_ty(context: LLVMContextRef) -> LLVMTypeRef {
  unsafe { LLVMInt64TypeInContext(context) }
}

pub fn make_context_struct_ty(
  context: LLVMContextRef,
  types: &mut Vec<LLVMTypeRef>,
) -> LLVMTypeRef {
  unsafe {
    LLVMStructTypeInContext(
      context,
      types.as_mut_ptr(),
      types.len() as c_uint,
      0,
    )
  }
}

pub fn make_context_void_ty(context: LLVMContextRef) -> LLVMTypeRef {
  unsafe { LLVMVoidTypeInContext(context) }
}
