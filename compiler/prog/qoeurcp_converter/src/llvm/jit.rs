use super::util::cstring;

use qoeurcp_tokenizer::ast::*;

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::ffi::CString;
use std::ptr;

use llvm_sys::core::*;
use llvm_sys::prelude::*;
use llvm_sys::target::*;
use llvm_sys::target_machine::*;
use llvm_sys::transforms::pass_manager_builder::*;
use llvm_sys::*;

pub struct Jit {
  pub context: LLVMContextRef,
  pub module: LLVMModuleRef,
  pub builder: LLVMBuilderRef,
  pub target: RefCell<LLVMTargetRef>,
  pub target_machine: RefCell<LLVMTargetMachineRef>,
  pub target_data: RefCell<LLVMTargetDataRef>,
}

impl Drop for Jit {
  fn drop(&mut self) {
    unsafe {
      LLVMDisposeBuilder(self.builder);
      LLVMDisposeModule(self.module);
      LLVMContextDispose(self.context);
    }
  }
}

impl Jit {
  pub fn new() -> Jit {
    unsafe {
      let context = LLVMContextCreate();
      let module = LLVMModuleCreateWithName(cstring!(""));
      let builder = LLVMCreateBuilderInContext(context);

      Self {
        context: context,
        module: module,
        builder: builder,
        target: RefCell::new(ptr::null_mut()),
        target_machine: RefCell::new(ptr::null_mut()),
        target_data: RefCell::new(ptr::null_mut()),
      }
    }
  }

  pub fn codegen(&mut self, stmts: Vec<Box<Stmt>>) {
    unsafe {
      let context = LLVMContextCreate();

      let module =
        LLVMModuleCreateWithName(b"basics\0".as_ptr() as *const _);

      let builder = LLVMCreateBuilderInContext(context);

      let int_type = LLVMInt64TypeInContext(context);
      let fun_ty = LLVMFunctionType(int_type, ptr::null_mut(), 0, 0);

      let fun =
        LLVMAddFunction(module, b"main\0".as_ptr() as *const _, fun_ty);

      let entry_name = CString::new("entry").unwrap();

      let bb =
        LLVMAppendBasicBlockInContext(context, fun, entry_name.as_ptr());

      LLVMPositionBuilderAtEnd(builder, bb);

      let mut names = HashMap::new();
      insert_allocations(context, builder, &mut names, &stmts);

      let int_type = LLVMInt64TypeInContext(context);
      let zero = LLVMConstInt(int_type, 0, 0);

      let mut ret_value = zero; // return value on empty program

      stmts.iter().for_each(|stmt| {
        ret_value =
          self.codegen_stmt(context, builder, fun, &mut names, stmt);
      });

      LLVMBuildRet(builder, ret_value);

      let out_file = CString::new("out/test.ll").unwrap();
      LLVMPrintModuleToFile(module, out_file.as_ptr(), ptr::null_mut());

      LLVMDisposeBuilder(builder);
      LLVMDisposeModule(module);
      LLVMContextDispose(context);
    }
  }

  fn codegen_binop_expr(
    &mut self,
    lhs: &Box<Expr>,
    op: &BinOpKind,
    rhs: &Box<Expr>,
  ) -> LLVMValueRef {
    match op {
      BinOpKind::Add => {
        let lhs_expr = self.codegen_expr_stmt(lhs);
        let rhs_expr = self.codegen_expr_stmt(rhs);

        make_codegen_binop_add_expr(self.builder, lhs_expr, rhs_expr)
      }
      _ => unimplemented!(),
    }
  }

  fn codegen_expr_stmt(&mut self, expr: &Box<Expr>) -> LLVMValueRef {
    match expr.kind() {
      ExprKind::BinOp {
        ref lhs,
        ref op,
        ref rhs,
        ..
      } => self.codegen_binop_expr(lhs, op, rhs),
      ExprKind::Lit(ref lit) => self.codegen_lit_expr(lit),
      _ => unimplemented!(),
    }
  }

