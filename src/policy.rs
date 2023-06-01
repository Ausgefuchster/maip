use serde::{Deserialize, Serialize};

use crate::json_string_or_vec::string_or_seq_string;

#[derive(Serialize, Deserialize, Debug)]
pub struct PolicyDocument {
    #[serde(alias = "Version")]
    version: String,
    #[serde(alias = "Statement")]
    pub statement: Vec<PolicyStatement>,
}

impl PolicyDocument {
    pub fn new(version: String, statement: Vec<PolicyStatement>) -> Self {
        Self { version, statement }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PolicyStatement {
    #[serde(alias = "Effect")]
    effect: String,
    #[serde(alias = "Action", deserialize_with = "string_or_seq_string")]
    action: Vec<String>,
    #[serde(alias = "Resource", deserialize_with = "string_or_seq_string")]
    resource: Vec<String>,
}

impl PolicyStatement {
    pub fn new(effect: String, action: Vec<String>, resource: Vec<String>) -> Self {
        Self {
            effect,
            action,
            resource,
        }
    }
}

trait Merge<T> {
    fn merge(&mut self, other: T);
}

impl Merge<Vec<String>> for Vec<String> {
    fn merge(&mut self, other: Vec<String>) {
        other.iter().for_each(|x| {
            if !self.contains(x) {
                self.push(x.clone());
            }
        });
    }
}

pub fn merge_statements(
    first_statement: &PolicyStatement,
    second_statement: &PolicyStatement,
) -> Option<PolicyStatement> {
    if first_statement.effect != second_statement.effect {
        return None;
    }

    let mut merged_statement = PolicyStatement::new(
        first_statement.effect.clone(),
        first_statement.action.clone(),
        first_statement.resource.clone(),
    );

    merged_statement
        .action
        .extend(second_statement.action.clone());

    merged_statement
        .resource
        .merge(second_statement.resource.clone());

    Some(merged_statement)
}
