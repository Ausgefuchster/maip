use crate::cli::{Arguments, Command};
use crate::policy::{merge_policy_documents, policy_from_file, policy_to_file, PolicyDocument};
use std::collections::HashMap;

#[derive(Default)]
pub struct Merge {
    files: Vec<String>,
    out: String,
    all: String,
}

impl Arguments for Merge {
    fn set_option_args(&mut self, args: &HashMap<String, Vec<String>>) -> Result<(), String> {
        self.out = args
            .get("out")
            .unwrap_or(&vec!["merged.json".to_string()])
            .get(0)
            .ok_or("Missing value for --out option".to_string())?
            .to_string();
        if let Some(files) = args.get("file") {
            self.files = files.clone();
        }
        if let Some(all) = args.get("all") {
            self.all = all
                .get(0)
                .ok_or("Missing value for --all option".to_string())?
                .to_string();
        }
        Ok(())
    }

    fn set_positional_args(&mut self, args: &[String]) -> Result<(), String> {
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

        policy_to_file(self.out.as_str(), &result)?;
        Ok(())
    }

    fn name(&self) -> &str {
        "merge"
    }

    fn required_args(&self) -> Vec<String> {
        vec![]
    }

    fn optional_args(&self) -> Vec<String> {
        vec!["file".to_string(), "out".to_string()]
    }
}
