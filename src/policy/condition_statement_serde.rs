use serde::{ser::SerializeMap, Serialize};

use super::{Condition, ConditionStatement};

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

impl Serialize for Condition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        map.serialize_key(&self.condition_key)?;
        if self.condition_value.len() == 1 {
            map.serialize_value(&self.condition_value[0])?;
        } else {
            map.serialize_value(&self.condition_value)?;
        }
        map.end()
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
