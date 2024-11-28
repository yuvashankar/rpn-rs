use clap::Parser;
use reverse_rpn_rs::{reverse_rpn, Error, Result};

#[derive(Parser, Debug)]
#[command(version, about)]
/// A simple reverse polish notation calculator.
///
/// ```example
/// `100.5 300.3 +`
/// `2 7 *`
/// ```
///
struct Opts {
    /// The mathematical operations formatted in the reverse polish notation
    input: String,
}

fn main() -> Result<()> {
    let input = Opts::parse();

    let rpn_output = reverse_rpn(&input.input)?;

    let result = rpn_output.ok_or(Error::NoResult)?;

    println!("{result}");
    Ok(())
}
