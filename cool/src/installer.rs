use std::fmt::Debug;
use std::hash::Hash;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub use apt::*;
pub use brew::*;
pub use cargo::*;
pub use dnf::*;
pub use rpm::*;
pub use yum::*;

use crate::result::CoolResult;
use crate::shell::ShellResult;

mod apt;
mod brew;
mod cargo;
mod dnf;
mod rpm;
mod yum;

pub trait Installable {
    fn install(&mut self, name: &str, args: Option<&[&str]>) -> CoolResult<ShellResult>;

    fn uninstall(&mut self, name: &str, args: Option<&[&str]>) -> CoolResult<ShellResult>;

    fn check_available(&mut self, name: &str, args: Option<&[&str]>) -> CoolResult<bool>;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Installer {
    // #[cfg(target_os = "linux")]
    Apt(Apt),
    // #[cfg(target_os = "macos")]
    Brew(Brew),
    Cargo(Cargo),
}

impl AsRef<dyn Installable> for Installer {
    fn as_ref(&self) -> &(dyn Installable + 'static) {
        match self {
            Installer::Apt(apt) => apt,
            Installer::Brew(brew) => brew,
            Installer::Cargo(cargo) => cargo,
        }
    }
}

impl AsMut<dyn Installable> for Installer {
    fn as_mut(&mut self) -> &mut (dyn Installable + 'static) {
        match self {
            Installer::Apt(apt) => apt,
            Installer::Brew(brew) => brew,
            Installer::Cargo(cargo) => cargo,
        }
    }
}

impl Installable for Installer {
    fn install(&mut self, name: &str, args: Option<&[&str]>) -> CoolResult<ShellResult> {
        self.as_mut().install(name, args)
    }

    fn uninstall(&mut self, name: &str, args: Option<&[&str]>) -> CoolResult<ShellResult> {
        self.as_mut().install(name, args)
    }

    fn check_available(&mut self, name: &str, args: Option<&[&str]>) -> CoolResult<bool> {
        match self {
            Installer::Apt(apt) => apt.check_available(name, args),
            Installer::Brew(brew) => brew.check_available(name, args),
            Installer::Cargo(cargo) => cargo.check_available(name, args),
        }
    }
}

impl Serialize for Installer {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            // #[cfg(target_os = "linux")]
            Installer::Apt(_) => serializer.serialize_str("apt"),
            // #[cfg(target_os = "macos")]
            Installer::Brew(_) => serializer.serialize_str("brew"),
            Installer::Cargo(_) => serializer.serialize_str("cargo"),
        }
    }
}

impl<'de> Deserialize<'de> for Installer {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let name = String::deserialize(deserializer)?;
        match name.as_str() {
            // #[cfg(target_os = "linux")]
            "apt" => Ok(Installer::Apt(Apt)),
            // #[cfg(target_os = "macos")]
            "brew" => Ok(Installer::Brew(Brew)),
            "cargo" => Ok(Installer::Cargo(Cargo)),
            _ => Err(serde::de::Error::custom(format!(
                "unknown installer {}",
                name
            ))),
        }
    }
}
