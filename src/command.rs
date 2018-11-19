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
    type Err = RshError;

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

pub fn rsh_execute(args: Vec<String>) -> Result<Status, RshError> {
    if args.is_empty() {
        return Ok(Status::NoCommand);
    }

    Commands::from_str(&args[0])
        .and_then(|c| match c {
            Commands::Cd      => rsh_cd(args),
            Commands::Help    => rsh_help(),
            Commands::Exit    => rsh_exit(),
            Commands::Execute => rsh_spawn(args),
        })
}

fn rsh_cd(arg: Vec<String>) -> Result<Status, RshError> {
    if arg.len() == 1 {
        return Err(RshError::CommandError("rsh: expected argument to \"cd\"".to_string()));
    } else {
        let path = Path::new(&arg[1]);
        chdir(path)
            .map(|_| Status::Success)
            .map_err(|e| RshError::CommandError(e.to_string()))
    }
}

fn rsh_help() -> Result<Status, RshError> {
    println!("Type program names and arguments, and hit enter.");
    println!("The following are build in: ");
    BUILTINS.iter().for_each(|c| println!("  {}", c));
    Ok(Status::Success)
}

fn rsh_exit() -> Result<Status, RshError> {
    Ok(Status::Exit)
} 

fn rsh_spawn(args: Vec<String>) -> Result<Status, RshError> {
    let pid = fork().map_err(|e| RshError::ForkError(e.to_string()))?;

    match pid {
        ForkResult::Parent { child } => {
            let wpid = waitpid(child, None)
                .map_err(|e| RshError::ParentError(e.to_string()));
            
            match wpid {
                Ok(WaitStatus::Exited(_, _))      => Ok(Status::Success),
                Ok(WaitStatus::Signaled(_, _, _)) => Ok(Status::Success),
                Err(e)                            => Err(RshError::ParentError(e.to_string())),
                _                                 => Ok(Status::Success),
            }
        }
        ForkResult::Child => {
            let cmd = args.iter()
                .map(|s| CString::new(s.to_string()).expect("Error creating CString"))
                .collect::<Vec<_>>();

            execvp(&cmd[0], &cmd)
                .map(|_| Status::Success)
                .map_err(|e| RshError::ChildError(e.to_string()))
        }
    }
}
