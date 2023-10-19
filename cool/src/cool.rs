use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::thread;

use color_eyre::eyre::eyre;
use color_eyre::Report;
use crossbeam::channel::Receiver;
use lazy_static::lazy_static;
use once_cell::sync::Lazy;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::error::InstallError;
use crate::result::CoolResult;
use crate::tasks::{Download, Tasks};

pub static COOL_LIST: Lazy<Arc<RwLock<HashMap<String, Cool>>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    let brew_cool = Cool {
        name: "homebrew".to_string(),
        description: "适用于macOS的包管理器。它使您能够从命令行安装和更新软件包，从而使您的Mac保持最新状态，而无需使用App Store。".to_string(),
        dependencies: vec![],
        macos: Some(PlatformTasks {
            install_tasks: Tasks(vec![
                // Download::new("https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh", format!("{}/install.sh", std::env::temp_dir().to_str().unwrap())),
            ]),
            uninstall_tasks: Tasks(vec![]),
            check_tasks: Tasks(vec![]),
        }),
        linux: None,
        windows: None,
    };
    map.insert(brew_cool.name.clone(), brew_cool);

    Arc::new(RwLock::new(map))
});

lazy_static! {
    static ref INSTALLING: Arc<RwLock<HashMap<String, Receiver<()>>>> =
        Arc::new(RwLock::new(HashMap::new()));
    static ref UNINSTALLING: Arc<RwLock<HashMap<String, Receiver<()>>>> =
        Arc::new(RwLock::new(HashMap::new()));
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Cool {
    pub name: String,
    pub description: String,
    pub dependencies: Vec<String>,
    pub macos: Option<PlatformTasks>,
    pub linux: Option<PlatformTasks>,
    pub windows: Option<PlatformTasks>,
}

impl Cool {
    pub fn new(
        name: String,
        description: String,
        dependencies: Vec<String>,
        macos: Option<PlatformTasks>,
        linux: Option<PlatformTasks>,
        windows: Option<PlatformTasks>,
    ) -> Self {
        Self {
            name,
            description,
            dependencies,
            macos,
            linux,
            windows,
        }
    }

    fn current_platform_tasks(&self) -> CoolResult<&PlatformTasks> {
        let mut platform_tasks = if cfg!(macos) {
            &self.macos
        } else if cfg!(linux) {
            &self.linux
        } else if cfg!(windows) {
            &self.windows
        } else {
            return Err(eyre!("unsupported platform"));
        };

        platform_tasks
            .as_ref()
            .take()
            .ok_or_else(|| eyre!("{} is not supported on this platform", self.name))
    }

    pub fn install(&mut self) -> CoolResult<Vec<Vec<String>>> {
        let name = self.name.clone();
        if INSTALLING.read().unwrap().contains_key(&name) {
            return Err(Report::new(InstallError::AlreadyInstalling(name)));
        }

        info!("installing {}", name);

        if UNINSTALLING.read().unwrap().contains_key(&name) {
            info!("waiting for {} to be uninstalled", name);
            UNINSTALLING.read().unwrap()[&name].recv()?;
        }

        self.install_dependencies()?;

        let platform_tasks = self.current_platform_tasks()?;
        let mut tasks = platform_tasks.install_tasks.clone();
        let handle = thread::spawn(move || tasks.execute());

        let (sender, receiver) = crossbeam::channel::bounded(1);

        INSTALLING
            .write()
            .unwrap()
            .insert(name.clone(), receiver.clone());

        let result = handle.join().unwrap()?;
        sender.send(())?;
        INSTALLING.write().unwrap().remove(&name);

        Ok(result)
    }

    pub fn uninstall(&mut self) -> CoolResult<Vec<Vec<String>>> {
        let name = self.name.clone();
        if UNINSTALLING.read().unwrap().contains_key(&name) {
            return Err(Report::new(InstallError::AlreadyUninstalling(name)));
        }

        info!("uninstalling {}", name);

        if INSTALLING.read().unwrap().contains_key(&name) {
            info!("waiting for {} to be installed", name);
            INSTALLING.read().unwrap()[&name].recv()?;
        }

        let platform_tasks = self.current_platform_tasks()?;
        let mut tasks = platform_tasks.uninstall_tasks.clone();
        let handle = thread::spawn(move || tasks.execute());

        let (sender, receiver) = crossbeam::channel::bounded(1);

        UNINSTALLING
            .write()
            .unwrap()
            .insert(name.clone(), receiver.clone());

        let result = handle.join().unwrap()?;
        sender.send(())?;
        UNINSTALLING.write().unwrap().remove(&name);

        Ok(result)
    }

    fn install_dependencies(&self) -> CoolResult<Vec<Vec<String>>> {
        let results = self
            .dependencies
            .par_iter()
            .map(|d| {
                if let Some(cool) = COOL_LIST.write().unwrap().get_mut(d) {
                    Ok(cool.install()?)
                } else {
                    Err(eyre!("{} not found", d))
                }
            })
            .try_fold(Vec::new, |mut results, result| match result {
                Ok(result) => {
                    results.extend(result);
                    Ok(results)
                }
                Err(e) => Err(e),
            })
            .try_reduce(Vec::new, |mut results, result| {
                results.extend(result);
                Ok(results)
            })?;
        Ok(results)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlatformTasks {
    pub install_tasks: Tasks,
    pub uninstall_tasks: Tasks,
    pub check_tasks: Tasks,
}
