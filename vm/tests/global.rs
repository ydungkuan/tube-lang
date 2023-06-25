use tube_vm::vm::VmExit;
use support::{assert_file_exit_and_stdio, assert_files_exit};

mod support;

fn test_files(paths: &[&str], result: VmExit) -> Result<(), std::io::Error> {
  assert_files_exit(paths, FILE_PATH, result)
}

fn test_file_with_stdio(
  path: &str,
  stdout: Option<Vec<&str>>,
  stderr: Option<Vec<&str>>,
  result: VmExit,
) -> Result<(), std::io::Error> {
  assert_file_exit_and_stdio(path, FILE_PATH, None, None, stdout, stderr, result)
}

const FILE_PATH: &str = file!();

#[test]
fn bool() -> Result<(), std::io::Error> {
  test_files(&vec!["std_lib/global/bool/str.tube"], VmExit::Ok)?;

  test_files(&vec![], VmExit::CompileError)?;

  test_files(&vec![], VmExit::RuntimeError)
}

#[test]
fn class() -> Result<(), std::io::Error> {
  test_files(
    &vec![
      "std_lib/global/class/name.tube",
      "std_lib/global/class/str.tube",
      "std_lib/global/class/superCls.tube",
    ],
    VmExit::Ok,
  )?;

  test_files(&vec![], VmExit::RuntimeError)
}

#[test]
fn channel() -> Result<(), std::io::Error> {
  test_files(
    &vec![
      "std_lib/global/channel/capacity.tube",
      "std_lib/global/channel/close.tube",
      "std_lib/global/channel/len.tube",
      "std_lib/global/channel/str.tube",
    ],
    VmExit::Ok,
  )?;

  test_files(
    &vec!["std_lib/global/channel/close_close.tube"],
    VmExit::RuntimeError,
  )
}

#[test]
fn closure() -> Result<(), std::io::Error> {
  test_files(
    &vec![
      "std_lib/global/closure/name.tube",
      "std_lib/global/closure/call.tube",
      "std_lib/global/closure/len.tube",
    ],
    VmExit::Ok,
  )?;

  test_files(
    &vec![
      "std_lib/global/closure/name_wrong_args.tube",
      "std_lib/global/closure/call_wrong_args.tube",
      "std_lib/global/closure/len_wrong_args.tube",
    ],
    VmExit::RuntimeError,
  )
}

#[test]
fn error() -> Result<(), std::io::Error> {
  test_files(
    &vec![
      "std_lib/global/error/construct.tube",
      "std_lib/global/error/sub_class.tube",
    ],
    VmExit::Ok,
  )?;

  test_files(
    &vec![
    ],
    VmExit::RuntimeError,
  )
}

#[test]
fn fun() -> Result<(), std::io::Error> {
  test_files(
    &vec![
      "std_lib/global/fun/name.tube",
      "std_lib/global/fun/call.tube",
      "std_lib/global/fun/len.tube",
    ],
    VmExit::Ok,
  )?;

  test_files(
    &vec![
      "std_lib/global/fun/name_wrong_args.tube",
      "std_lib/global/fun/call_wrong_args.tube",
      "std_lib/global/fun/len_wrong_args.tube",
    ],
    VmExit::RuntimeError,
  )
}

#[test]
fn iter() -> Result<(), std::io::Error> {
  test_files(
    &vec![
      "std_lib/global/iter/all.tube",
      "std_lib/global/iter/any.tube",
      "std_lib/global/iter/chain.tube",
      "std_lib/global/iter/each.tube",
      "std_lib/global/iter/filter.tube",
      "std_lib/global/iter/filter_method.tube",
      "std_lib/global/iter/first.tube",
      "std_lib/global/iter/into.tube",
      "std_lib/global/iter/iter.tube",
      "std_lib/global/iter/last.tube",
      "std_lib/global/iter/len.tube",
      "std_lib/global/iter/map.tube",
      "std_lib/global/iter/map_method.tube",
      "std_lib/global/iter/next.tube",
      "std_lib/global/iter/reduce.tube",
      "std_lib/global/iter/skip.tube",
      "std_lib/global/iter/str.tube",
      "std_lib/global/iter/take.tube",
      "std_lib/global/iter/to_list.tube",
      "std_lib/global/iter/zip.tube",
    ],
    VmExit::Ok,
  )?;

  test_files(&vec![], VmExit::RuntimeError)
}

#[test]
fn list() -> Result<(), std::io::Error> {
  test_files(
    &vec![
      "std_lib/global/list/clear.tube",
      "std_lib/global/list/collect.tube",
      "std_lib/global/list/has.tube",
      "std_lib/global/list/index.tube",
      "std_lib/global/list/index_get.tube",
      "std_lib/global/list/index_get_negative.tube",
      "std_lib/global/list/index_get_nested.tube",
      "std_lib/global/list/index_set.tube",
      "std_lib/global/list/index_set_negative.tube",
      "std_lib/global/list/index_set_nested.tube",
      "std_lib/global/list/index_set_pass_through.tube",
      "std_lib/global/list/insert.tube",
      "std_lib/global/list/iter.tube",
      "std_lib/global/list/len.tube",
      "std_lib/global/list/pop.tube",
      "std_lib/global/list/push.tube",
      "std_lib/global/list/remove.tube",
      "std_lib/global/list/rev.tube",
      "std_lib/global/list/slice.tube",
      "std_lib/global/list/str.tube",
    ],
    VmExit::Ok,
  )?;

  test_files(
    &vec![
      "std_lib/global/list/index_get_fractional.tube",
      "std_lib/global/list/index_get_fractional_negative.tube",
      "std_lib/global/list/index_get_out_of_range.tube",
      "std_lib/global/list/index_get_out_of_range_negative.tube",
      "std_lib/global/list/index_set_fractional.tube",
      "std_lib/global/list/index_set_fractional_negative.tube",
      "std_lib/global/list/index_set_out_of_range.tube",
      "std_lib/global/list/index_set_out_of_range_negative.tube",
      "std_lib/global/list/insert_out_of_bounds.tube",
      "std_lib/global/list/remove_out_of_bounds.tube",
    ],
    VmExit::RuntimeError,
  )
}

