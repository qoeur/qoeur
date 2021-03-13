use qoeurcp_tokenizer::ast::*;
use qoeurcp_tokenizer::parse;

use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{DataContext, Linkage, Module};

use std::collections::HashMap;
use std::slice;

pub struct Jit {
  builder_context: FunctionBuilderContext,
  ctx: codegen::Context,
  data_ctx: DataContext,
  module: JITModule,
}

impl Default for Jit {
  fn default() -> Jit {
    let builder = JITBuilder::new(cranelift_module::default_libcall_names());
    let module = JITModule::new(builder);

    Self {
      builder_context: FunctionBuilderContext::new(),
      ctx: module.make_context(),
      data_ctx: DataContext::new(),
      module,
    }
  }
}

impl Jit {
  pub fn compile(&mut self, input: &str) -> Result<*const u8, String> {
    let name = "";
    let tree = parse(input);

    self.translate(vec![], String::new(), vec![])?;

    let id = self
      .module
      .declare_function(name, Linkage::Export, &self.ctx.func.signature)
      .map_err(|e| e.to_string())?;

    self
      .module
      .define_function(
        id,
        &mut self.ctx,
        &mut codegen::binemit::NullTrapSink {},
      )
      .map_err(|e| e.to_string())?;

    self.module.clear_context(&mut self.ctx);
    self.module.finalize_definitions();

    let code = self.module.get_finalized_function(id);

    Ok(code)
  }

  pub fn create_data(
    &mut self,
    name: &str,
    contents: Vec<u8>,
  ) -> Result<&[u8], String> {
    self.data_ctx.define(contents.into_boxed_slice());

    let id = self
      .module
      .declare_data(name, Linkage::Export, true, false)
      .map_err(|e| e.to_string())?;

    self
      .module
      .define_data(id, &self.data_ctx)
      .map_err(|e| e.to_string())?;

    self.data_ctx.clear();
    self.module.finalize_definitions();

    let buffer = self.module.get_finalized_data(id);

    Ok(unsafe { slice::from_raw_parts(buffer.0, buffer.1) })
  }

  fn translate(
    &mut self,
    params: Vec<String>,
    the_return: String,
    stmts: Vec<Box<Stmt>>,
  ) -> Result<(), String> {
    let int = self.module.target_config().pointer_type();

    for _p in &params {
      self.ctx.func.signature.params.push(AbiParam::new(int));
    }

    self.ctx.func.signature.returns.push(AbiParam::new(int));

    let mut builder =
      FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_context);
    let entry_block = builder.create_block();

    builder.append_block_params_for_function_params(entry_block);
    builder.switch_to_block(entry_block);
    builder.seal_block(entry_block);

    let variables = declare_variables(
      int,
      &mut builder,
      &params,
      &the_return,
      &stmts,
      entry_block,
    );

    let mut trans = FunctionTranslator {
      int,
      builder,
      variables,
      module: &mut self.module,
    };

    for expr in stmts {
      trans.translate_expr(expr);
    }

    let return_variable = trans.variables.get(&the_return).unwrap();
    let return_value = trans.builder.use_var(*return_variable);

    trans.builder.ins().return_(&[return_value]);
    trans.builder.finalize();

    Ok(())
  }
}

struct FunctionTranslator<'a> {
  int: types::Type,
  builder: FunctionBuilder<'a>,
  variables: HashMap<String, Variable>,
  module: &'a mut JITModule,
}

impl<'a> FunctionTranslator<'a> {
  fn translate_expr(&mut self, stmt: Box<Stmt>) -> Value {
    match stmt.kind {
      _ => unreachable!(),
    }
  }
}

fn declare_variable(
  int: types::Type,
  builder: &mut FunctionBuilder,
  variables: &mut HashMap<String, Variable>,
  index: &mut usize,
  name: &str,
) -> Variable {
  let var = Variable::new(*index);

  if !variables.contains_key(name) {
    variables.insert(name.into(), var);
    builder.declare_var(var, int);
    *index += 1;
  }

  var
}

// TODO: util
fn declare_variables(
  int: types::Type,
  builder: &mut FunctionBuilder,
  params: &Vec<String>,
  the_return: &str,
  stmts: &Vec<Box<Stmt>>,
  entry_block: cranelift::prelude::Block,
) -> HashMap<String, Variable> {
  let mut variables = HashMap::new();
  let mut index = 0;

  for (i, name) in params.iter().enumerate() {
    let val = builder.block_params(entry_block)[i];
    let var = declare_variable(int, builder, &mut variables, &mut index, name);

    builder.def_var(var, val);
  }

  let zero = builder.ins().iconst(int, 0);
  let return_variable =
    declare_variable(int, builder, &mut variables, &mut index, the_return);

  builder.def_var(return_variable, zero);

  for stmt in stmts {
    declare_variables_in_stmt(int, builder, &mut variables, &mut index, stmt);
  }

  variables
}

fn declare_variables_in_stmt(
  int: types::Type,
  builder: &mut FunctionBuilder,
  variables: &mut HashMap<String, Variable>,
  index: &mut usize,
  expr: &Box<Stmt>,
) {
  match expr.kind {
    _ => (),
  }
}
