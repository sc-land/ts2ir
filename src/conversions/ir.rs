use crate::traits::IRFromTypeScript;
use ir::{Alveolus, IR, Larvie, Casts, Flora};

/// Implementation of the IRFromTypeScript trait for the IR struct
///
/// This implementation handles the conversion of TypeScript source code into
/// an IR structure by parsing simple class definitions.
impl IRFromTypeScript for IR {
    /// Converts TypeScript source code to an IR structure
    ///
    /// This function parses simple TypeScript class definitions and converts them
    /// to appropriate IR structures:
    /// - Class properties are converted to Casts
    /// - Classes are converted to Larvie and added as Alveolus
    ///
    /// # Parameters
    ///
    /// * `input` - The TypeScript source code to convert
    ///
    /// # Returns
    ///
    /// An IR structure containing all converted elements
    fn from_typescript(input: &str) -> IR {
        let mut alveolus = Vec::new();

        // Simple parser for class definitions
        if let Some(class_info) = parse_simple_class(input) {
            let mut casts = Vec::new();
            
            // Convert properties to casts
            for prop in class_info.properties {
                casts.push(Casts {
                    primor: prop.name,
                    flora: match prop.type_name.as_str() {
                        "number" => Flora::Int,
                        "string" => Flora::Str,
                        "boolean" => Flora::Bool,
                        _ => Flora::Str, // default
                    },
                    seals: vec![], // No seals for now
                });
            }

            // Create larvie from class
            let larvie = Larvie {
                primor: class_info.name,
                casts,
                instincts: vec![], // No methods for now
            };

            alveolus.push(Alveolus::Larvie(larvie));
        }

        IR { alveolus }
    }
}

#[derive(Debug)]
struct SimpleClass {
    name: String,
    properties: Vec<SimpleProperty>,
}

#[derive(Debug)]
struct SimpleProperty {
    name: String,
    type_name: String,
}

/// Simple parser for TypeScript classes
/// This is a minimal implementation focused on the basic class structure
fn parse_simple_class(input: &str) -> Option<SimpleClass> {
    let lines: Vec<&str> = input.lines().collect();
    let mut class_name = String::new();
    let mut properties = Vec::new();
    let mut in_class = false;

    for line in lines {
        let trimmed = line.trim();

        if trimmed.starts_with("class ") {
            // Extract class name
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 2 {
                class_name = parts[1].trim_end_matches('{').to_string();
                in_class = true;
            }
        } else if in_class && trimmed.contains(':') && !trimmed.starts_with("//") {
            // Parse property
            if let Some(property) = parse_simple_property(trimmed) {
                properties.push(property);
            }
        } else if in_class && trimmed == "}" {
            break;
        }
    }

    if class_name.is_empty() {
        None
    } else {
        Some(SimpleClass {
            name: class_name,
            properties,
        })
    }
}

/// Parse a simple property line like "energy: number;"
fn parse_simple_property(line: &str) -> Option<SimpleProperty> {
    let trimmed = line.trim_end_matches(';').trim();

    if let Some(colon_pos) = trimmed.find(':') {
        let name = trimmed[..colon_pos].trim().to_string();
        let type_name = trimmed[colon_pos + 1..].trim().to_string();

        return Some(SimpleProperty { name, type_name });
    }

    None
}
