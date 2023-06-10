use maip::policy::{
    merge_policy_documents, policy_from_file, policy_to_file, PolicyDocument,
};

use crate::Command;

pub struct MergeCommand {}

impl Command for MergeCommand {
    fn get_name(&self) -> &str {
        "merge"
    }

    fn get_description(&self) -> &str {
        "Merge IAM policy documents"
    }

    fn get_arguments(&self) -> Vec<String> {
        vec!["file".to_string(), "managed-policy".to_string()]
    }

    fn get_subcommands(&self) -> Vec<Box<dyn Command>> {
        vec![]
    }

    fn get_aliases(&self) -> &[String] {
        &[]
    }

    fn execute(&self, args: &[String]) {
        let policy_documents: Vec<PolicyDocument> = args[..args.len() - 1]
            .iter()
            .map(|arg| match policy_from_file(arg) {
                Ok(policy) => policy,
                Err(e) => {
                    println!("Error: {}", e);
                    std::process::exit(1);
                }
            })
            .collect();

        let merged_document = merge_policy_documents(policy_documents.as_slice()).unwrap();

        policy_to_file(args[args.len() - 1].as_str(), &merged_document).unwrap();
    }
}
