use std::{
    error::Error,
    fs::File,
    io::{BufReader, BufWriter},
};

use serde::{Deserialize, Serialize};

use super::policy_statement::{merge_statements, PolicyStatement};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct PolicyDocument {
    pub version: String,
    pub statement: Vec<PolicyStatement>,
}

impl PolicyDocument {
    pub fn new(version: String, statement: Vec<PolicyStatement>) -> Self {
        Self { version, statement }
    }

    /// Returns the number of characters in the policy document
    pub fn size(&self) -> usize {
        serde_json::to_string(self).unwrap().len()
    }

    /// Sorts the statements by effect
    /// Then by action length
    pub fn sort(&mut self) {
        self.statement
            .sort_by(|a, b| a.effect.cmp(&b.effect).then(a.action.cmp(&b.action)));
    }
}

pub fn merge_policy_documents(documents: &[PolicyDocument]) -> Option<PolicyDocument> {
    assert!(
        documents.iter().all(|d| d.version == "2012-10-17"),
        "Only version 2012-10-17 is supported"
    );

    let mut new_document = documents.iter().fold(
        PolicyDocument::new("2012-10-17".to_string(), Vec::new()),
        |mut acc, document| {
            acc.statement.extend(document.statement.clone());
            acc
        },
    );

    merge_policy_document_statements(&mut new_document);

    new_document.sort();
    Some(new_document)
}

pub fn merge_policy_document_statements(document: &mut PolicyDocument) {
    let mut merged_statements: Vec<PolicyStatement> = Vec::new();

    for statement in document.statement.iter() {
        let mut merged = false;
        for other_statement in merged_statements.iter_mut() {
            if let Some(merged_statement) = merge_statements(statement, other_statement) {
                *other_statement = merged_statement;
                other_statement.action.sort_by_key(|a| a.to_lowercase());
                merged = true;
                break;
            }
        }
        if !merged {
            let mut statement = statement.clone();
            statement.action.sort_by_key(|a| a.to_lowercase());
            merged_statements.push(statement);
        }
    }

    document.statement = merged_statements;
}

pub fn policy_from_file(file: &str) -> Result<PolicyDocument, Box<dyn Error>> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);
    let policy_document: PolicyDocument = serde_json::from_reader(reader)?;
    Ok(policy_document)
}

pub fn policy_to_file(file: &str, policy_document: &PolicyDocument) -> Result<(), Box<dyn Error>> {
    let file = File::create(file)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, policy_document)?;
    Ok(())
}
