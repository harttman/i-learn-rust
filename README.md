- `fmt::Console`:
**pub struct Console** - *{} impl: pub -> log("Hello, world!")*
- `tools::ToolManager`:
**pub struct ToolManager** - *{ name: String, description: String } impl: -> pub run_package(&self)*

```rs
use crate::plugin::{tools::*, fmt::*};

pub mod plugin;

fn main() {
  let hello = ToolManager {
    name: String::from("Hello"),
    description: String::from("hay"),
  };
  hello.run_package();
  Console::log("Test - 123");
```
