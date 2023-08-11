mod cli;
use cli::args::CliConfig;
use std::path::Path;
use io::reader::FileReader;
use io::writer::FileWriter;

use converter::json_to_xml::JsonToXmlConverter;

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

            // Read the file
            let reader = FileReader::new();
            let contents = match reader.read_file_to_string(Path::new(&config.input_file)) {
                Ok(contents) => contents,
                Err(e) => {
                    eprintln!("Error reading input file: {}", e);
                    return;
                }
            };

            let result = match config.conversion_type {

                cli::args::ConversionType::JsonToXml => {
                    let converter = JsonToXmlConverter::new();
                    converter.convert_json_to_xml(&contents)
                },
                _ => {
                    println!("Unsupported conversion type cause your conversion type is: {:?}", config.conversion_type);
                    eprintln!("Unsupported conversion type.");
                    return;
                }
            };

            // Handle possible conversion errors
            let converted_data = match result {
                Ok(data) => data,
                Err(e) => {
                    eprintln!("Conversion error: {:?}", e);


                    return;
                }
            };

            // Write the result to the output file
            let writer = FileWriter::new();
            if let Err(e) = writer.write_string_to_file(Path::new(&config.output_file), &converted_data) {
                eprintln!("Error writing to output file: {}", e);
            }

        },
        Err(e) => {
            eprintln!("Error parsing arguments: {}", e);
        }
    }
}
