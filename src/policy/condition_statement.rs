#[derive(Debug, Default, PartialEq, Clone)]
pub struct ConditionStatement {
    pub condition_operator: String,
    pub conditions: Vec<Condition>,
}

impl ConditionStatement {
    pub fn new(condition_operator: String, conditions: Vec<Condition>) -> Self {
        Self {
            condition_operator,
            conditions,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Condition {
    pub condition_key: String,
    pub condition_value: Vec<String>,
}

impl Condition {
    pub fn new(condition_key: String, condition_value: Vec<String>) -> Self {
        Self {
            condition_key,
            condition_value,
        }
    }
}
