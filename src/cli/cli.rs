use std::collections::HashMap;

use super::Command;

pub struct CLI {
    commands: Vec<Box<dyn Command>>,
    description: String,
    version: String,
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

fn parse_option_args(args: &[String]) -> Result<HashMap<String, Vec<String>>, String> {
    let mut option_args: HashMap<String, Vec<String>> = HashMap::new();
    for (i, arg) in args.iter().enumerate() {
        if !arg.starts_with("--") {
            continue;
        }

        let (key, value) = get_equals_pair(arg)
            .or(get_index_pair(i, args))
            .ok_or(format!("Missing value for key: {}", args[i]))?;

        push_to_value_if_present(&mut option_args, key, value);
    }
    Ok(option_args)
}

fn push_to_value_if_present(option_args: &mut HashMap<String, Vec<String>>, key: String, value: String) {
    if let Some(values) = option_args.get_mut(&key) {
        values.push(value);
    } else {
        option_args.insert(key, vec![value]);
    }
}

fn get_equals_pair(key: &str) -> Option<(String, String)> {
    let (key, value) = key.split_once('=')?;
    let key = remove_prefix(key);
    Some((key.to_string(), value.to_string()))
}

fn get_index_pair(key_index: usize, args: &[String]) -> Option<(String, String)> {
    let value = args.get(key_index + 1)?;
    let key = remove_prefix(args[key_index].as_str());
    Some((key.to_string(), value.to_string()))
}

fn remove_prefix(key: &str) -> &str {
    match key {
        key if key.starts_with("--") => key.trim_start_matches("--"),
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
            String::from("value")
        ];

        let result = super::parse_option_args(&args).unwrap();

        assert_eq!(result.get("key").unwrap(), &vec!["value", "value"]);
    }

    #[test]
    fn test_parse_option_args_missing_value() {
        let args = vec![String::from("--key")];

        let result = super::parse_option_args(&args);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Missing value for key: --key");
    }
}
