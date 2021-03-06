macro time($e:expr) {
  let t0 = crate::time::precise_time_ns();
  let result = $e;
  let dt = crate::time::precise_time_ns() - t0;

  (result, dt)
}
