use std::io::Write;

use clap::{Parser, Subcommand};
use enum_dispatch::enum_dispatch;
use is_terminal::IsTerminal;
use lib_frankfurter::api::ServerClient;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use url::Url;

pub mod convert;
pub mod currencies;
pub mod period;
pub mod utils;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Frankfurter API wrapper.",
    propagate_version(true),
    verbatim_doc_comment
)]
pub struct Cli {
    /// Specify when to colorize output.
    #[arg(short, long, value_name = "WHEN", default_value_t = clap::ColorChoice::default(), ignore_case = true)]
    color: clap::ColorChoice,

    ///  URL of the Frankfurter API which queries should be directed to, e.g. http://localhost:8080
    #[arg(short, long)]
    url: Option<Url>,

    /// Show debug info for errors
    #[arg(short, long, action)]
    debug: bool,

    #[command(subcommand)]
    #[allow(missing_docs)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
#[allow(missing_docs)]
#[enum_dispatch]
pub enum Command {
    Convert(convert::Command),
    Currencies(currencies::Command),
    Period(period::Command),
}

impl Cli {
    /// Get the active [`ColorChoice`].
    ///
    /// Manually converts from [`Self::color`], which is of type [`clap::ColorChoice`], to [`termcolor::ColorChoice`].
    fn choice(&self) -> ColorChoice {
        match self.color {
            clap::ColorChoice::Auto => ColorChoice::Auto,
            clap::ColorChoice::Always => ColorChoice::Always,
            clap::ColorChoice::Never => ColorChoice::Never,
        }
    }

    /// Return a standard output stream that optionally supports color.
    #[must_use]
    fn stdout(&self) -> StandardStream {
        let mut choice = self.choice();
        if choice == ColorChoice::Auto && !std::io::stdout().is_terminal() {
            choice = ColorChoice::Never;
        }

        StandardStream::stdout(choice)
    }

    /// Return a standard error stream that optionally supports color.
    #[must_use]
    fn stderr(&self) -> StandardStream {
        let mut choice = self.choice();
        if choice == ColorChoice::Auto && !std::io::stderr().is_terminal() {
            choice = ColorChoice::Never;
        }

        StandardStream::stderr(choice)
    }

    /// Execute command, possibly returning an error.
    pub async fn execute(self) {
        let server_client: ServerClient = self
            .url
            .as_ref()
            .map(|u| ServerClient::new(u.clone()))
            .unwrap_or_default();

        let stdout = self.stdout();
        let mut stderr = self.stderr();

        if let Err(e) = self.command.execute(server_client, stdout).await {
            stderr
                .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
                .expect("Couldn't set stderr colour");

            if self.debug {
                dbg!(&e);
            }

            writeln!(&mut stderr, "Error: {e}").expect("Failed to write to stderr");
        }
    }
}

/// Provides a common interface for executing the subcommands.
#[enum_dispatch(Command)]
trait ExecuteSubcommand {
    /// Executes the subcommand.
    async fn execute(
        self,
        server_client: ServerClient,
        stdout: StandardStream,
    ) -> anyhow::Result<()>;
}

#[derive(Debug, Parser)]
pub struct SubcommandBaseModifiers {
    /// Print the full JSON response from the server
    #[arg(short = 'j', long, action)]
    pub json: bool,

    /// Print the raw output instead of a table
    #[arg(short = 'r', long, action, conflicts_with = "json")]
    pub raw: bool,
}
