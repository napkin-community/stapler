// use owo_colors::{OwoColorize, Stream::Stderr};

use std::{
    fs::{self, File},
    io, thread,
    time::Duration,
};

use colored::Colorize;
use dialoguer::Confirm;
use indicatif::ProgressBar;
use stapler::StaplerLib;

#[derive(clap::Parser)]
#[clap(
    about = "Get the most recent draf of Napkin",
    long_about = r#"Download the Napkin book from Web[1] to $CONFIG/.stapler/napkin-draft.pdf
If there was a pre-existing PDF file, prompt user to overwrite or not

[1]: https://venhance.github.io/napkin/Napkin.pdf"#
)]
pub struct Get {
    #[arg(
        short,
        long,
        help = "When the flag is set, overwrite pre-existing napkin-draft.pdf without prompt"
    )]
    yes: bool,
}

impl Get {
    pub fn run(self) -> eyre::Result<()> {
        let target_path = StaplerLib::draft_pdf_path()?;
        if !self.yes && fs::exists(&target_path)? {
            let overwrite = Confirm::new()
                .with_prompt(format!(
                    "  {} Would you overwrite {}?",
                    "   Question".purple().bold(),
                    target_path.to_string_lossy().underline()
                ))
                .default(true)
                .interact()?;
            if !overwrite {
                eprintln!(
                    "  {} downloading the book into {}",
                    "     Cancel".yellow().bold(),
                    target_path.to_string_lossy().underline()
                );
                return Ok(());
            }
        }
        let pb = ProgressBar::new_spinner();
        pb.set_message(format!(
            "{} ⟨{}⟩ from the Web...",
            "Downloading".green().bold(),
            "An Infinitely Large Napkin".italic()
        ));
        fs::create_dir_all(StaplerLib::config_dir()?)?;
        let handle = thread::spawn({
            let pb = pb.clone();
            move || loop {
                pb.tick();
                thread::sleep(Duration::from_millis(20));
            }
        });
        let mut response = ureq::get("https://venhance.github.io/napkin/Napkin.pdf").call()?;
        fs::remove_file(&target_path)?;
        io::copy(
            &mut response.body_mut().as_reader(),
            &mut File::create(&target_path)?,
        )?;
        drop(handle);
        pb.finish();
        eprintln!(
            "  {} downloading the book into {}",
            "  Completed".green().bold(),
            target_path.to_string_lossy().underline()
        );

        Ok(())
    }
}