#[test]
fn map() -> Result<(), std::io::Error> {
  test_files(
    &vec![
      "std_lib/global/map/get.tube",
      "std_lib/global/map/has.tube",
      "std_lib/global/map/index_get.tube",
      "std_lib/global/map/index_get_nan.tube",
      "std_lib/global/map/index_get_nested.tube",
      "std_lib/global/map/index_get_nested.tube",
      "std_lib/global/map/index_get_ref_equal.tube",
      "std_lib/global/map/index_set.tube",
      "std_lib/global/map/index_set_pass_through.tube",
      "std_lib/global/map/insert.tube",
      "std_lib/global/map/iter.tube",
      "std_lib/global/map/len.tube",
      "std_lib/global/map/remove.tube",
      "std_lib/global/map/set.tube",
      "std_lib/global/map/str.tube",
    ],
    VmExit::Ok,
  )?;

  test_files(
    &vec![
      "std_lib/global/map/remove_missing_key.tube",
      "std_lib/global/map/index_get_key_not_found.tube",
    ],
    VmExit::RuntimeError,
  )
}

#[test]
fn method() -> Result<(), std::io::Error> {
  test_files(
    &vec![
      "std_lib/global/method/name.tube",
      "std_lib/global/method/call.tube",
    ],
    VmExit::Ok,
  )?;

  test_files(&vec![], VmExit::RuntimeError)
}

#[test]
fn module() -> Result<(), std::io::Error> {
  test_files(
    &vec!["std_lib/global/module/name.tube"],
    VmExit::Ok,
  )?;

  test_files(&vec![], VmExit::RuntimeError)
}

#[test]
fn nil() -> Result<(), std::io::Error> {
  test_files(&vec!["std_lib/global/nil/str.tube"], VmExit::Ok)?;

  test_files(&vec![], VmExit::RuntimeError)
}

#[test]
fn number() -> Result<(), std::io::Error> {
  test_files(
    &vec![
      "std_lib/global/number/ceil.tube",
      "std_lib/global/number/cmp.tube",
      "std_lib/global/number/floor.tube",
      "std_lib/global/number/parse.tube",
      "std_lib/global/number/round.tube",
      "std_lib/global/number/str.tube",
      "std_lib/global/number/times.tube",
    ],
    VmExit::Ok,
  )?;

  test_files(&vec![], VmExit::RuntimeError)
}

#[test]
fn object() -> Result<(), std::io::Error> {
  test_files(
    &vec![
      "std_lib/global/object/cls.tube",
      "std_lib/global/object/equals.tube",
      "std_lib/global/object/is_a.tube",
      "std_lib/global/object/str.tube",
    ],
    VmExit::Ok,
  )
}

#[test]
#[cfg(not(feature = "debug"))]
fn print() -> Result<(), std::io::Error> {
  test_file_with_stdio(
    "std_lib/global/print/basic.tube",
    Some(vec!["10", "true", "['cat']", "{}"]),
    None,
    VmExit::Ok,
  )?;

  test_file_with_stdio(
    "std_lib/global/print/multi.tube",
    Some(vec!["10 false true ['dog'] { 'cat': nil }"]),
    None,
    VmExit::Ok,
  )?;

  test_file_with_stdio(
    "std_lib/global/print/with_newline_char.tube",
    Some(vec!["hi!", "bye!"]),
    None,
    VmExit::Ok,
  )
}

#[test]
fn str() -> Result<(), std::io::Error> {
  test_files(
    &vec![
      "std_lib/global/str/down_case.tube",
      "std_lib/global/str/has.tube",
      "std_lib/global/str/index.tube",
      "std_lib/global/str/iter.tube",
      "std_lib/global/str/len.tube",
      "std_lib/global/str/slice.tube",
      "std_lib/global/str/split.tube",
      "std_lib/global/str/str.tube",
      "std_lib/global/str/trim.tube",
      "std_lib/global/str/trim_start.tube",
      "std_lib/global/str/trim_end.tube",
      "std_lib/global/str/up_case.tube",
    ],
    VmExit::Ok,
  )?;

  test_files(&vec![], VmExit::RuntimeError)
}

#[test]
fn tuple() -> Result<(), std::io::Error> {
  test_files(
    &vec![
      "std_lib/global/tuple/collect.tube",
      "std_lib/global/tuple/has.tube",
      "std_lib/global/tuple/index.tube",
      "std_lib/global/tuple/index_get.tube",
      "std_lib/global/tuple/index_get_negative.tube",
      "std_lib/global/tuple/index_get_nested.tube",
      "std_lib/global/tuple/iter.tube",
      "std_lib/global/tuple/len.tube",
      "std_lib/global/tuple/slice.tube",
      "std_lib/global/tuple/str.tube",
    ],
    VmExit::Ok,
  )?;

  test_files(
    &vec![
      "std_lib/global/tuple/index_get_fractional.tube",
      "std_lib/global/tuple/index_get_fractional_negative.tube",
      "std_lib/global/tuple/index_get_out_of_range.tube",
      "std_lib/global/tuple/index_get_out_of_range_negative.tube",
    ],
    VmExit::RuntimeError,
  )
}
