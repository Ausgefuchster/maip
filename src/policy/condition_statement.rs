use std::collections::HashMap;

use serde::{ser::SerializeMap, Deserialize, Deserializer, Serialize};
use serde_json::Value;

#[derive(Debug, PartialEq, Clone)]
pub struct ConditionStatement {
    condition_operator: String,
    conditions: Vec<Condition>,
}

impl ConditionStatement {
    pub fn new(condition_operator: String, conditions: Vec<Condition>) -> Self {
        Self {
            condition_operator,
            conditions,
        }
    }
}

impl<'de> Deserialize<'de> for ConditionStatement {
    fn deserialize<D>(deserializer: D) -> Result<ConditionStatement, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ConditionStatementVisitor)
    }
}

struct ConditionStatementVisitor;

impl<'de> serde::de::Visitor<'de> for ConditionStatementVisitor {
    type Value = ConditionStatement;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a condition statement")
    }

    fn visit_map<V>(self, mut map: V) -> Result<ConditionStatement, V::Error>
    where
        V: serde::de::MapAccess<'de>,
    {
        let condition_operator = map.next_key::<String>()?.unwrap();
        let condition_object = map.next_value::<HashMap<String, Value>>()?;

        let conditions: Vec<Condition> = condition_object
            .iter()
            .map(|(k, v)| condition_from_key_and_value(k, v))
            .collect();

        Ok(ConditionStatement::new(condition_operator, conditions))
    }
}

impl Serialize for ConditionStatement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        map.serialize_key(&self.condition_operator)?;

        self.conditions
            .iter()
            .for_each(|condition| map.serialize_value(condition).unwrap());

        map.end()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Condition {
    condition_key: String,
    condition_value: Vec<String>,
}

impl Condition {
    pub fn new(condition_key: String, condition_value: Vec<String>) -> Self {
        Self {
            condition_key,
            condition_value,
        }
    }
}

impl Serialize for Condition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        map.serialize_key(&self.condition_key)?;
        map.serialize_value(&self.condition_value)?;
        map.end()
    }
}

fn condition_from_key_and_value(key: &str, value: &Value) -> Condition {
    if let Some(value) = value.as_array() {
        Condition::new(
            key.to_string(),
            value
                .iter()
                .map(|v| v.as_str().unwrap().to_string())
                .collect(),
        )
    } else {
        Condition::new(key.to_string(), vec![value.as_str().unwrap().to_string()])
    }
}

#[cfg(test)]
mod deserialize_tests {
    use super::*;

    #[test]
    fn test_deserialize_simple_condition_statement() {
        let json = r#"{
            "StringEquals": {
                "iam:AWSServiceName": [
                    "autoscaling.amazonaws.com",
                    "ec2scheduled.amazonaws.com",
                    "elasticloadbalancing.amazonaws.com",
                    "spot.amazonaws.com",
                    "spotfleet.amazonaws.com",
                    "transitgateway.amazonaws.com"
                ]
            }
        }"#;

        let condition_statement: ConditionStatement = serde_json::from_str(json).unwrap();

        let expected = ConditionStatement::new(
            "StringEquals".to_string(),
            vec![Condition::new(
                "iam:AWSServiceName".to_string(),
                vec![
                    "autoscaling.amazonaws.com".to_string(),
                    "ec2scheduled.amazonaws.com".to_string(),
                    "elasticloadbalancing.amazonaws.com".to_string(),
                    "spot.amazonaws.com".to_string(),
                    "spotfleet.amazonaws.com".to_string(),
                    "transitgateway.amazonaws.com".to_string(),
                ],
            )],
        );
        assert_eq!(condition_statement, expected);
    }

    #[test]
    fn test_deserialize_condition_statement_one_value() {
        let json = r#"
        {
            "StringEquals": {
                "iam:AWSServiceName": "autoscaling.amazonaws.com"
            }
        }
        "#;

        let condition_statement: ConditionStatement = serde_json::from_str(json).unwrap();

        let expected = ConditionStatement::new(
            "StringEquals".to_string(),
            vec![Condition::new(
                "iam:AWSServiceName".to_string(),
                vec!["autoscaling.amazonaws.com".to_string()],
            )],
        );
        assert_eq!(condition_statement, expected);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_condition_statement() {
        let condition_statement = ConditionStatement::new(
            "StringEquals".to_string(),
            vec![Condition::new(
                "iam:AWSServiceName".to_string(),
                vec![
                    "autoscaling.amazonaws.com".to_string(),
                    "ec2scheduled.amazonaws.com".to_string(),
                    "elasticloadbalancing.amazonaws.com".to_string(),
                    "spot.amazonaws.com".to_string(),
                    "spotfleet.amazonaws.com".to_string(),
                    "transitgateway.amazonaws.com".to_string(),
                ],
            )],
        );

        let json = serde_json::to_string(&condition_statement).unwrap();

        assert_eq!(
            json,
            r#"{"StringEquals":{"iam:AWSServiceName":["autoscaling.amazonaws.com","ec2scheduled.amazonaws.com","elasticloadbalancing.amazonaws.com","spot.amazonaws.com","spotfleet.amazonaws.com","transitgateway.amazonaws.com"]}}"#
        );
    }

    #[test]
    fn test_serialize_condition_one_value() {
        let condition_statement = ConditionStatement::new(
            "StringEquals".to_string(),
            vec![Condition::new(
                "iam:AWSServiceName".to_string(),
                vec!["autoscaling.amazonaws.com".to_string()],
            )],
        );

        let json = serde_json::to_string(&condition_statement).unwrap();

        assert_eq!(
            json,
            r#"{"StringEquals":{"iam:AWSServiceName":"autoscaling.amazonaws.com"}}"#
        );
    }
}
