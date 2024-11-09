use termcolor::{StandardStream, WriteColor};

pub fn if_supports_colour(
    stream: &StandardStream,
    colour: comfy_table::Color,
) -> comfy_table::Color {
    if stream.supports_color() {
        colour
    } else {
        comfy_table::Color::Reset
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use comfy_table::Color;
    use termcolor::{ColorChoice, StandardStream};

    #[test]
    fn test_if_supports_colour() {
        let get_stream = StandardStream::stdout;

        assert_eq!(
            comfy_table::Color::Reset,
            if_supports_colour(&get_stream(ColorChoice::Never), Color::Red)
        );
        assert_eq!(
            comfy_table::Color::Reset,
            if_supports_colour(&get_stream(ColorChoice::Never), Color::Cyan)
        );

        assert_eq!(
            comfy_table::Color::Green,
            if_supports_colour(&get_stream(ColorChoice::Always), Color::Green)
        );
        assert_eq!(
            comfy_table::Color::Black,
            if_supports_colour(&get_stream(ColorChoice::AlwaysAnsi), Color::Black)
        );

        assert_eq!(
            comfy_table::Color::Reset,
            if_supports_colour(&get_stream(ColorChoice::Always), Color::Reset)
        );

        // NOTE: No test for `ColorChoice::Auto` - not sure how that would react in a CI environment
    }
}
