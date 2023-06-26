use std::collections::HashMap;

use serde::{de::Error, Deserialize, Serialize};
use serde_json::Value;

use super::{merge::Merge, Condition, ConditionStatement};

#[derive(Serialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PolicyStatement {
    pub effect: String,

    #[serde(deserialize_with = "string_or_seq_string")]
    pub action: Vec<String>,

    #[serde(deserialize_with = "string_or_seq_string")]
    pub resource: Vec<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    condition: Vec<ConditionStatement>,
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

impl<'de> Deserialize<'de> for PolicyStatement {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(PolicyStatementVisitor)
    }
}

struct PolicyStatementVisitor;

impl<'de> serde::de::Visitor<'de> for PolicyStatementVisitor {
    type Value = PolicyStatement;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a policy statement")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut effect = None;
        let mut action = None;
        let mut resource = None;
        let mut condition = None;

        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "Effect" => {
                    effect = Some(map.next_value::<String>()?);
                }
                "Action" => {
                    let value: Value = map.next_value().unwrap();
                    action = Some(get_value_as_vec(&value));
                }
                "Resource" => {
                    let value: Value = map.next_value().unwrap();
                    resource = Some(get_value_as_vec(&value));
                }
                "Condition" => {
                    let conditions = map
                        .next_value::<HashMap<String, HashMap<String, Value>>>()?
                        .iter()
                        .map(|(key, condition)| {
                            (
                                key,
                                condition
                                    .iter()
                                    .map(|(condition_key, condition_value)| {
                                        Condition::new(
                                            condition_key.to_owned(),
                                            get_value_as_vec(condition_value),
                                        )
                                    })
                                    .collect::<Vec<Condition>>(),
                            )
                        })
                        .map(|(operator, condition)| {
                            ConditionStatement::new(operator.to_owned(), condition)
                        })
                        .collect::<Vec<ConditionStatement>>();
                    condition = Some(conditions);
                }
                _ => {
                    return Err(Error::unknown_field(&key, &[]));
                }
            }
        }

        let effect = effect.ok_or_else(|| Error::missing_field("Effect"))?;
        let action = action.ok_or_else(|| Error::missing_field("Action"))?;
        let resource = resource.ok_or_else(|| Error::missing_field("Resource"))?;
        let condition = condition.unwrap_or_default();

        Ok(PolicyStatement {
            effect,
            action,
            resource,
            condition,
        })
    }
}

fn get_value_as_vec(value: &Value) -> Vec<String> {
    match value {
        Value::Array(array) => array
            .iter()
            .map(|v| v.as_str().unwrap().to_owned())
            .collect::<Vec<String>>(),
        Value::String(string) => vec![string.to_owned()],
        _ => panic!("Invalid value type"),
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
