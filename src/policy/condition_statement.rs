#[derive(Debug, Default, PartialEq, Clone)]
pub struct ConditionStatement {
    pub operator: String,
    pub conditions: Vec<Condition>,
}

impl ConditionStatement {
    pub fn new(operator: String, conditions: Vec<Condition>) -> Self {
        Self {
            operator,
            conditions,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Condition {
    pub key: String,
    pub values: Vec<String>,
}

impl Condition {
    pub fn new(condition_key: String, condition_value: Vec<String>) -> Self {
        Self {
            key: condition_key,
            values: condition_value,
        }
    }
}
