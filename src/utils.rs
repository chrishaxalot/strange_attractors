use super::constants::PRECISION;
use rug::Float;

pub fn scale(
    input_min: &Float,
    input_max: &Float,
    output_min: u32,
    output_max: u32,
    input: &Float,
) -> i32 {
    let input_width = Float::with_val(PRECISION, input_max - input_min);
    let output_width = Float::with_val(PRECISION, output_max - output_min);

    let input_abs_position = Float::with_val(PRECISION, input - input_min);
    let input_rel_position = Float::with_val(PRECISION, input_abs_position / input_width);
    let output_absolute_position = Float::with_val(PRECISION, &input_rel_position * output_width);
    let output_absolute_position = output_absolute_position
        .to_integer()
        .expect("couldn't convert to integer")
        .to_i32()
        .expect("couldn't convert to i32");

    output_min as i32 + output_absolute_position
}
