use rs_hex2int::hex_to_i16_le;
use serde::{Deserialize, Serialize};
use std::io::{self, Read};

#[derive(Debug, Deserialize)]
struct Input {
    hex: String,
}

#[derive(Debug, Serialize)]
struct Output<T> {
    value: T,
}

#[derive(Debug, Serialize)]
struct ErrorOutput {
    error: String,
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let input: Input = match serde_json::from_str(&buffer) {
        Ok(input) => input,
        Err(e) => {
            let error_output = ErrorOutput {
                error: format!("Failed to parse input JSON: {}", e),
            };
            serde_json::to_writer_pretty(io::stdout(), &error_output)?;
            return Ok(());
        }
    };

    match hex_to_i16_le(&input.hex) {
        Ok(value) => {
            let output = Output { value };
            serde_json::to_writer_pretty(io::stdout(), &output)?;
        }
        Err(e) => {
            let error_output = ErrorOutput {
                error: format!("Conversion error: {}", e),
            };
            serde_json::to_writer_pretty(io::stdout(), &error_output)?;
        }
    }

    Ok(())
}
