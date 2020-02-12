use std::path::PathBuf;

pub use filesystem_shell::FilesystemShell;

use crate::error::ShellError;

pub mod filesystem_shell;

pub trait Shell: std::fmt::Debug {
    fn name(&self) -> String;
    fn homedir(&self) -> Option<PathBuf>;

    //    fn ls(
    //        &self,
    //        args: LsArgs,
    //        context: &RunnablePerItemContext,
    //    ) -> Result<OutputStream, ShellError>;
    //    fn cd(&self, args: EvaluatedWholeStreamCommandArgs) -> Result<OutputStream, ShellError>;
    //    fn cp(&self, args: CopyArgs, name: Tag, path: &str) -> Result<OutputStream, ShellError>;
    //    fn mkdir(&self, args: MkdirArgs, name: Tag, path: &str) -> Result<OutputStream, ShellError>;
    //    fn mv(&self, args: MoveArgs, name: Tag, path: &str) -> Result<OutputStream, ShellError>;
    //    fn rm(&self, args: RemoveArgs, name: Tag, path: &str) -> Result<OutputStream, ShellError>;
    //    fn path(&self) -> String;
    //    fn pwd(&self, args: EvaluatedWholeStreamCommandArgs) -> Result<OutputStream, ShellError>;
    //    fn set_path(&mut self, path: String);
}
