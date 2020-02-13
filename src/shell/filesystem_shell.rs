use std::env;
use std::env::current_dir;
use std::path::PathBuf;
use std::sync::atomic::Ordering;

use crate::commands::cd::CdArgs;
use crate::commands::cp::CopyArgs;
use crate::commands::ls::LsArgs;
use crate::commands::mkdir::MkdirArgs;
use crate::commands::RunnableContext;
use crate::error::ShellError;
use crate::evaluate::Value;

#[derive(Debug, Clone, Default)]
pub struct FilesystemShell {}

impl FilesystemShell {
    pub fn new() -> Self {
        Self::default()
    }
}

impl super::Shell for FilesystemShell {
    fn name(&self) -> String {
        "filesystem".to_string()
    }

    fn homedir(&self) -> Option<PathBuf> {
        dirs::home_dir()
    }

    fn ls(
        &self,
        LsArgs { path }: LsArgs,
        context: &RunnableContext,
    ) -> Result<Option<Vec<Value>>, ShellError> {
        let ctrl_c = context.ctrl_c.clone();
        let path = match path {
            None => {
                if is_dir_empty(&self.path().into()) {
                    return Ok(None);
                } else {
                    PathBuf::from("./*")
                }
            }
            Some(mut p) => {
                if p.is_dir() {
                    if is_dir_empty(&p) {
                        return Ok(None);
                    }
                    p.push("*");
                }
                p
            }
        };
        let mut paths = match glob::glob(&path.to_string_lossy()) {
            Ok(g) => Ok(g),
            Err(e) => Err(ShellError::runtime_error("Invalid File or Pattern")),
        }?
        .peekable();
        if paths.peek().is_none() {
            return Err(ShellError::runtime_error("Invalid File or Pattern"));
        }
        let mut results = vec![];
        for path in paths {
            if ctrl_c.load(Ordering::SeqCst) {
                break;
            }
            if let Ok(path) = path {
                if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
                    let item = format!("{}: {}", get_path_type(&path), name);
                    results.push(Value::String(item));
                }
            }
        }
        if results.is_empty() {
            Ok(None)
        } else {
            Ok(Some(results))
        }
    }

    fn cd(&self, args: CdArgs) -> Result<Option<Vec<Value>>, ShellError> {
        let target = match args.dst {
            None => match dirs::home_dir() {
                Some(o) => o,
                _ => {
                    return Err(ShellError::runtime_error(
                        "Can not change to home directory",
                    ));
                }
            },
            Some(target) => target,
        };
        if target.exists() && !target.is_dir() {
            return Err(ShellError::runtime_error(format!(
                "{} is not a directory",
                target.to_string_lossy().to_string()
            )));
        }
        let path = PathBuf::from(self.path());
        match dunce::canonicalize(path.join(&target)) {
            Ok(p) => {
                env::set_current_dir(p).expect("cannot to set current directory");
                Ok(None)
            }
            Err(_) => Err(ShellError::runtime_error("directory not found")),
        }
    }

    fn cp(&self, args: CopyArgs) -> Result<Option<Vec<Value>>, ShellError> {
        unimplemented!()
    }

    fn mkdir(
        &self,
        MkdirArgs { rest: directories }: MkdirArgs,
    ) -> Result<Option<Vec<Value>>, ShellError> {
        let full_path = PathBuf::from(self.path());
        for dir in directories {
            let create_at = {
                let mut loc = full_path.clone();
                loc.push(&dir);
                loc
            };

            let dir_res = std::fs::create_dir_all(create_at);
            if let Err(reason) = dir_res {
                return Err(ShellError::runtime_error(reason.to_string()));
            }
        }
        Ok(None)
    }

    fn path(&self) -> String {
        current_dir()
            .expect("can't get current directory")
            .to_string_lossy()
            .to_string()
    }
    fn pwd(&self) -> Result<Option<Vec<Value>>, ShellError> {
        Ok(None)
    }
}

fn is_dir_empty(d: &PathBuf) -> bool {
    match d.read_dir() {
        Err(_e) => true,
        Ok(mut s) => s.next().is_none(),
    }
}

fn get_path_type(d: &PathBuf) -> &str {
    if d.is_dir() {
        "d"
    } else if d.is_file() {
        "f"
    } else {
        " "
    }
}
