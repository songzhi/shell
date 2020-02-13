use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct FilesystemShell {}

impl FilesystemShell {
    pub fn new() -> Self {
        Self {}
    }
}

impl super::Shell for FilesystemShell {
    fn name(&self) -> String {
        unimplemented!()
    }

    fn homedir(&self) -> Option<PathBuf> {
        unimplemented!()
    }
}
