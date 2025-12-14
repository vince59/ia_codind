use std::io::{self, Write};
use std::process;

/// Minimum length required for a code description
const MIN_DESCRIPTION_LENGTH: usize = 5;

fn main() {
    println!("=== IA Coding - Code Generator ===");
    println!("Generate Rust code from natural language descriptions");
    println!("Type 'quit' or 'exit' to leave\n");

    loop {
        print!("Enter your description (e.g., 'Create a for loop in Rust iterating from 0 to 10'): ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();
                
                // Check for exit commands
                if input.is_empty() {
                    continue;
                }
                
                if input.eq_ignore_ascii_case("quit") || input.eq_ignore_ascii_case("exit") {
                    println!("Goodbye!");
                    break;
                }

                // Process the input
                match generate_code(input) {
                    Ok(code) => {
                        println!("\n--- Generated Code ---");
                        println!("{}", code);
                        println!("--- End of Generated Code ---\n");
                    }
                    Err(e) => {
                        eprintln!("Error: {}\n", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to read input: {}", e);
                process::exit(1);
            }
        }
    }
}

/// Generate code based on natural language description
/// 
/// # Arguments
/// * `description` - Natural language description of the code to generate
/// 
/// # Returns
/// * `Result<String, String>` - Generated code or error message
fn generate_code(description: &str) -> Result<String, String> {
    // Validate input
    if description.is_empty() {
        return Err("Description cannot be empty".to_string());
    }

    if description.len() < MIN_DESCRIPTION_LENGTH {
        return Err(format!(
            "Description is too short. Please provide a more detailed description (at least {} characters).",
            MIN_DESCRIPTION_LENGTH
        ));
    }

    // TODO: Integration with llama_cpp will be implemented here
    // For now, we return an error explaining the model configuration requirement
    // Once a model is configured, replace this with actual llama_cpp code
    // See README.md for detailed integration instructions
    
    Err(format!(
        "Model not configured. To enable code generation:\n\
         1. Download a GGUF model file (e.g., from Hugging Face)\n\
         2. Place it in a 'models' directory in the project root\n\
         3. Update the generate_code() function to load and use the model\n\
         4. See README.md for detailed instructions\n\
         \n\
         Your request: '{}'",
        description
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_code_empty_input() {
        let result = generate_code("");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Description cannot be empty");
    }

    #[test]
    fn test_generate_code_short_input() {
        let result = generate_code("test");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("too short"));
    }

    #[test]
    fn test_generate_code_valid_input() {
        let result = generate_code("Create a for loop in Rust");
        // Currently returns error as model is not loaded
        assert!(result.is_err());
    }
}
