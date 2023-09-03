use std::env;

pub struct Command {
    command_name: String,
    argc: Vec<i32>,
    range: bool,
}

impl Command {
    pub fn new(command_name_: &str,mut argc_: Vec<i32>,range_: bool) -> Self {
        if argc_.is_empty() { panic!("Command requires number of parameters but none were provided."); }
        if range_ && argc_.len() % 2 != 0 { panic!("Command considered range but some range was left incomplete."); }
        if !range_{
            for i in 0..argc_.len() {
                for j in (argc_.len() - 1..=0).rev() {
                    if i != j && argc_[i] == argc_[j] { argc_.remove(i); }
                }
            }
        }

        Command {
            command_name: command_name_.to_string(),
            argc: argc_,
            range: range_,
        }
    }

    fn get_command(v: &Vec<Self>,command_name: String) -> &Self {
        for comm in v { if comm.command_name == command_name { return comm; } }
        panic!("Unknown command. Exiting.");
    }
}

pub struct ArgumentManager {
    commands: Vec<Command>,
    args: Vec<String>,
}

impl ArgumentManager {
    pub fn new(mut commands_: Vec<Command>) -> Self {
        for i in 0..commands_.len() {
            for j in (commands_.len() - 1..=0).rev() {
                if i != j && commands_[i].command_name == commands_[j].command_name { commands_.remove(i); }
            }
        }

        ArgumentManager {
            commands: commands_,
            args: env::args().collect(),
        }
    }

    pub fn is_command_valid(&self) -> bool {
        if self.args.is_empty() || self.args.len() < 2 { return false; }

        let command_name = self.args[1].clone();
        let command = Command::get_command(&self.commands,command_name);
        let arg_count = self.args.len() - 2;

        if command.range {
            for i in (0..command.argc.len()).step_by(2) {
                if (arg_count as i32) >= command.argc[i] && (arg_count as i32) <= command.argc[i + 1] { return true; }
            }
            false
        } else { command.argc.contains(&(arg_count as i32)) }
    }

    pub fn get_element(&self,index: usize) -> String {
        if !self.is_command_valid() { panic!("Command not valid. Exiting."); }

        self.args[index + 1].clone()
    }
}