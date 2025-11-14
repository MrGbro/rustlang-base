// rcli ssv -i input.csv -o output.json --header -d ","

use std::path::Path;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "command", version, author,about,long_about=None)]
struct Opts {
    #[command(subcommand)]
    cmd: SubCommand,
}

#[derive(Debug, Parser)]
enum SubCommand {
    #[command(name = "csv", about = "Show CSV or Convert CSV to other formats")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
struct CsvOpts {
    /// 输入文件路径
    #[arg(short, long, help = "Input file",value_parser=verify_input_file)]
    input: String,
    /// 输出文件路径
    #[arg(short, long, help = "Output file", default_value = "output.json")]
    output: String,
    /// CSV分隔符
    #[arg(short, long, help = "Delimiter", default_value_t = ',')]
    delimiter: char,
    /// 是否包含表头
    #[arg(long, help = "Header", default_value_t = true)]
    header: bool,
}

fn verify_input_file(file_path: &str) -> Result<String, &'static str> {
    // Here you can add your file verification logic
    // For example, check if the file exists and is readable
    if Path::new(file_path).exists() {
        Ok(file_path.to_string())
    } else {
        Err("Input file does not exist or is not readable")
    }
}

fn main() {
    let opts = Opts::parse();
    println!("Hello, world! {:?}", opts);
}
