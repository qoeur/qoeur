#![feature(box_patterns)]
#![feature(box_syntax)]
#![feature(decl_macro)]
#![recursion_limit = "256"]

#[macro_use]
mod macros;

use time::SteadyTime;

pub struct Timer {}

impl Timer {
  pub fn time_operation<Op, Res>(
    level: usize,
    op_name: &str,
    op: Op,
    indent: Box<dyn FnOnce(usize) -> String>,
  ) -> Res
  where
    Op: FnOnce() -> Res,
    Res: Sized,
  {
    let start_time = SteadyTime::now();
    let res = op();
    let end_time = SteadyTime::now() - start_time;
    let us = end_time.num_microseconds().unwrap_or(0) % 1000;

    println!(
      "{}{}: {}.{:03} ms",
      indent(level),
      op_name,
      end_time.num_milliseconds(),
      us
    );

    res
  }

  pub fn time_operation_mut<Op, Res>(
    level: usize,
    op_name: &str,
    mut op: Op,
    indent: Box<dyn FnOnce(usize) -> String>,
  ) -> Res
  where
    Op: FnMut() -> Res,
    Res: Sized,
  {
    let start_time = SteadyTime::now();
    let res = op();
    let end_time = SteadyTime::now() - start_time;
    let us = end_time.num_microseconds().unwrap_or(0) % 1000;

    println!(
      "{}{}: {}.{:03} ms",
      indent(level),
      op_name,
      end_time.num_milliseconds(),
      us
    );

    res
  }
}
