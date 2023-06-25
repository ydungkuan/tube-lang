#![deny(clippy::all)]
use tube_vm::vm::default_native_vm;
use std::env;
use std::fs::read_to_string;
use std::{path::PathBuf, process};

#[cfg(feature = "jemalloc")]
use jemallocator::Jemalloc;

#[cfg(feature = "jemalloc")]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

fn main() {
  let mut vm = default_native_vm();
  let args: Vec<String> = env::args().collect();

  match args.as_slice() {
    [_] => process::exit(1),
    _ => {
      let file_path = &args.as_slice()[1];
      let path = PathBuf::from(file_path);

      match read_to_string(&path) {
        Ok(source) => process::exit(vm.run(path, &source).0),
        Err(e) => {
          eprintln!("{e}");
          process::exit(4)
        },
      }
    },
  }
}
