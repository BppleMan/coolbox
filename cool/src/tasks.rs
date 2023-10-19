use serde::{Deserialize, Serialize};

pub use command::*;
pub use compress::*;
pub use copy::*;
pub use decompress::*;
pub use delete::*;
pub use download::*;
pub use git::*;
pub use install::*;
pub use r#move::*;

use crate::result::CoolResult;
use crate::state::StateAble;

mod command;
mod compress;
mod copy;
mod decompress;
mod delete;
mod download;
mod git;
mod install;
mod r#move;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum ExecutableState {
    #[default]
    NotStarted,
    Running,
    Finished,
    Error,
}

pub trait Executable: StateAble {
    fn execute(&mut self) -> CoolResult<()> {
        *self.current_state() = ExecutableState::Running;
        match self._run() {
            Ok(_) => {
                *self.current_state() = ExecutableState::Finished;
                Ok(())
            }
            Err(e) => {
                *self.current_state() = ExecutableState::Error;
                Err(e)
            }
        }
    }

    fn _run(&mut self) -> CoolResult<()>;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Task {
    Command(Command),
    Compress(Compress),
    Copy(Copy),
    Decompress(Decompress),
    Delete(Delete),
    Install(Install),
    Move(Move),
    Git(Git),
}

impl AsRef<dyn Executable> for Task {
    fn as_ref(&self) -> &(dyn Executable + 'static) {
        match self {
            Task::Copy(copy) => copy,
            Task::Command(command) => command,
            Task::Compress(compress) => compress,
            Task::Decompress(decompress) => decompress,
            Task::Delete(delete) => delete,
            Task::Install(install) => install,
            Task::Move(r#move) => r#move,
            Task::Git(git) => git,
        }
    }
}

impl AsMut<dyn Executable> for Task {
    fn as_mut(&mut self) -> &mut (dyn Executable + 'static) {
        match self {
            Task::Copy(copy) => copy,
            Task::Command(command) => command,
            Task::Compress(compress) => compress,
            Task::Decompress(decompress) => decompress,
            Task::Delete(delete) => delete,
            Task::Install(install) => install,
            Task::Move(r#move) => r#move,
            Task::Git(git) => git,
        }
    }
}

impl StateAble for Task {
    fn current_state(&mut self) -> &mut ExecutableState {
        self.as_mut().current_state()
    }

    fn outputs(&mut self) -> &mut Vec<String> {
        self.as_mut().outputs()
    }

    fn errors(&mut self) -> &mut Vec<String> {
        self.as_mut().errors()
    }
}

impl Executable for Task {
    fn _run(&mut self) -> CoolResult<()> {
        self.as_mut()._run()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Tasks(pub Vec<Task>);

impl Tasks {
    pub fn execute(&mut self) -> CoolResult<Vec<Vec<String>>> {
        self.0.iter_mut().try_fold(Vec::new(), |mut results, task| {
            task.as_mut().execute()?;
            results.push(task.outputs().clone());
            Ok(results)
        })
    }
}
