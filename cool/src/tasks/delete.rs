use serde::{Deserialize, Serialize};

use crate::result::CoolResult;
use crate::state::StateAble;
use crate::tasks::{Executable, ExecutableState};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Delete {
    #[serde(deserialize_with = "crate::render_str")]
    pub path: String,

    #[serde(skip)]
    state: ExecutableState,
    #[serde(skip)]
    outputs: Vec<String>,
    #[serde(skip)]
    errors: Vec<String>,
}

impl Delete {
    pub fn new(path: String) -> Self {
        Self {
            path,
            state: ExecutableState::NotStarted,
            outputs: vec![],
            errors: vec![],
        }
    }
}

impl StateAble for Delete {
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

impl Executable for Delete {
    fn _run(&mut self) -> CoolResult<()> {
        fs_extra::remove_items(&[&self.path])?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::fs::File;
    use tempfile::{Builder, NamedTempFile};

    use crate::init_backtrace;
    use crate::result::CoolResult;
    use crate::tasks::delete::Delete;
    use crate::tasks::Executable;

    #[test]
    fn delete_file() -> CoolResult<()> {
        init_backtrace();
        let base_dir = Builder::new().prefix("cool").suffix("delete").tempdir()?;
        let path = NamedTempFile::new_in(base_dir.path())?;
        assert!(path.path().exists());
        Delete::new(path.path().to_string_lossy().to_string()).execute()?;
        assert!(!path.path().exists());
        Ok(())
    }

    #[test]
    fn delete_dir() -> CoolResult<()> {
        init_backtrace();
        let base_dir = Builder::new().prefix("cool").suffix("delete").tempdir()?;

        let source_dir = base_dir.path().join("source");
        fs_extra::dir::create(&source_dir, true)?;
        let _child_file = File::create(source_dir.join("child_file"))?;
        let child_dir = source_dir.join("child_dir");
        fs_extra::dir::create(&child_dir, true)?;
        let _child_file1 = File::create(child_dir.join("child_file1"))?;
        let _child_file2 = File::create(child_dir.join("child_file2"))?;

        Delete::new(source_dir.to_string_lossy().to_string()).execute()?;

        assert!(!source_dir.exists());

        Ok(())
    }
}
