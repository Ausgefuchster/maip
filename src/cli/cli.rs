use std::collections::HashMap;

use super::Command;

pub struct CLI {
    pub commands: Vec<Box<dyn Command>>,
    pub description: String,
    pub version: String,
}

impl CLI {
    pub fn build() -> Self {
        Self {
            commands: Vec::new(),
            description: String::new(),
            version: String::new(),
        }
    }

    pub fn description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }

    pub fn version(mut self, version: &str) -> Self {
        self.version = version.to_string();
        self
    }

    pub fn command<T: Command + 'static>(mut self, command: T) -> Self {
        self.commands.push(Box::new(command));
        self
    }

    pub fn parse(&mut self, args: Vec<String>) -> Result<(), String> {
        let option_args = parse_option_args(&args)?;
        if let Some(command) = args.get(0) {
            let command = self
                .commands
                .iter_mut()
                .find(|c| c.name() == command)
                .ok_or(format!("Command not found: {}", command))?;

            command.set_values(&option_args)?;
            command.run(args[1..].to_vec())?;

            return Ok(());
        }

        println!("{} {}", self.description, self.version);
        Ok(())
    }
}

fn parse_option_args(args: &Vec<String>) -> Result<HashMap<String, String>, String> {
    args.iter()
        .enumerate()
        .filter(|(_, v)| v.starts_with("--") || v.starts_with("-"))
        .map(|(i, v)| {
            get_equals_pair(v)
                .or(get_index_pair(i, args))
                .ok_or(format!("Missing value for key: {}", args[i]))
        })
        .collect()
}

fn get_equals_pair(key: &str) -> Option<(String, String)> {
    let (key, value) = key.split_once("=")?;
    let key = remove_prefix(key);
    Some((key.to_string(), value.to_string()))
}

fn get_index_pair(key_index: usize, args: &Vec<String>) -> Option<(String, String)> {
    let value = args.get(key_index + 1)?;
    let key = remove_prefix(args[key_index].as_str());
    Some((key.to_string(), value.to_string()))
}

fn remove_prefix(key: &str) -> &str {
    match key {
        key if key.starts_with("--") => key.trim_start_matches("--"),
        key if key.starts_with("-") => key.trim_start_matches("-"),
        _ => key,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_option_args() {
        let args = vec![
            String::from("--key=value"),
            String::from("--key"),
            String::from("value"),
            String::from("-k=value"),
            String::from("-k"),
            String::from("value"),
        ];

        let result = super::parse_option_args(&args).unwrap();

        assert_eq!(result.get("key").unwrap(), "value");
        assert_eq!(result.get("k").unwrap(), "value");
    }

    #[test]
    fn test_parse_option_args_missing_value() {
        let args = vec![String::from("--key")];

        let result = super::parse_option_args(&args);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Missing value for key: --key");
    }
}
