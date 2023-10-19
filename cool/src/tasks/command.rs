use color_eyre::eyre::eyre;
// use serde::{Deserialize, Serialize};

use crate::result::CoolResult;
use crate::shell::{Shell, ShellExecutor, ShellResult};
use crate::state::StateAble;
use crate::tasks::{Executable, ExecutableState};

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Command {
    #[serde(deserialize_with = "crate::render_str")]
    pub script: String,
    pub args: Option<Vec<String>>,
    pub envs: Option<Vec<(String, String)>>,
    pub shell: Shell,

    #[serde(skip)]
    state: ExecutableState,
    #[serde(skip)]
    outputs: Vec<String>,
    #[serde(skip)]
    errors: Vec<String>,
}

impl Command {
    pub fn new(
        script: String,
        args: Option<Vec<String>>,
        envs: Option<Vec<(String, String)>>,
        shell: Shell,
    ) -> Self {
        Self {
            script,
            args,
            envs,
            shell,
            state: ExecutableState::NotStarted,
            outputs: vec![],
            errors: vec![],
        }
    }
}

impl StateAble for Command {
    fn current_state(&mut self) -> &mut ExecutableState {
        &mut self.state
    }

    fn outputs(&mut self) -> &mut Vec<String> {
        &mut self.outputs
    }

    fn errors(&mut self) -> &mut Vec<String> {
        &mut self.errors
    }
}

impl Executable for Command {
    fn _run(&mut self) -> CoolResult<()> {
        let initial_result: CoolResult<()> = Err(eyre!("No attempts made"));

        (0..5).fold(initial_result, |acc, _| {
            if let Err(_) = acc {
                let ShellResult {
                    input: _input,
                    output,
                    error,
                } = self.shell.run(
                    &self.script,
                    self.args
                        .as_ref()
                        .map(|args| args.iter().map(AsRef::as_ref).collect::<Vec<_>>())
                        .as_deref(),
                    self.envs
                        .as_ref()
                        .map(|envs| {
                            envs.iter()
                                .map(|(k, v)| (k.as_str(), v.as_str()))
                                .collect::<Vec<_>>()
                        })
                        .as_deref(),
                )?;
                rayon::scope(|s| {
                    s.spawn(|_| {
                        while let Ok(r) = output.recv() {
                            self.outputs.push(r);
                        }
                    });
                    s.spawn(|_| {
                        while let Ok(r) = error.recv() {
                            self.errors.push(r);
                        }
                    });
                });
            }
            Ok(())
        })
    }
}

#[cfg(test)]
mod test {
    use crate::init_backtrace;
    use crate::result::CoolResult;
    use crate::shell::{Sh, Shell};
    use crate::tasks::{Command, Executable};

    #[test]
    fn test_serialize() -> CoolResult<()> {
        init_backtrace();

        let mut expect = Command::new("echo hello".to_string(), None, None, Shell::Sh(Sh));
        let toml = toml::to_string(&expect)?;
        let command: Command = toml::from_str(&toml)?;
        pretty_assertions::assert_eq!(expect, command);

        expect.execute()?;
        pretty_assertions::assert_eq!("hello\n".to_string(), expect.outputs.join("\n"));
        Ok(())
    }

    #[test]
    fn ping() -> CoolResult<()> {
        init_backtrace();

        let mut command = Command::new(
            "ping -c 1 www.baidu.com".to_string(),
            None,
            None,
            Shell::Sh(Sh),
        );
        let result = command.execute();
        assert!(result.is_ok());
        Ok(())
    }
}
