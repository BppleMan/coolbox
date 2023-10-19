use crate::result::CoolResult;
use crate::tasks::{Executable, ExecutableState};
use color_eyre::eyre::eyre;
use cool_macros::State;
use serde::{Deserialize, Serialize};
use which::which;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, State)]
pub struct Which {
    pub command: String,

    #[serde(skip)]
    state: ExecutableState,
    #[serde(skip)]
    outputs: Vec<String>,
    #[serde(skip)]
    errors: Vec<String>,
}

impl Which {
    pub fn new(command: String) -> Self {
        Self {
            command,
            state: ExecutableState::NotStarted,
            outputs: vec![],
            errors: vec![],
        }
    }
}

impl Executable for Which {
    fn _run(&mut self) -> CoolResult<()> {
        match which(&self.command) {
            Ok(result) => {
                self.outputs.push(result.to_string_lossy().to_string());
                Ok(())
            }
            Err(_) => {
                let msg = format!("{} not found", &self.command);
                self.errors.push(msg.clone());
                Err(eyre!(msg))
            }
        }
    }
}
