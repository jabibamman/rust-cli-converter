use std::fmt;
use std::path::Path;

pub struct CliConfig {
    pub input_file: String,
    pub output_file: String,
    pub conversion_type: ConversionType,
    pub help: bool,
}

#[derive(PartialEq, Debug)]
pub enum ConversionType {
    JsonToXml,
    XmlToJson,
}


impl fmt::Display for ConversionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConversionType::JsonToXml => write!(f, "JsonToXml"),
            ConversionType::XmlToJson => write!(f, "XmlToJson"),
        }
    }
}

impl CliConfig {
    pub fn new() -> Self {
        Self {
            input_file: String::new(),
            output_file: String::new(),
            conversion_type: ConversionType::JsonToXml,
            help: false,
        }
    }

    pub fn parse_arguments() -> Result<CliConfig, String> {
        let ignored_flags = ["--package", "--bin"];

        let args = std::env::args().collect::<Vec<String>>();

        let mut config = CliConfig::new();
        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "-i" => {
                    i += 1;
                    config.input_file = args[i].clone();
                }
                "-o" => {
                    i += 1;
                    config.output_file = args[i].clone();
                    let path = Path::new(&config.output_file);
                    if let Some(extension) = path.extension() {
                        let format = extension.to_str().unwrap();
                        config.conversion_type = match format {
                            "json" => ConversionType::JsonToXml,
                            "xml" => ConversionType::XmlToJson,
                            _ => return Err("Invalid format".to_string()),
                        };
                    }
                }

                "-h" => {
                    config.help = true;
                }

                flag if ignored_flags.contains(&flag) => { /* ignore */ },

                _ => {
                  if args[i].starts_with("-") {
                    return Err(format!("Invalid argument: {}", args[i]));
                  }
                }
            }
            i += 1;
        }

        if config.help {
            return Err("Help".to_string());
        }

        if config.input_file.is_empty() || config.output_file.is_empty() {
            return Err("No input file or output file".to_string());
        }

        if config.conversion_type != ConversionType::JsonToXml && config.conversion_type != ConversionType::XmlToJson {
            return Err("No format".to_string());
        }

        Ok(config)
    }

}