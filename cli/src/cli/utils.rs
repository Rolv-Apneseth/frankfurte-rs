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
