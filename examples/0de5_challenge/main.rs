use rpn_rs::Result;
fn main() -> Result<()> {
    let file_contents =
        std::fs::read_to_string("examples/0de5_challenge/rpn_expression-5869168714501555882.txt")?;
    let output = rpn_rs::rpn_calculator::<f64>(&file_contents)?;

    let result = output.unwrap();
    println!("{result}");
    Ok(())
}
