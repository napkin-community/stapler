use std::fs;

use colored::Colorize;
use jiff::{fmt::strtime::format, Timestamp, Zoned};
use stapler::StaplerLib;

#[derive(clap::Parser)]
#[clap(
    about = "List all downloaded/built Napkin books",
    long_about = r#"List all downloaded/built Napkin books inside $CONFIG/.stapler/"#
)]
pub struct List {
    #[arg(
        long,
        help = "Output to stdout without formatting, if used with --json this flag will be ignored"
    )]
    raw: bool,
    #[arg(long, help = "Output to stdout in JSON format")]
    json: bool,
}

impl List {
    pub fn run(self) -> eyre::Result<()> {
        if self.raw && self.json {
            eprintln!("warn: --raw is supplied with --json, the --raw will be ignored");
        }
        if self.json {
            print!("[");
        }
        let mut preceding_comma = false;
        let draft_pdf = StaplerLib::draft_pdf_path()?;
        if let Ok(metadata) = fs::metadata(&draft_pdf) {
            if self.json {
                print!("{:?}", draft_pdf.display());
                preceding_comma = true;
            } else if self.raw {
                println!("{}", draft_pdf.display());
            } else {
                eprintln!(
                    "  {} most recent draft of {}",
                    "Found".green().bold(),
                    "Napkin".italic()
                );
                eprintln!(
                    "        - {} {}",
                    draft_pdf.to_string_lossy().underline(),
                    format!(
                        "downloaded at {}",
                        Zoned::try_from(metadata.created()?)?.strftime("%Y-%m-%d")
                    )
                    .bright_black()
                );
            }
        }
        if self.json {
            println!("]");
        }
        Ok(())
    }
}
