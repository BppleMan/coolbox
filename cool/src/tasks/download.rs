use crate::result::CoolResult;
use crate::state::StateAble;
use crate::tasks::{Executable, ExecutableState};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Download {
    pub url: String,
    pub dest: String,

    #[serde(skip)]
    state: ExecutableState,
    #[serde(skip)]
    outputs: Vec<String>,
    #[serde(skip)]
    errors: Vec<String>,
}

impl Download {
    pub fn new(url: String, dest: String) -> Self {
        Self {
            url,
            dest,
            state: ExecutableState::NotStarted,
            outputs: vec![],
            errors: vec![],
        }
    }
}

impl StateAble for Download {
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

impl Executable for Download {
    fn _run(&mut self) -> CoolResult<()> {
        let mut bytes = reqwest::blocking::get(&self.url)?.bytes()?;
        std::fs::write(&self.dest, &mut bytes)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::init_backtrace;
    use crate::result::CoolResult;
    use crate::tasks::{Download, Executable, ExecutableState};
    use tempfile::{Builder, NamedTempFile};

    #[test]
    fn smoke() -> CoolResult<()> {
        init_backtrace();
        let mut server = mockito::Server::new();
        let url = server.url();

        // download server
        let mock = server
            .mock("GET", "/download")
            .with_status(200)
            .with_header("content-type", "binary/octet-stream")
            .with_body([0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
            .create();

        let base_dir = Builder::new().prefix("cool").suffix("download").tempdir()?;
        let path = NamedTempFile::new_in(base_dir.path())?;
        let mut download = Download::new(
            format!("{}/download", url),
            path.path().display().to_string(),
        );
        download.execute()?;
        mock.assert();
        pretty_assertions::assert_eq!(ExecutableState::Finished, download.state);
        assert!(path.path().exists());
        Ok(())
    }
}
