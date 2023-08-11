mod cli;
use cli::args::CliConfig;

mod io;
mod converter;

fn main() {
    match CliConfig::parse_arguments() {
        Ok(config) => {
            println!("Config successfully parsed:");
            println!("---------------------------");
            println!("  Input path: {}", config.input_file);
            println!("  Output path: {}", config.output_file);
            println!("  Conversion type: {:?}", config.conversion_type);
            println!("---------------------------");
        },
        Err(e) => {
            eprintln!("Error parsing arguments: {}", e);
        }
    }
}