  fn codegen_lit_expr(&mut self, kind: &LitKind) -> LLVMValueRef {
    match kind {
      // LitKind::Bool(ref value) => make_codegen_lit_bool_expr(value),
      // LitKind::Char(ref value) => make_codegen_lit_char_expr(value),
      LitKind::Real(ref value) => make_codegen_lit_real_expr(self.context, value),
      LitKind::Int(ref value) => make_codegen_lit_int_expr(self.context, value),
      // LitKind::Str(ref value) => make_codegen_lit_str_expr(value),
      _ => unreachable!(),
    }
  }

  fn codegen_stmt(
    &mut self,
    context: LLVMContextRef,
    builder: LLVMBuilderRef,
    fun: LLVMValueRef,
    names: &mut HashMap<String, LLVMValueRef>,
    stmt: &Box<Stmt>,
  ) -> LLVMValueRef {
    match stmt.kind() {
      StmtKind::Expr(ref expr) => self.codegen_expr_stmt(expr),
      _ => unreachable!(),
    }
  }
}

fn insert_allocations(
  context: LLVMContextRef,
  builder: LLVMBuilderRef,
  names: &mut HashMap<String, LLVMValueRef>,
  stmts: &[Box<Stmt>],
) {
  // let mut variable_names = HashSet::new();

  // for stmt in stmts {
  //   match stmt.kind {
  //     StmtKind::Val
  //     _ => unreachable!(),
  //   }
  // }

  // for variable_name in variable_names {
  // unsafe {
  // let int_type = LLVMInt64TypeInContext(context);
  // let name = CString::new(variable_name.as_bytes()).unwrap();
  // let pointer = LLVMBuildAlloca(builder, int_type, name.as_ptr());

  // names.insert(variable_name.to_owned(), pointer);
  // }
  // }
}

pub fn make_codegen_lit_expr(
  context: LLVMContextRef,
  expr: Box<Expr>,
) -> LLVMValueRef {
  unsafe {
    let int_ty = LLVMInt64TypeInContext(context);
    let int = expr.text().parse().unwrap();

    LLVMConstInt(int_ty, int, 0)
  }
}

pub fn make_codegen_binop_add_expr(
  builder: LLVMBuilderRef,
  lhs: LLVMValueRef,
  rhs: LLVMValueRef,
) -> LLVMValueRef {
  unsafe {
    let name = CString::new("addtmp").unwrap();
    LLVMBuildAdd(builder, lhs, rhs, name.as_ptr())
  }
}

pub fn make_codegen_sub_binop_expr(
  builder: LLVMBuilderRef,
  lhs: LLVMValueRef,
  rhs: LLVMValueRef,
) -> LLVMValueRef {
  unsafe {
    let name = CString::new("subtmp").unwrap();
    LLVMBuildSub(builder, lhs, rhs, name.as_ptr())
  }
}

pub fn make_codegen_mul_binop_expr(
  builder: LLVMBuilderRef,
  lhs: LLVMValueRef,
  rhs: LLVMValueRef,
) -> LLVMValueRef {
  unsafe {
    let name = CString::new("multmp").unwrap();
    LLVMBuildMul(builder, lhs, rhs, name.as_ptr())
  }
}

pub fn make_codegen_div_binop_expr(
  builder: LLVMBuilderRef,
  lhs: LLVMValueRef,
  rhs: LLVMValueRef,
) -> LLVMValueRef {
  unsafe {
    let name = CString::new("divtmp").unwrap();
    LLVMBuildUDiv(builder, lhs, rhs, name.as_ptr())
  }
}

pub fn make_codegen_lit_int_expr(
  context: LLVMContextRef,
  int: &i64,
) -> LLVMValueRef {
  unsafe {
    let int_ty = LLVMInt64TypeInContext(context);
    LLVMConstInt(int_ty, *int as u64, 0)
  }
}

pub fn make_codegen_lit_real_expr(
  context: LLVMContextRef,
  real: &f64,
) -> LLVMValueRef {
  unsafe {
    // TODO: no-sense, not sure if we can mix float context with real constant
    let real_ty = LLVMFloatTypeInContext(context);
    LLVMConstReal(real_ty, *real)
  }
}
