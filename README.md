# Argument Manager
A simple one-file Rust library for better management of program arguments.

# How does it work?
**It is a bit hard to explain so I made a simple example for you to have a base:**

```rust
mod arg_manager;
use arg_manager::*;

fn main() {
    let mut commands: Vec<Command> = Vec::new();
    commands.push(Command::new("command1",vec![1],false));
    commands.push(Command::new("command2",vec![1,2],false));
    commands.push(Command::new("command3",vec![0],false));
    commands.push(Command::new("command4",vec![1,5,6],true));

    let manager = ArgumentManager::new(commands);

    if !manager.is_command_valid() { panic!("Either the command is invalid or the amount of arguments is invalid."); }

    let command: &String = &manager.get_element(0);

    match command.as_str() {
        "command1" => { println!("Command 1 executed."); }
        "command2" => { println!("Command 2 executed."); }
        "command3" => { println!("Command 3 executed."); }
        "command4" => { println!("Command 4 executed."); }
        &_ => { panic!("Unknown command."); }
    }
}
```
