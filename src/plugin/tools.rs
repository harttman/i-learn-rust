#[derive(Debug)]
pub struct ToolManager {
  pub name: String,
  pub description: String,
}

impl ToolManager {
  pub fn run_tool(&self) {
    let name = &self.name;
    println!("Run tool {name}...");
  }
}
