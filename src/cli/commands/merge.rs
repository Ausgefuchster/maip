use std::collections::HashMap;
use std::fs::{read_dir, ReadDir};

use crate::cli::{Arguments, Command};
use crate::policy::{
    merge_policy_documents, policy_from_arn, policy_from_file, policy_to_file, PolicyDocument,
};

#[derive(Default)]
pub struct Merge {
    files: Vec<String>,
    arns: Vec<String>,
    out: String,
    all: String,
}

impl Arguments for Merge {
    fn set_option_args(&mut self, args: &HashMap<String, Vec<String>>) -> Result<(), String> {
        if let Some(out) = args.get("out") {
            self.out = out
                .get(0)
                .ok_or("Missing value for --out option".to_string())?
                .to_string();
        }
        if let Some(files) = args.get("file") {
            self.files = files.clone();
        }
        if let Some(arns) = args.get("arn") {
            self.arns = arns.clone();
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
        documents.extend(arns_to_documents(&self.arns)?);

        if !self.all.is_empty() {
            let directory = read_dir(self.all.as_str())
                .map_err(|e| format!("Failed to read directory: {}", e))?;
            let files = get_json_files(directory);
            documents.extend(files_to_documents(&files)?);
        }

        let mut result = merge_documents(&documents)?;
        result.reduce();
        result.sort();

        if self.out.is_empty() {
            println!("{}", serde_json::to_string_pretty(&result).unwrap());
            return Ok(());
        }
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
        vec!["file".to_string(), "out".to_string(), "all".to_string()]
    }
}

fn files_to_documents(files: &[String]) -> Result<Vec<PolicyDocument>, String> {
    files
        .iter()
        .map(|file| policy_from_file(file.as_str()))
        .collect()
}

fn arns_to_documents(arns: &[String]) -> Result<Vec<PolicyDocument>, String> {
    arns.iter()
        .map(|arn| policy_from_arn(arn.as_str()))
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

fn merge_documents(documents: &[PolicyDocument]) -> Result<PolicyDocument, String> {
    if documents.is_empty() {
        return Err("No documents to merge".to_string());
    }

    let result = merge_policy_documents(documents)?;
    Ok(result)
}
