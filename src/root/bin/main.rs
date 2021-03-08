use qoeurcp::tokenizer::{parse, tokenize};

fn main() {
  let pathname = "data/code/minimal.q5";
  let path = std::path::Path::new(&pathname);

  let f = match std::fs::read_to_string(&path) {
    Err(_) => None,
    Ok(file) => Some(file),
  };

  let f = f.unwrap();
  let _ = tokenize(&f);
  let _ = parse(&f);
}
