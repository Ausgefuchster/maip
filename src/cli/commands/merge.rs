use std::collections::HashMap;
use std::fs::{read_dir, ReadDir};

use crate::cli::{Arguments, Command};
use crate::policy::{merge_policy_documents, policy_from_file, policy_to_file, PolicyDocument};

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

    fn set_positional_args(&mut self, _: &[String]) -> Result<(), String> {
        Ok(())
    }
}

impl Command for Merge {
    fn run(&self) -> Result<(), String> {
        let mut documents = files_to_documents(&self.files)?;

        if !self.all.is_empty() {
            let directory = read_dir(self.all.as_str())
                .map_err(|e| format!("Failed to read directory: {}", e))?;
            let files = get_json_files(directory);
            documents.extend(files_to_documents(&files)?);
        }

        merge_documents(&documents)?;
        Ok(())
    }

    fn name(&self) -> &str {
        "merge"
    }

    fn required_args(&self) -> Vec<String> {
        vec![]
    }

    fn optional_args(&self) -> Vec<String> {
        vec!["file".to_string(), "out".to_string(), "all".to_string()]
    }
}

fn files_to_documents(files: &[String]) -> Result<Vec<PolicyDocument>, String> {
    files
        .iter()
        .map(|file| policy_from_file(file.as_str()))
        .collect()
}

fn get_json_files(directory: ReadDir) -> Vec<String> {
    directory
        .filter_map(|f| {
            let file = f.ok()?;
            let path = file.path();
            let extension = path.extension()?;
            if !path.is_file() || extension != "json" {
                return None;
            }
            Some(path.to_str()?.to_string())
        })
        .collect::<Vec<String>>()
}

fn merge_documents(documents: &[PolicyDocument]) -> Result<(), String> {
    if documents.is_empty() {
        return Err("Policies to merge".to_string());
    }
    let result = merge_policy_documents(documents);

    policy_to_file("merged.json", &result)?;
    Ok(())
}
