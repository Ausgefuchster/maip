use std::collections::HashMap;

use serde::{Deserialize, de::Error};
use serde_json::Value;

use super::{PolicyStatement, Condition, ConditionStatement};

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