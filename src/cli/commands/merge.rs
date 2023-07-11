use crate::cli::{Arguments, Command};
use crate::policy::{merge_policy_documents, policy_from_file, policy_to_file, PolicyDocument};
use std::collections::HashMap;

#[derive(Default)]
pub struct Merge {
    files: Vec<String>,
    output: String,
}

impl Arguments for Merge {
    fn set_values(&mut self, args: &HashMap<String, Vec<String>>) -> Result<(), String> {
        self.files = args.get("file").ok_or("Missing argument: file")?.clone();
        self.output = args.get("output").ok_or("Missing argument: output")?[0].to_string();
        Ok(())
    }
}

impl Command for Merge {
    fn run(&self, args: Vec<String>) -> Result<(), String> {
        let documents = self
            .files
            .iter()
            .map(|f| policy_from_file(f))
            .collect::<Result<Vec<PolicyDocument>, String>>()?;
        let result = merge_policy_documents(&documents);

        policy_to_file(self.output.as_str(), &result)?;
        Ok(())
    }

    fn name(&self) -> &str {
        "merge"
    }

    fn required_args(&self) -> Vec<String> {
        vec![]
    }

    fn optional_args(&self) -> Vec<String> {
        vec![
            "file".to_string(),
            "output".to_string(),
        ]
    }
}
