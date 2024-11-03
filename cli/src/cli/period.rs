use std::io::Write;

use chrono::NaiveDate;
use clap::Parser;
use comfy_table::{
    modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL_CONDENSED, Cell, CellAlignment, Color,
    ContentArrangement, Table,
};
use lib_frankfurter::{
    api::{self, ServerClient},
    data::{Currency, CurrencyValue},
};
use termcolor::StandardStream;

use super::{utils::if_supports_colour, ExecuteSubcommand, SubcommandBaseModifiers};

#[derive(Debug, Parser)]
pub struct Command {
    /// Base currency to convert FROM, e.g. EUR
    #[arg(ignore_case = true, index = 1)]
    base: Currency,

    /// Target currencies to convert TO, e.g. USD,AUD
    #[arg(ignore_case = true, short = 't', long, value_delimiter = ',')]
    targets: Vec<Currency>,

    /// A number representing the amount of the base currency to show exchange rates for (default = 1)
    #[arg(short = 'a', long)]
    amount: Option<CurrencyValue>,

    /// The start date for the time period to fetch exchange rates for.
    ///
    /// Dates should be written in the form `yyyy-mm-dd`
    #[arg(index = 2)]
    start: NaiveDate,
    /// Date for exchange rates, in the form `yyyy-mm-dd`.
    ///
    /// If not specified, this will default to the latest available date.
    #[arg(index = 3)]
    end: Option<NaiveDate>,

    #[command(flatten)]
    modifiers: SubcommandBaseModifiers,
}

impl From<&Command> for api::period::Request {
    fn from(value: &Command) -> Self {
        api::period::Request {
            amount: value.amount,
            base: Some(value.base.clone()),
            targets: Some(value.targets.clone()),
            start_date: value.start,
            end_date: value.end,
        }
    }
}

impl ExecuteSubcommand for Command {
    /// Executes the `period` subcommand.
    async fn execute(
        self,
        server_client: ServerClient,
        mut stdout: StandardStream,
    ) -> anyhow::Result<()> {
        let SubcommandBaseModifiers { json, raw } = self.modifiers;
        let response = server_client.period((&self).into()).await?;

        if json {
            writeln!(&mut stdout, "{}", serde_json::to_string_pretty(&response)?)?;
        } else if raw {
            for (date, map) in response.rates.into_iter() {
                writeln!(&mut stdout, "{date}")?;

                stdout.write_all(
                    &map.into_iter()
                        .flat_map(|(k, v)| format!("\t{k}\t{v}\r\n").into_bytes())
                        .collect::<Vec<u8>>(),
                )?;
            }
        } else {
            let mut table = Table::new();

            table
                .load_preset(UTF8_FULL_CONDENSED)
                .apply_modifier(UTF8_ROUND_CORNERS)
                .set_header(vec!["Date", "Currency", "Value"])
                .set_content_arrangement(ContentArrangement::Dynamic);

            for (date, map) in response.rates.into_iter() {
                let mut iter = map.into_iter();
                let first = iter
                    .next()
                    .unwrap_or_else(|| panic!("No rates returned for date {date}"));

                table.add_row(vec![
                    Cell::new(date).fg(if_supports_colour(&stdout, Color::Blue)),
                    Cell::new(first.0)
                        .set_alignment(CellAlignment::Center)
                        .fg(if_supports_colour(&stdout, Color::Green)),
                    Cell::new(first.1).fg(if_supports_colour(&stdout, Color::Cyan)),
                ]);

                for (k, v) in iter {
                    table.add_row(vec![
                        Cell::new(""),
                        Cell::new(k)
                            .set_alignment(CellAlignment::Center)
                            .fg(if_supports_colour(&stdout, Color::Green)),
                        Cell::new(v).fg(if_supports_colour(&stdout, Color::Cyan)),
                    ]);
                }
            }

            writeln!(&mut stdout, "{}", table)?;
        }

        Ok(())
    }
}
