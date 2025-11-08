use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let path = "assets/data.csv";
    let contents = read_file(path)?;
    println!("{}", contents);
    Ok(())
}

#[allow(unused)]
fn read_file(file_path: &str) -> Result<String> {
    let result = fs::read_to_string(file_path)?; //内部使用了error convert
    Ok(result)
}
