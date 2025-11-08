use std::fs;

fn main() -> Result<(), std::io::Error> {
    let path = "assets/data.csv";
    let contents = read_file(path)?;
    println!("{}", contents);
    Ok(())
}

#[allow(unused)]
fn read_file(file_path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(file_path)
}
