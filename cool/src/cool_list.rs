use crate::shell::{MacOSSudo, Shell};
use crate::tasks::{Task, Tasks, WhichTask};
use crate::Cool;
use include_dir::{include_dir, Dir};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

static COOL_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/../assets/cools");

pub static COOL_LIST: Lazy<Arc<RwLock<HashMap<String, Cool>>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    COOL_DIR.find("**/*.toml").unwrap().for_each(|entry| {
        if cfg!(macos) {
            let parent = entry.path().parent().unwrap().to_string_lossy().to_string();
            if &parent == "brew"
                || &parent == "universal"
                || &parent == "cargo"
                || &parent == "flutter"
                || &parent == "shell"
            {
                let cool =
                    toml::from_str::<Cool>(entry.as_file().unwrap().contents_utf8().unwrap())
                        .unwrap();
                map.insert(cool.name.clone(), cool);
            }
        }
    });
    Arc::new(RwLock::new(map))
});

#[cfg(test)]
mod test {
    use crate::result::CoolResult;
    use crate::shell::{MacOSSudo, Shell};
    use crate::tasks::{Task, Tasks, WhichTask};
    use crate::{init_backtrace, Cool, COOL_LIST};

    #[test]
    fn test_cool_list() -> CoolResult<()> {
        init_backtrace();
        COOL_LIST.read().unwrap();
        let brew_cool = Cool {
            name: "homebrew".to_string(),
            description: "适用于macOS的包管理器。它使您能够从命令行安装和更新软件包，从而使您的Mac保持最新状态，而无需使用App Store。".to_string(),
            dependencies: vec![],
            install_tasks: Tasks(vec![
                Task::download("https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh", "{{TEMP_DIR}}/homebrew/install.sh"),
                Task::command("{{TEMP_DIR}}/homebrew/install.sh", None::<Vec<&str>>, Some(vec![("NONINTERACTIVE", "1")]), Shell::MacOSSudo(MacOSSudo)),
            ]),
            uninstall_tasks: Tasks(vec![
                Task::download("https://raw.githubusercontent.com/Homebrew/install/HEAD/uninstall.sh", "{{TEMP_DIR}}/homebrew/uninstall.sh"),
                Task::command("{{TEMP_DIR}}/homebrew/uninstall.sh", None::<Vec<&str>>, Some(vec![("NONINTERACTIVE", "1")]), Shell::MacOSSudo(MacOSSudo)),
            ]),
            check_tasks: Tasks(vec![
                Task::WhichTask(WhichTask::new("brew".to_string()))
            ]),
        };
        let string = toml::to_string(&brew_cool)?;
        println!("{}", string);
        let cool = toml::from_str::<Cool>(&string)?;
        println!("{:#?}", cool);
        Ok(())
    }
}
