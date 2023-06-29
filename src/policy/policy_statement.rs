use serde::Serialize;

use super::{merge::Merge, ConditionStatement};

#[derive(Serialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PolicyStatement {
    pub effect: String,

    #[serde(deserialize_with = "string_or_seq_string")]
    pub action: Vec<String>,

    #[serde(deserialize_with = "string_or_seq_string")]
    pub resource: Vec<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub condition: Vec<ConditionStatement>,
}

impl PolicyStatement {
    pub fn new(
        effect: String,
        action: Vec<String>,
        resource: Vec<String>,
        condition: Vec<ConditionStatement>,
    ) -> Self {
        Self {
            effect,
            action,
            resource,
            condition,
        }
    }
}

pub fn merge_statements(
    first_statement: &PolicyStatement,
    second_statement: &PolicyStatement,
) -> Option<PolicyStatement> {
    if !first_statement.condition.is_empty() || !second_statement.condition.is_empty() {
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
    first_resources == second_resources
}

fn get_asterisk(first_resources: &[String], second_resources: &[String]) -> Option<String> {
    if first_resources.contains(&"*".to_string()) || second_resources.contains(&"*".to_string()) {
        return Some("*".to_string());
    }
    None
}
