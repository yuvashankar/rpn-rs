use rpn_rs::Result;
fn main() -> Result<()> {
    let file_contents = std::fs::read_to_string("rpn_expression-5869168714501555882.txt")?;
    let output = rpn_rs::rpn_calculator(&file_contents)?;

    let result = output.unwrap();
    println!("{result}");
    Ok(())
}
