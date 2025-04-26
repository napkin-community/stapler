use std::path::PathBuf;

use directories::ProjectDirs;
use eyre::OptionExt;

pub enum StaplerLib {}

impl StaplerLib {
    pub fn config_dir() -> eyre::Result<PathBuf> {
        ProjectDirs::from("io.github.napkin-community", "Napkin Community", "Stapler")
            .ok_or_eyre("The Operating System do not provide directory for Stapler CLI app")
            .map(|dirs| dirs.config_dir().to_owned())
    }

    pub fn draft_pdf_path() -> eyre::Result<PathBuf> {
        Self::config_dir().map(|path| path.join("napkin-draft.pdf"))
    }
}
