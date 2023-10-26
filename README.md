# DSAM
**Dreamer's Simple Argument Manager (DSAM, for short) is a simplistic crate that provides a better experience when dealing with command line arguments.**

**It provides a set of functions that allow to better deal with these arguments, as well as make it easy to add new commands and use them.**

## What does it allow me to do?
- **Define how many arguments the command will have**
- **Define range-based arguments**
- **Define the name of the command**
- **Easily obtain an argument from it's index**

## What are range-based arguments?
**To put it simply, the amount of arguments a command can have becomes a range, which reduces the amount of code required to define commands with many arguments.**

## How do I install this crate?
**Open your `Cargo.toml` file and add the following line under `[dependencies]`:**
```toml
```

## Example
```rust
use dsam::*;

fn main() {
    let mut commands: Vec<Command> = Vec::new();

    commands.push(Command::new("command1", vec![1], false)); 
    commands.push(Command::new("command2", vec![1, 2], false));
    commands.push(Command::new("command3", vec![0], false));
    commands.push(Command::new("command4", vec![1, 5], true));

    let manager = ArgumentManager::new(commands);

    if !manager.is_command_valid() {
        panic!("Either the command is invalid or the amount of arguments is invalid.");
    }

    let command: &String = &manager.get_element(0);

    match command.as_str() {
        "command1" => {
            println!("Command 1 executed.");
        }
        "command2" => {
            println!("Command 2 executed.");
        }
        "command3" => {
            println!("Command 3 executed.");
        }
        "command4" => {
            println!("Command 4 executed.");
        }
        &_ => {
            panic!("Unknown command.");
        }
    }
}
```
