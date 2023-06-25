mod clock;

use clock::declare_clock_funs;
use tube_core::{hooks::GcHooks, managed::Gc, module::Module};

use crate::StdResult;

pub(crate) fn add_clock_funs(hooks: &GcHooks, module: Gc<Module>) -> StdResult<()> {
  declare_clock_funs(hooks, module)
}
