use tube_vm::vm::VmExit;
use support::assert_files_exit;

mod support;

fn test_files(paths: &[&str], result: VmExit) -> Result<(), std::io::Error> {
  assert_files_exit(paths, FILE_PATH, result)
}

const FILE_PATH: &str = file!();

#[test]
fn utils() -> Result<(), std::io::Error> {
  test_files(
    &vec![
      "std_lib/regexp/class/captures.tube",
      "std_lib/regexp/class/match.tube",
      "std_lib/regexp/class/test.tube",
    ],
    VmExit::Ok,
  )?;

  test_files(&vec![], VmExit::CompileError)?;

  test_files(&vec![], VmExit::RuntimeError)
}
