use std::io::Write;

use clap::Parser;
use comfy_table::{
    modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL_CONDENSED, Cell, CellAlignment, Color,
    ContentArrangement, Table,
};
use lib_frankfurter::{
    api::{self, ServerClient},
    Currency, CurrencyValue, ValidDate,
};
use termcolor::StandardStream;

use super::{utils::if_supports_colour, ExecuteSubcommand, SubcommandBaseModifiers};

/// Convert between 2 currencies using the convert exchange rates
#[derive(Debug, Parser)]
pub struct Command {
    /// Base currency to convert FROM
    #[arg(ignore_case = true, index = 1, default_value_t)]
    base: Currency,
    /// Target currencies to convert TO, e.g. USD,AUD
    #[arg(ignore_case = true, index = 2, value_delimiter = ',')]
    targets: Vec<Currency>,

    /// A number representing the amount of the base currency to show exchange rates for [default: 1]
    // TODO: if `clap` ever supports it, make this a positional argument after the targets.
    // See <https://github.com/clap-rs/clap/issues/4626>
    // Attempted solutions:
    // - `num_args` for `targets` -> all args are consumed anyway
    // - `last(true)` for `amount` -> requires that the amount comes after `--`
    #[arg(short = 'a', long)]
    amount: Option<CurrencyValue>,

    /// Date for exchange rates [form: yyyy-mm-dd, min: 1999-01-04, default: today]
    #[arg(short = 'd', long)]
    date: Option<ValidDate>,

    #[command(flatten)]
    modifiers: SubcommandBaseModifiers,
}

impl From<Command> for api::convert::Request {
    fn from(value: Command) -> Self {
        api::convert::Request {
            amount: value.amount,
            base: Some(value.base),
            targets: if value.targets.is_empty() {
                None
            } else {
                Some(value.targets)
            },
            date: value.date,
        }
    }
}

impl ExecuteSubcommand for Command {
    /// Executes the `convert` subcommand.
    async fn execute(
        self,
        server_client: ServerClient,
        mut stdout: StandardStream,
    ) -> anyhow::Result<()> {
        let SubcommandBaseModifiers { json, raw } = self.modifiers;
        let response = server_client.convert(self.into()).await?;

        if json {
            writeln!(&mut stdout, "{}", serde_json::to_string_pretty(&response)?)?;
        } else if raw {
            stdout.write_all(
                &response
                    .rates
                    .into_iter()
                    .flat_map(|(k, v)| format!("{k}\t{v}\r\n").into_bytes())
                    .collect::<Vec<u8>>(),
            )?;
        } else {
            let mut table = Table::new();
            table
                .load_preset(UTF8_FULL_CONDENSED)
                .apply_modifier(UTF8_ROUND_CORNERS)
                .set_header(vec!["Currency", "Value"])
                .set_content_arrangement(ContentArrangement::Dynamic)
                .add_rows(response.rates.into_iter().map(|(k, v)| {
                    vec![
                        Cell::new(k)
                            .set_alignment(CellAlignment::Right)
                            .fg(if_supports_colour(&stdout, Color::Green)),
                        Cell::new(v).fg(if_supports_colour(&stdout, Color::Cyan)),
                    ]
                }));

            writeln!(&mut stdout, "{}", table)?;
        }

        Ok(())
    }
}
