use error::*;
use status::*;

use std::fmt;
use std::ffi::CString;
use std::path::*;
use std::str::FromStr;
use nix::unistd::*;
use nix::sys::wait::*;

const BUILTINS: [&str; 3] = ["cd", "help", "exit"];

pub enum Commands {
    Cd,
    Help,
    Exit,
    Execute,
}

impl FromStr for Commands {
    type Err = LshError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cd"   => Ok(Commands::Cd),
            "help" => Ok(Commands::Help),
            "exit" => Ok(Commands::Exit),
            _      => Ok(Commands::Execute),
        }
    }
}

impl fmt::Display for Commands {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

pub fn lsh_execute(args: Vec<String>) -> Result<Status, LshError> {
    if args.is_empty() {
        return Ok(Status::NoCommand);
    }

    Commands::from_str(&args[0])
        .and_then(|c| match c {
            Commands::Cd      => lsh_cd(args),
            Commands::Help    => lsh_help(),
            Commands::Exit    => lsh_exit(),
            Commands::Execute => lsh_spawn(args),
        })
}

fn lsh_cd(arg: Vec<String>) -> Result<Status, LshError> {
    if arg.len() == 1 {
        return Err(LshError::CommandError("lsh: expected argument to \"cd\"".to_string()));
    } else {
        let path = Path::new(&arg[1]);
        chdir(path)
            .map(|_| Status::Success)
            .map_err(|e| LshError::CommandError(e.to_string()))
    }
}

fn lsh_help() -> Result<Status, LshError> {
    println!("Type program names and arguments, and hit enter.");
    println!("The following are build in: ");
    BUILTINS.iter().for_each(|c| println!("  {}", c));
    Ok(Status::Success)
}

fn lsh_exit() -> Result<Status, LshError> {
    Ok(Status::Exit)
} 

fn lsh_spawn(args: Vec<String>) -> Result<Status, LshError> {
    let pid = fork().map_err(|e| LshError::ForkError(e.to_string()))?;

    match pid {
        ForkResult::Parent { child } => {
            let wpid = waitpid(child, None)
                .map_err(|e| LshError::ParentError(e.to_string()));
            
            match wpid {
                Ok(WaitStatus::Exited(_, _))      => Ok(Status::Success),
                Ok(WaitStatus::Signaled(_, _, _)) => Ok(Status::Success),
                Err(e)                            => Err(LshError::ParentError(e.to_string())),
                _                                 => Ok(Status::Success),
            }
        }
        ForkResult::Child => {
            let cmd = args.iter()
                .map(|s| CString::new(s.to_string()).expect("Error creating CString"))
                .collect::<Vec<_>>();

            execvp(&cmd[0], &cmd)
                .map(|_| Status::Success)
                .map_err(|e| LshError::ChildError(e.to_string()))
        }
    }
}
