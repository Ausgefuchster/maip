use serde::{Deserialize, Serialize};

use crate::json_string_or_vec::string_or_seq_string;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PolicyDocument {
    version: String,
    statement: Vec<PolicyStatement>,
}

impl PolicyDocument {
    pub fn new(version: String, statement: Vec<PolicyStatement>) -> Self {
        Self { version, statement }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PolicyStatement {
    effect: String,

    #[serde(deserialize_with = "string_or_seq_string")]
    action: Vec<String>,

    #[serde(deserialize_with = "string_or_seq_string")]
    resource: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<ConditionStatement>,
}

impl PolicyStatement {
    pub fn new(
        effect: String,
        action: Vec<String>,
        resource: Vec<String>,
        condition: Option<ConditionStatement>,
    ) -> Self {
        Self {
            effect,
            action,
            resource,
            condition,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ConditionStatement {}

pub trait Merge<T> {
    /// Merges the other value into self
    /// # Example
    /// ```
    /// use maip::policy::Merge;
    ///
    /// let mut a: Vec<String> = vec!["a".to_string(), "b".to_string()];
    /// let b = vec!["b".to_string(), "c".to_string()];
    /// a.merge(b);
    /// assert_eq!(a, vec!["a".to_string(), "b".to_string(), "c".to_string()]);
    /// ```
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

/// Merges the two statements together
/// - If one statement has a condition, it will return None
/// - If they have a different effect, but action and resource are the same,
///   return a deny statement because deny always overrides allow
/// - Otherwise, If they have the same action it will merge the resources.
///   Furthermore, if one list contains an asterisk, it will return a statement with
///   only an asterisk as resource.
/// - But if they have different actions, it will only merge the resources if they
///   are equal
pub fn merge_statements(
    first_statement: &PolicyStatement,
    second_statement: &PolicyStatement,
) -> Option<PolicyStatement> {
    if first_statement.condition.is_some() || second_statement.condition.is_some() {
        return None;
    }

    if first_statement.effect != second_statement.effect {
        if same_action_and_resource(first_statement, second_statement) {
            return as_deny_statement(first_statement);
        }
        return None;
    }

    let mut merged_statement = first_statement.clone();

    let first_resource = &first_statement.resource;
    let second_resource = &second_statement.resource;

    let first_action = &first_statement.action;
    let second_action = &second_statement.action;

    let asterik = get_asterisk(first_resource, second_resource);
    if first_action == second_action {
        if let Some(asterisk) = asterik {
            merged_statement.resource = vec![asterisk];
            return Some(merged_statement);
        }
        merged_statement.resource.merge(second_resource.clone());
        return Some(merged_statement);
    }

    if can_merge_resources(first_resource, second_resource) {
        merged_statement.action.merge(second_action.clone());
        merged_statement.resource.merge(second_resource.clone());
        return Some(merged_statement);
    }
    None
}

fn same_action_and_resource(
    first_statement: &PolicyStatement,
    second_statement: &PolicyStatement,
) -> bool {
    first_statement.action == second_statement.action
        && first_statement.resource == second_statement.resource
}

fn as_deny_statement(statement: &PolicyStatement) -> Option<PolicyStatement> {
    Some(PolicyStatement::new(
        "Deny".to_string(),
        statement.action.clone(),
        statement.resource.clone(),
        statement.condition.clone(),
    ))
}

fn can_merge_resources(first_resources: &[String], second_resources: &[String]) -> bool {
    first_resources == second_resources || get_asterisk(first_resources, second_resources).is_some()
}

fn get_asterisk(first_resources: &[String], second_resources: &[String]) -> Option<String> {
    if first_resources.contains(&"*".to_string()) || second_resources.contains(&"*".to_string()) {
        return Some("*".to_string());
    }
    None
}
