use tube_vm::vm::{default_native_vm, VmExit};
use support::{assert_file_exit_and_stdio, assert_files_exit, assert_files_exit_with_cwd};

mod support;

fn test_file_exits(paths: &[&str], result: VmExit) -> Result<(), std::io::Error> {
  assert_files_exit(paths, FILE_PATH, result)
}

fn test_file_exits_with_cwd(
  paths: &[&str],
  cwd: &str,
  result: VmExit,
) -> Result<(), std::io::Error> {
  assert_files_exit_with_cwd(paths, FILE_PATH, cwd, result)
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
fn build() {
  default_native_vm();
  assert!(true);
}

#[test]
fn assignment() -> Result<(), std::io::Error> {
  test_file_exits(
    &vec![
      "language/assignment/allowed_postfix.tube",
      "language/assignment/associativity.tube",
      "language/assignment/global.tube",
      "language/assignment/local.tube",
      "language/assignment/syntax.tube",
    ],
    VmExit::Ok,
  )?;

  test_file_exits(
    &vec![
      "language/assignment/grouping.tube",
      "language/assignment/infix_operator.tube",
      "language/assignment/prefix_operator.tube",
      "language/assignment/to_this.tube",
      "language/assignment/undefined.tube",
    ],
    VmExit::CompileError,
  )?;

  test_file_exits(&vec![], VmExit::RuntimeError)
}

#[test]
fn binary_assignment() -> Result<(), std::io::Error> {
  test_file_exits(
    &vec![
      "language/binary_assignment/local.tube",
      "language/binary_assignment/syntax.tube",
      "language/binary_assignment/associativity.tube",
      "language/binary_assignment/global.tube",
      "language/binary_assignment/operators.tube",
    ],
    VmExit::Ok,
  )?;

  test_file_exits(
    &vec![
      "language/binary_assignment/grouping.tube",
      "language/binary_assignment/infix_operator.tube",
      "language/binary_assignment/prefix_operator.tube",
      "language/binary_assignment/to_this.tube",
      "language/binary_assignment/undefined.tube",
    ],
    VmExit::CompileError,
  )?;

  test_file_exits(&vec![], VmExit::RuntimeError)
}

#[test]
fn block() -> Result<(), std::io::Error> {
  test_file_exits(
    &vec!["language/block/empty.tube", "language/block/scope.tube"],
    VmExit::Ok,
  )
}

#[test]
fn bool() -> Result<(), std::io::Error> {
  test_file_exits(
    &vec!["language/bool/equality.tube", "language/bool/not.tube"],
    VmExit::Ok,
  )
}

#[test]
fn break_() -> Result<(), std::io::Error> {
  test_file_exits(
    &vec![
      "language/break/additional_scopes.tube",
      "language/break/for_break.tube",
      "language/break/nested_for_loops.tube",
      "language/break/nested_while_loops.tube",
      "language/break/while_break.tube",
      "language/break/with_drops.tube",
    ],
    VmExit::Ok,
  )?;

  test_file_exits(
    &vec![
      "language/break/outside_loop.tube",
      "language/break/in_function_in_loop.tube"
    ],
    VmExit::CompileError,
  )
}

#[test]
fn call() -> Result<(), std::io::Error> {
  test_file_exits(
    &vec![
      "language/call/bool.tube",
      "language/call/nil.tube",
      "language/call/num.tube",
      "language/call/object.tube",
      "language/call/string.tube",
    ],
    VmExit::RuntimeError,
  )
}

#[test]
fn channel() -> Result<(), std::io::Error> {
  test_file_exits(
    &vec![
      "language/channel/buffered.tube",
      "language/channel/channel_send_channel.tube",
      "language/channel/channel_sync.tube",
      "language/channel/fancy.tube",
      "language/channel/in_collection.tube",
      "language/channel/in_function.tube",
      "language/channel/in_instance.tube",
      "language/channel/in_instance_implicit.tube",
      "language/channel/multi_capture_increment.tube",
      "language/channel/receive_buffered.tube",
      "language/channel/receive_buffered_closed.tube",
      "language/channel/receive_sync.tube",
      "language/channel/receive_sync_closed.tube",
      "language/channel/receive_sync_no_send.tube",
      "language/channel/send_buffered.tube",
      "language/channel/send_sync.tube",
      "language/channel/sync.tube",
    ],
    VmExit::Ok,
  )?;

  test_file_exits(
    &vec![
      "language/channel/missing_closing_paren.tube",
      "language/channel/missing_open_paren.tube",
    ],
    VmExit::CompileError,
  )?;

  test_file_exits(
    &vec![
      "language/channel/close_closed.tube",
      "language/channel/non_integer_capacity.tube",
      "language/channel/non_number_capacity.tube",
      "language/channel/receive_buffered_deadlock.tube",
      "language/channel/receive_sync_deadlock.tube",
      "language/channel/send_buffered_closed.tube",
      "language/channel/send_buffered_deadlock.tube",
      "language/channel/send_sync_closed.tube",
      "language/channel/send_sync_deadlock.tube",
    ],
    VmExit::RuntimeError,
  )
}

#[test]
fn class() -> Result<(), std::io::Error> {
  test_file_exits(
    &vec![
      "language/class/allowed_postfix.tube",
      "language/class/empty.tube",
      "language/class/inherited_method.tube",
      "language/class/local_inherit_other.tube",
      "language/class/local_reference_self.tube",
      "language/class/reference_self.tube",
    ],
    VmExit::Ok,
  )?;

  test_file_exits(
    &vec![
      "language/class/inherit_self.tube",
      "language/class/local_inherit_self.tube",
    ],
    VmExit::CompileError,
  )?;

  test_file_exits(&vec![], VmExit::RuntimeError)
}

#[test]
fn closure() -> Result<(), std::io::Error> {
  test_file_exits(
    &vec![
      "language/closure/assign_to_closure.tube",
      "language/closure/assign_to_shadowed_later.tube",
      "language/closure/close_over_function_parameter.tube",
      "language/closure/close_over_later_variable.tube",
      "language/closure/close_over_method_parameter.tube",
      "language/closure/closed_closure_in_function.tube",
      "language/closure/nested_closure.tube",
      "language/closure/open_closure_in_function.tube",
      "language/closure/reference_closure_multiple_times.tube",
      "language/closure/reuse_closure_slot.tube",
      "language/closure/shadow_closure_with_local.tube",
      "language/closure/unused_closure.tube",
      "language/closure/unused_later_closure.tube",
    ],
    VmExit::Ok,
  )?;

  test_file_exits(&vec![], VmExit::CompileError)?;

  test_file_exits(&vec![], VmExit::RuntimeError)
}

#[test]
fn continue_() -> Result<(), std::io::Error> {
  test_file_exits(
    &vec![
      "language/continue/additional_scopes.tube",
      "language/continue/for_continue.tube",
      "language/continue/nested_for_loops.tube",
      "language/continue/nested_while_loops.tube",
      "language/continue/while_continue.tube",
      "language/continue/with_drops.tube",
    ],
    VmExit::Ok,
  )?;

  test_file_exits(
    &vec![
      "language/continue/outside_loop.tube",
      "language/continue/in_function_in_loop.tube"
    ],
    VmExit::CompileError,
  )
}

#[test]
fn comments() -> Result<(), std::io::Error> {
  test_file_exits(
    &vec![
      "language/comments/line_at_eof.tube",
      "language/comments/only_line_comment_and_line.tube",
      "language/comments/only_line_comment.tube",
      "language/comments/unicode.tube",
    ],
    VmExit::Ok,
  )
}

#[test]
fn constructor() -> Result<(), std::io::Error> {
  test_file_exits(
    &vec![
      "language/constructor/arguments.tube",
      "language/constructor/call_init_early_return.tube",
      "language/constructor/call_init_explicitly.tube",
      "language/constructor/default.tube",
      "language/constructor/early_return.tube",
      "language/constructor/return_in_nested_function.tube",
    ],
    VmExit::Ok,
  )?;

  test_file_exits(
    &vec!["language/constructor/return_value.tube"],
    VmExit::CompileError,
  )?;

  test_file_exits(
    &vec![
      "language/constructor/default_arguments.tube",
      "language/constructor/extra_arguments.tube",
      "language/constructor/missing_arguments.tube",
    ],
    VmExit::RuntimeError,
  )
}

#[test]
fn exception() -> Result<(), std::io::Error> {
  test_file_exits(
    &vec![
      "language/exception/top_level_catch.tube",
      "language/exception/nested_break.tube",
      "language/exception/nested_continue.tube",
      "language/exception/nested_return.tube",
      "language/exception/one_deep_catch.tube",
      "language/exception/two_deep_catch.tube",
      "language/exception/top_level_catch_raise.tube",
    ],
    VmExit::Ok,
  )?;

  test_file_exits(
    &vec![
      "language/exception/catch_no_block.tube",
      "language/exception/try_no_block.tube",
      "language/exception/try_no_catch.tube",
    ],
    VmExit::CompileError,
  )?;

  test_file_with_stdio(
    "language/exception/top_level_raise.tube",
    None,
    Some(vec![
      "Traceback (most recent call last):",
      "  fixture/language/exception/top_level_raise.tube:1 in script",
      "Error: raise",
    ]),
    VmExit::RuntimeError,
  )?;

  test_file_with_stdio(
    "language/exception/one_deep_raise.tube",
    None,
    Some(vec![
      "Traceback (most recent call last):",
      "  fixture/language/exception/one_deep_raise.tube:2 in raiser()",
      "  fixture/language/exception/one_deep_raise.tube:5 in script",
      "Error: raise",
    ]),
    VmExit::RuntimeError,
  )?;

  test_file_with_stdio(
    "language/exception/two_deep_raise.tube",
    None,
    Some(vec![
      "Traceback (most recent call last):",
      "  fixture/language/exception/two_deep_raise.tube:6 in raiser()",
      "  fixture/language/exception/two_deep_raise.tube:2 in outer()",
      "  fixture/language/exception/two_deep_raise.tube:9 in script",
      "Error: raise",
    ]),
    VmExit::RuntimeError,
  )
}

#[test]
fn export() -> Result<(), std::io::Error> {
  test_file_exits(
    &vec![
      "language/export/declaration_class.tube",
      "language/export/declaration_fn.tube",
      "language/export/declaration_let.tube",
    ],
    VmExit::Ok,
  )?;

  test_file_exits(
    &vec![
      "language/export/literal.tube",
      "language/export/local.tube",
      "language/export/non_declaration_class.tube",
      "language/export/non_declaration_fn.tube",
      "language/export/non_declaration_let.tube",
    ],
    VmExit::CompileError,
  )?;

  test_file_exits(&vec![], VmExit::RuntimeError)
}

#[test]
fn expressions() -> Result<(), std::io::Error> {
  test_file_exits(&vec!["language/expressions/evaluate.tube"], VmExit::Ok)?;

  test_file_exits(&vec![], VmExit::CompileError)?;

  test_file_exits(&vec![], VmExit::RuntimeError)
}

#[test]
fn field() -> Result<(), std::io::Error> {
  test_file_exits(
    &vec![
      "language/field/call_function_field.tube",
      "language/field/call_function_field_implicit.tube",
      "language/field/many.tube",
      "language/field/many_implicit.tube",
      "language/field/method_binds_self.tube",
      "language/field/method_binds_self_implicit.tube",
      "language/field/method.tube",
      "language/field/on_instance.tube",
      "language/field/on_instance_implicit.tube",
    ],
    VmExit::Ok,
  )?;

  test_file_exits(
    &vec!["language/field/set_evaluation_order.tube"],
    VmExit::CompileError,
  )?;

  test_file_exits(
    &vec![
      "language/field/call_nonfunction_field.tube",
      "language/field/get_on_bool.tube",
      "language/field/get_on_class.tube",
      "language/field/get_on_function.tube",
      "language/field/get_on_nil.tube",
      "language/field/get_on_num.tube",
      "language/field/get_on_string.tube",
      "language/field/set_undefined.tube",
      "language/field/set_on_bool.tube",
      "language/field/set_on_class.tube",
      "language/field/set_on_function.tube",
      "language/field/set_on_nil.tube",
      "language/field/set_on_num.tube",
      "language/field/set_on_string.tube",
      "language/field/get_undefined.tube",
    ],
    VmExit::RuntimeError,
  )
}

#[test]
fn for_loop() -> Result<(), std::io::Error> {
  test_file_exits(
    &vec![
      "language/for/return_inside.tube",
      "language/for/scope.tube",
      "language/for/closure_in_body.tube",
    ],
    VmExit::Ok,
  )?;

  test_file_exits(
    &vec![
      "language/for/cannot_assign_iter.tube",
      "language/for/class_in_body.tube",
      "language/for/fn_in_body.tube",
      "language/for/let_in_body.tube",
      "language/for/statement_iterator.tube",
    ],
    VmExit::CompileError,
  )?;

  test_file_exits(&vec![], VmExit::RuntimeError)
}

#[test]
fn function() -> Result<(), std::io::Error> {
  test_file_exits(
    &vec![
      "language/function/allowed_postfix.tube",
      "language/function/empty_body.tube",
      "language/function/local_recursion.tube",
      "language/function/mutual_recursion.tube",
      "language/function/parameters.tube",
      "language/function/print.tube",
      "language/function/recursion.tube",
    ],
    VmExit::Ok,
  )?;

  test_file_exits(
    &vec![
      "language/function/body_must_be_block.tube",
      "language/function/missing_comma_in_parameters.tube",
      "language/function/too_many_arguments.tube",
      "language/function/too_many_parameters.tube",
      "language/function/local_mutual_recursion.tube",
    ],
    VmExit::CompileError,
  )?;

  test_file_exits(
    &vec![
      "language/function/extra_arguments.tube",
      "language/function/missing_arguments.tube",
    ],
    VmExit::RuntimeError,
  )
}

#[test]
fn hooks() -> Result<(), std::io::Error> {
  test_file_exits(
    &vec![
      "language/hooks/call_native.tube",
      "language/hooks/call_ly_function.tube",
      "language/hooks/call_ly_closure.tube",
      "language/hooks/call_ly_instance.tube",
    ],
    VmExit::Ok,
  )?;

  test_file_exits(&vec![], VmExit::CompileError)?;

  test_file_exits(
    &vec![
      "language/hooks/pass_error_native.tube",
      "language/hooks/pass_error_ly_function.tube",
      "language/hooks/pass_error_ly_closure.tube",
      "language/hooks/pass_error_ly_instance.tube",
    ],
    VmExit::RuntimeError,
  )
}

#[test]
fn if_stmt() -> Result<(), std::io::Error> {
  test_file_exits(
    &vec![
      "language/if/dangling_else.tube",
      "language/if/else.tube",
      "language/if/if.tube",
      "language/if/truth.tube",
    ],
    VmExit::Ok,
  )?;

  test_file_exits(
    &vec![
      "language/if/class_in_else.tube",
      "language/if/class_in_then.tube",
      "language/if/fun_in_else.tube",
      "language/if/fun_in_then.tube",
      "language/if/let_in_then.tube",
      "language/if/let_in_then.tube",
    ],
    VmExit::CompileError,
  )?;

  test_file_exits(&vec![], VmExit::RuntimeError)
}

#[test]
fn implicit_return() -> Result<(), std::io::Error> {
  test_file_exits(
    &vec![
      "language/implicit_return/in_function.tube",
      "language/implicit_return/in_method.tube",
    ],
    VmExit::Ok,
  )?;

  test_file_exits(
    &vec![
      "language/implicit_return/after_else.tube",
      "language/implicit_return/after_if.tube",
      "language/implicit_return/after_while.tube",
      "language/implicit_return/at_top_level.tube",
      "language/implicit_return/in_function_middle.tube",
      "language/implicit_return/in_method_middle.tube",
      "language/implicit_return/in_init.tube",
    ],
    VmExit::CompileError,
  )?;

  test_file_exits(&vec![], VmExit::RuntimeError)
}

#[test]
fn import() -> Result<(), std::io::Error> {
  test_file_exits(
    &vec![
      "language/import/module.tube",
      "language/import/module_rename.tube",
      "language/import/symbol.tube",
      "language/import/symbol_rename.tube",
    ],
    VmExit::Ok,
  )?;

  test_file_exits_with_cwd(
    &vec![
      "language/import/user_import/symbol.tube",
      "language/import/user_import/module.tube",
    ],
    "language/import/user_import",
    VmExit::Ok,
  )?;

  test_file_exits(
    &vec![
      "language/import/import_in_fun.tube",
      "language/import/import_in_class.tube",
      "language/import/import_in_scope.tube",
      "language/import/missing_path.tube",
      "language/import/missing_semicolon.tube",
      "language/import/non_identifier_path.tube",
      "language/import/rename_missing.tube",
      "language/import/rename_not_identifer.tube",
      "language/import/rename_redefine.tube",
      "language/import/symbols_redefine.tube",
      "language/import/symbols_rename_missing.tube",
      "language/import/symbols_rename_not_identifer.tube",
    ],
    VmExit::CompileError,
  )?;

  test_file_exits(
    &vec![
      "language/import/module_not_real.tube",
      "language/import/symbols_not_real.tube",
    ],
    VmExit::RuntimeError,
  )?;

  test_file_exits_with_cwd(
    &vec!["language/import/user_import/cycle_1.tube"],
    "language/import/user_import",
    VmExit::RuntimeError,
  )
}

#[test]
fn inheritance() -> Result<(), std::io::Error> {
  test_file_exits(
    &vec![
      "language/inheritance/constructor.tube",
      "language/inheritance/inherit_methods.tube",
      "language/inheritance/set_fields_from_base_class.tube",
    ],
    VmExit::Ok,
  )?;

  test_file_exits(
    &vec!["language/inheritance/parenthesized_superclass.tube"],
    VmExit::CompileError,
  )?;

  test_file_exits(
    &vec![
      "language/inheritance/inherit_from_function.tube",
      "language/inheritance/inherit_from_nil.tube",
      "language/inheritance/inherit_from_number.tube",
    ],
    VmExit::RuntimeError,
  )
}

#[test]
fn inline_cache() -> Result<(), std::io::Error> {
  test_file_exits(
    &vec![
      "language/inline_cache/invoke_thrash.tube",
      "language/inline_cache/property_get_thrash.tube",
      "language/inline_cache/property_set_thrash.tube",
      "language/inline_cache/super_invoke_thrash.tube",
    ],
    VmExit::Ok,
  )?;

  test_file_exits(&vec![], VmExit::CompileError)?;

  test_file_exits(&vec![], VmExit::RuntimeError)
}

#[test]
fn iterator() -> Result<(), std::io::Error> {
  test_file_exits(
    &vec![
      "language/iterator/equality.tube",
      "language/iterator/assign_iter_keep_state.tube",
    ],
    VmExit::Ok,
  )?;

  test_file_exits(&vec![], VmExit::CompileError)?;

  test_file_exits(&vec![], VmExit::RuntimeError)
}

#[test]
fn launch() -> Result<(), std::io::Error> {
  test_file_exits(
    &vec![
      "language/launch/launch_bound_method.tube",
      "language/launch/launch_exit.tube",
      "language/launch/launch_init.tube",
      "language/launch/launch_method.tube",
      "language/launch/launch_multi.tube",
      "language/launch/launch_multi_channel_join.tube",
      "language/launch/launch_native.tube",
      "language/launch/launch_single.tube",
      "language/launch/launch_with_capture_global.tube",
      "language/launch/launch_with_capture_local.tube",
    ],
    VmExit::Ok,
  )?;

  test_file_exits(
    &vec![
      "language/launch/no_call.tube",
      "language/launch/no_expr.tube",
      "language/launch/no_semi.tube",
    ],
    VmExit::CompileError,
  )?;

  test_file_exits(
    &vec!["language/launch/launch_error.tube"],
    VmExit::RuntimeError,
  )
}

#[test]
fn limit() -> Result<(), std::io::Error> {
  test_file_exits(&vec!["language/limit/reuse_constants.tube"], VmExit::Ok)?;

  test_file_exits(
    &vec![
      "language/limit/loop_too_large.tube",
      "language/limit/too_many_constants.tube",
      "language/limit/too_many_locals.tube",
      "language/limit/too_many_captures.tube",
    ],
    VmExit::CompileError,
  )?;

  test_file_exits(
    &vec!["language/limit/stack_overflow.tube"],
    VmExit::RuntimeError,
  )
}

#[test]
fn lambda() -> Result<(), std::io::Error> {
  test_file_exits(
    &vec![
      "language/lambda/expression_body.tube",
      "language/lambda/empty_body.tube",
      "language/lambda/mutual_recursion.tube",
      "language/lambda/recursion.tube",
      "language/lambda/parameters.tube",
      "language/lambda/str.tube",
    ],
    VmExit::Ok,
  )?;

  test_file_exits(
    &vec![
      "language/lambda/local_recursion.tube",
      "language/lambda/body_must_be_block_or_expr.tube",
      "language/lambda/missing_comma_in_parameters.tube",
      "language/lambda/too_many_parameters.tube",
      "language/lambda/too_many_arguments.tube",
      "language/lambda/local_mutual_recursion.tube",
    ],
    VmExit::CompileError,
  )?;

  test_file_exits(
    &vec![
      "language/lambda/extra_arguments.tube",
      "language/lambda/missing_arguments.tube",
    ],
    VmExit::RuntimeError,
  )
}

#[test]
fn list() -> Result<(), std::io::Error> {
  test_file_exits(
    &vec![
      "language/list/empty.tube",
      "language/list/homogeneous.tube",
      "language/list/mixed.tube",
    ],
    VmExit::Ok,
  )?;

  test_file_exits(
    &vec![
      "language/list/missing_comma_in_initializer.tube",
      "language/list/missing_closing_bracket.tube",
    ],
    VmExit::CompileError,
  )?;

  test_file_exits(&vec![], VmExit::RuntimeError)
}

#[test]
fn logical_operator() -> Result<(), std::io::Error> {
  test_file_exits(
    &vec![
      "language/logical_operator/and.tube",
      "language/logical_operator/and_truth.tube",
      "language/logical_operator/nested_ternary.tube",
      "language/logical_operator/or.tube",
      "language/logical_operator/or_truth.tube",
      "language/logical_operator/ternary.tube",
    ],
    VmExit::Ok,
  )?;

  test_file_exits(&vec![], VmExit::CompileError)?;

  test_file_exits(&vec![], VmExit::RuntimeError)
}

#[test]
fn map() -> Result<(), std::io::Error> {
  test_file_exits(
    &vec![
      "language/map/empty.tube",
      "language/map/homogeneous.tube",
      "language/map/mixed.tube",
    ],
    VmExit::Ok,
  )?;

  test_file_exits(
    &vec![
      "language/map/missing_closing_curly.tube",
      "language/map/missing_colon.tube",
      "language/map/statement_key.tube",
      "language/map/statement_value.tube",
    ],
    VmExit::CompileError,
  )?;

  test_file_exits(&vec![], VmExit::RuntimeError)
}

#[test]
fn method() -> Result<(), std::io::Error> {
  test_file_exits(
    &vec![
      "language/method/arity.tube",
      "language/method/empty_block.tube",
      "language/method/print_bound_method.tube",
    ],
    VmExit::Ok,
  )?;

  test_file_exits(
    &vec![
      "language/method/too_many_arguments.tube",
      "language/method/too_many_parameters.tube",
      "language/method/refer_to_name.tube",
    ],
    VmExit::CompileError,
  )?;

  test_file_exits(
    &vec![
      "language/method/extra_arguments.tube",
      "language/method/missing_arguments.tube",
      "language/method/not_found.tube",
    ],
    VmExit::RuntimeError,
  )
}

#[test]
fn native() -> Result<(), std::io::Error> {
  test_file_exits(
    &vec![
      "language/native/assert.tube",
      "language/native/assert_eq.tube",
      "language/native/assert_ne.tube",
      "language/native/clock.tube",
      "language/native/signature_fixed_arity.tube",
      "language/native/signature_type.tube",
    ],
    VmExit::Ok,
  )?;

  test_file_exits(&vec![], VmExit::CompileError)?;

  test_file_exits(&vec![], VmExit::RuntimeError)
}

#[test]
fn nil() -> Result<(), std::io::Error> {
  test_file_exits(&vec!["language/nil/literal.tube"], VmExit::Ok)?;

  test_file_exits(&vec![], VmExit::CompileError)?;

  test_file_exits(&vec![], VmExit::RuntimeError)
}

#[test]
fn number() -> Result<(), std::io::Error> {
  test_file_exits(&vec!["language/number/literals.tube"], VmExit::Ok)?;

  test_file_exits(
    &vec![
      "language/number/decimal_point_at_eof.tube",
      "language/number/leading_dot.tube",
      "language/number/trailing_dot.tube",
    ],
    VmExit::CompileError,
  )?;

  test_file_exits(&vec![], VmExit::RuntimeError)
}

#[test]
fn operator() -> Result<(), std::io::Error> {
  test_file_exits(
    &vec![
      "language/operator/add.tube",
      "language/operator/comparison.tube",
      "language/operator/divide.tube",
      "language/operator/equals_class.tube",
      "language/operator/equals_method.tube",
      "language/operator/equals.tube",
      "language/operator/multiply.tube",
      "language/operator/negate.tube",
      "language/operator/not_class.tube",
      "language/operator/not_equals.tube",
      "language/operator/not.tube",
      "language/operator/subtract.tube",
    ],
    VmExit::Ok,
  )?;

  test_file_exits(&vec![], VmExit::CompileError)?;

  test_file_exits(
    &vec![
      "language/operator/add_bool_nil.tube",
      "language/operator/add_bool_num.tube",
      "language/operator/add_bool_string.tube",
      "language/operator/add_nil_nil.tube",
      "language/operator/add_num_nil.tube",
      "language/operator/add_string_nil.tube",
      "language/operator/divide_nonnum_num.tube",
      "language/operator/divide_num_nonnum.tube",
      "language/operator/greater_nonnum_num.tube",
      "language/operator/greater_num_nonnum.tube",
      "language/operator/greater_or_equal_nonnum_num.tube",
      "language/operator/greater_or_equal_num_nonnum.tube",
      "language/operator/less_nonnum_num.tube",
      "language/operator/less_num_nonnum.tube",
      "language/operator/less_or_equal_nonnum_num.tube",
      "language/operator/less_or_equal_num_nonnum.tube",
      "language/operator/multiply_nonnum_num.tube",
      "language/operator/multiply_num_nonnum.tube",
      "language/operator/negate_nonnum.tube",
      "language/operator/subtract_nonnum_num.tube",
      "language/operator/subtract_num_nonnum.tube",
    ],
    VmExit::RuntimeError,
  )
}

#[test]
fn regression() -> Result<(), std::io::Error> {
  test_file_exits(
    &vec![
      "language/regression/40.tube",
      "language/regression/394.tube",
      "language/regression/continue.tube",
    ],
    VmExit::Ok,
  )?;

  test_file_exits(
    &vec!["language/regression/missing_symbol.tube"],
    VmExit::CompileError,
  )
}

#[test]
fn return_test() -> Result<(), std::io::Error> {
  test_file_exits(
    &vec![
      "language/return/after_else.tube",
      "language/return/after_if.tube",
      "language/return/after_while.tube",
      "language/return/in_function.tube",
      "language/return/return_nil_if_no_value.tube",
      "language/return/return_nil_if_no_value.tube",
    ],
    VmExit::Ok,
  )?;

  test_file_exits(
    &vec!["language/return/at_top_level.tube"],
    VmExit::CompileError,
  )?;

  test_file_exits(&vec![], VmExit::RuntimeError)
}


#[test]
fn raise() -> Result<(), std::io::Error> {
  test_file_exits(
    &vec![
      "language/raise/raise_error.tube",
    ],
    VmExit::Ok,
  )?;

  test_file_exits(
    &vec![
      "language/raise/no_expr.tube",
      "language/raise/no_semi.tube",
    ],
    VmExit::CompileError,
  )?;

  test_file_exits(
    &vec![
      "language/raise/not_instance.tube",
      "language/raise/not_error_subclass.tube",
    ],
    VmExit::RuntimeError,
  )
}

#[test]
fn static_method() -> Result<(), std::io::Error> {
  test_file_exits(
    &vec![
      "language/static_method/arity.tube",
      "language/static_method/call_bound.tube",
      "language/static_method/empty_block.tube",
      "language/static_method/print_bound_method.tube",
    ],
    VmExit::Ok,
  )?;

  test_file_exits(
    &vec![
      "language/static_method/no_self.tube",
      "language/static_method/no_self_implicit.tube",
      "language/static_method/not_inherited.tube",
      "language/static_method/too_many_arguments.tube",
      "language/static_method/too_many_parameters.tube",
      "language/static_method/refer_to_name.tube",
    ],
    VmExit::CompileError,
  )?;

  test_file_exits(
    &vec![
      "language/static_method/extra_arguments.tube",
      "language/static_method/missing_arguments.tube",
      "language/static_method/not_found.tube",
    ],
    VmExit::RuntimeError,
  )
}

#[test]
fn string() -> Result<(), std::io::Error> {
  test_file_exits(
    &vec![
      "language/string/interpolation.tube",
      "language/string/literals.tube",
      "language/string/multiline.tube",
      "language/string/escape.tube",
      "language/string/unicode_escape.tube",
    ],
    VmExit::Ok,
  )?;

  test_file_exits(
    &vec![
      "language/string/invalid_escape.tube",
      "language/string/invalid_interpolation_missing_close.tube",
      "language/string/invalid_unicode_hex.tube",
      "language/string/invalid_unicode_missing_close.tube",
      "language/string/invalid_unicode_missing_open.tube",
      "language/string/invalid_unicode_no_hex.tube",
      "language/string/invalid_unicode_too_long.tube",
      "language/string/unterminated_double.tube",
      "language/string/unterminated_single.tube",
      "language/string/error_after_multiline.tube",
    ],
    VmExit::CompileError,
  )?;

  test_file_exits(&vec![], VmExit::RuntimeError)
}

#[test]
fn super_() -> Result<(), std::io::Error> {
  test_file_exits(
    &vec![
      "language/super/bound_method.tube",
      "language/super/call_other_method.tube",
      "language/super/call_same_method.tube",
      "language/super/closure.tube",
      "language/super/constructor.tube",
      "language/super/indirectly_inherited.tube",
      "language/super/reassign_superclass.tube",
      "language/super/super_in_closure_in_inherited_method.tube",
      "language/super/super_in_inherited_method.tube",
    ],
    VmExit::Ok,
  )?;

  test_file_exits(
    &vec![
      "language/super/parenthesized.tube",
      "language/super/super_at_top_level.tube",
      "language/super/super_in_top_level_function.tube",
      "language/super/super_without_dot.tube",
      "language/super/super_without_name.tube",
    ],
    VmExit::CompileError,
  )?;

  test_file_exits(
    &vec![
      "language/super/extra_arguments.tube",
      "language/super/missing_arguments.tube",
      "language/super/no_superclass_method.tube",
    ],
    VmExit::RuntimeError,
  )
}

#[test]
fn variable() -> Result<(), std::io::Error> {
  test_file_exits(
    &vec![
      "language/variable/early_bound.tube",
      "language/variable/in_middle_of_block.tube",
      "language/variable/in_nested_block.tube",
      "language/variable/local_from_method.tube",
      "language/variable/scope_reuse_in_different_blocks.tube",
      "language/variable/shadow_and_local.tube",
      "language/variable/shadow_global.tube",
      "language/variable/shadow_local.tube",
      "language/variable/uninitialized.tube",
    ],
    VmExit::Ok,
  )?;

  test_file_exits(
    &vec![
      "language/variable/collide_with_parameter.tube",
      "language/variable/duplicate_local.tube",
      "language/variable/duplicate_parameter.tube",
      "language/variable/redeclare_global.tube",
      "language/variable/redefine_global.tube",
      "language/variable/undefined_global.tube",
      "language/variable/undefined_local.tube",
      "language/variable/unreached_undefined.tube",
      "language/variable/use_false_as_var.tube",
      "language/variable/use_global_in_initializer.tube",
      "language/variable/use_local_in_initializer.tube",
      "language/variable/use_nil_as_var.tube",
      "language/variable/use_this_as_var.tube",
    ],
    VmExit::CompileError,
  )?;

  test_file_exits(&vec![], VmExit::RuntimeError)
}

#[test]
fn self_() -> Result<(), std::io::Error> {
  test_file_exits(
    &vec![
      "language/self/closure.tube",
      "language/self/nested_class.tube",
      "language/self/nested_closure.tube",
      "language/self/pass_through_assign_implicit.tube",
      "language/self/pass_through_assign.tube",
      "language/self/self_in_method.tube",
    ],
    VmExit::Ok,
  )?;

  test_file_exits(
    &vec![
      "language/self/self_at_top_level.tube",
      "language/self/self_in_top_level_function.tube",
    ],
    VmExit::CompileError,
  )?;

  test_file_exits(&vec![], VmExit::RuntimeError)
}

#[test]
fn tuple() -> Result<(), std::io::Error> {
  test_file_exits(
    &vec![
      "language/tuple/empty.tube",
      "language/tuple/homogeneous.tube",
      "language/tuple/mixed.tube",
    ],
    VmExit::Ok,
  )?;

  test_file_exits(
    &vec![
      "language/tuple/empty_comma.tube",
      "language/tuple/missing_comma_in_initializer.tube",
      "language/tuple/missing_closing_bracket.tube",
    ],
    VmExit::CompileError,
  )?;

  test_file_exits(&vec![], VmExit::RuntimeError)
}

#[test]
fn while_test() -> Result<(), std::io::Error> {
  test_file_exits(
    &vec![
      "language/while/closure_in_body.tube",
      "language/while/return_closure.tube",
      "language/while/return_inside.tube",
      "language/while/syntax.tube",
    ],
    VmExit::Ok,
  )?;

  test_file_exits(
    &vec![
      "language/while/class_in_body.tube",
      "language/while/fun_in_body.tube",
      "language/while/var_in_body.tube",
    ],
    VmExit::CompileError,
  )?;

  test_file_exits(&vec![], VmExit::RuntimeError)
}
