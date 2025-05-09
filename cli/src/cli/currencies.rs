use std::io::Write;

use clap::Parser;
use comfy_table::{
    modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL_CONDENSED, Cell, CellAlignment, Color,
    ContentArrangement, Table,
};
use lib_frankfurter::api::{self, ServerClient};
use termcolor::StandardStream;

use super::{ExecuteSubcommand, SubcommandBaseModifiers};
use crate::cli::utils::if_supports_colour;

/// Fetch the latest supported currency codes and their full names
#[derive(Debug, Parser)]
pub struct Command {
    #[command(flatten)]
    modifiers: SubcommandBaseModifiers,
}

impl ExecuteSubcommand for Command {
    /// Executes the `currencies` subcommand.
    async fn execute(
        self,
        server_client: ServerClient,
        mut stdout: StandardStream,
    ) -> anyhow::Result<()> {
        let response = server_client
            .currencies(api::currencies::Request {})
            .await?;

        if self.modifiers.json {
            writeln!(&mut stdout, "{}", serde_json::to_string_pretty(&response)?)?;
        } else if self.modifiers.raw {
            stdout.write_all(
                &response
                    .0
                    .into_iter()
                    .flat_map(|(k, v)| format!("{k}\t{v}\r\n").into_bytes())
                    .collect::<Vec<u8>>(),
            )?;
        } else {
            let mut table = Table::new();
            table
                .load_preset(UTF8_FULL_CONDENSED)
                .apply_modifier(UTF8_ROUND_CORNERS)
                .set_header(vec!["Code", "Full name"])
                .set_content_arrangement(ContentArrangement::Dynamic)
                .add_rows(response.0.into_iter().map(|(k, v)| {
                    vec![
                        Cell::new(k)
                            .set_alignment(CellAlignment::Right)
                            .fg(if_supports_colour(&stdout, Color::Green)),
                        Cell::new(v).fg(if_supports_colour(&stdout, Color::Cyan)),
                    ]
                }));

            writeln!(&mut stdout, "{table}")?;
        }

        Ok(())
    }
}
