use crate::plugin::*;

pub mod plugin;

fn main() {
  let pkg = tools::ToolManager {
    name: "package-tools".to_string(),
    description: "package list with tool pkg.".to_string(),
  };
  pkg.run_tool();
  fmt::Console::log("Hello, world!\n");
}
