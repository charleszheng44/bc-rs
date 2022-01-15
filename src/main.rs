mod evaluator;
mod parser;
mod tokenizer;

use anyhow::Result;

fn main() -> Result<()> {
    println!("please type the input expression:");
    let mut inp_expr = String::new();
    std::io::stdin().read_line(&mut inp_expr)?;
    Ok(())
}
