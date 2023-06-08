use maip::policy::{merge_statements, ConditionStatement, PolicyStatement};

#[test]
fn test_merge_allow_different_action_same_resource() {
    let first_statement = PolicyStatement::new(
        "Allow".to_string(),
        vec!["ec2:Describe*".to_string()],
        vec!["*".to_string()],
        None,
    );
    let second_statement = PolicyStatement::new(
        "Allow".to_string(),
        vec!["rds:Describe*".to_string()],
        vec!["*".to_string()],
        None,
    );
    let merged_statement = merge_statements(&first_statement, &second_statement);

    assert!(merged_statement.is_some());

    let expected_statement = PolicyStatement::new(
        "Allow".to_string(),
        vec!["ec2:Describe*".to_string(), "rds:Describe*".to_string()],
        vec!["*".to_string()],
        None,
    );
    assert_eq!(merged_statement.unwrap(), expected_statement);
}

#[test]
fn test_merge_allow_same_action_different_resource_no_asterisk() {
    let first_statement = PolicyStatement::new(
        "Allow".to_string(),
        vec!["ec2:Describe*".to_string()],
        vec!["arn:aws:ec2:us-east-1:123456789012:instance/*".to_string()],
        None,
    );
    let second_statement = PolicyStatement::new(
        "Allow".to_string(),
        vec!["ec2:Describe*".to_string()],
        vec!["arn:aws:ec2:us-east-1:123456789012:volume/*".to_string()],
        None,
    );

    let merged_statement = merge_statements(&first_statement, &second_statement);

    let expected_statement = PolicyStatement::new(
        "Allow".to_string(),
        vec!["ec2:Describe*".to_string()],
        vec![
            "arn:aws:ec2:us-east-1:123456789012:instance/*".to_string(),
            "arn:aws:ec2:us-east-1:123456789012:volume/*".to_string(),
        ],
        None,
    );
    assert_eq!(merged_statement.unwrap(), expected_statement);
}

#[test]
fn test_merge_allow_different_action_and_resource() {
    let first_statement = PolicyStatement::new(
        "Allow".to_string(),
        vec!["ec2:Describe*".to_string()],
        vec!["arn:aws:ec2:us-east-1:123456789012:instance/*".to_string()],
        None,
    );
    let second_statement = PolicyStatement::new(
        "Allow".to_string(),
        vec!["rds:Describe*".to_string()],
        vec!["arn:aws:rds:us-east-1:123456789012:db/*".to_string()],
        None,
    );

    let merged_statement = merge_statements(&first_statement, &second_statement);

    assert!(merged_statement.is_none());
}

#[test]
fn test_merge_specific_resource_and_asterisk() {
    let first_statement = PolicyStatement::new(
        "Allow".to_string(),
        vec!["pi:*".to_string()],
        vec!["arn:aws:pi:*:*:metrics/rds/*".to_string()],
        None,
    );

    let second_statement = PolicyStatement::new(
        "Allow".to_string(),
        vec!["rds:*".to_string()],
        vec!["*".to_string()],
        None,
    );

    let merged_statement = merge_statements(&first_statement, &second_statement);
    assert!(merged_statement.is_none())
}
#[test]
fn test_merge_different_resource() {
    let first_statement = PolicyStatement::new(
        "Allow".to_string(),
        vec!["ec2:Describe*".to_string()],
        vec!["*".to_string()],
        None,
    );
    let second_statement = PolicyStatement::new(
        "Allow".to_string(),
        vec!["ec2:Describe*".to_string()],
        vec!["arn:aws:ec2:us-east-1:123456789012:instance/*".to_string()],
        None,
    );
    let merged_statement = merge_statements(&first_statement, &second_statement);

    let expected_statement = PolicyStatement::new(
        "Allow".to_string(),
        vec!["ec2:Describe*".to_string()],
        vec!["*".to_string()],
        None,
    );
    assert_eq!(merged_statement.unwrap(), expected_statement);
}

#[test]
fn test_merge_allow_and_deny_same() {
    let first_statement = PolicyStatement::new(
        "Allow".to_string(),
        vec!["ec2:Describe*".to_string()],
        vec!["*".to_string()],
        None,
    );
    let second_statement = PolicyStatement::new(
        "Deny".to_string(),
        vec!["ec2:Describe*".to_string()],
        vec!["*".to_string()],
        None,
    );
    let merged_statement = merge_statements(&first_statement, &second_statement);

    let expected_statement = PolicyStatement::new(
        "Deny".to_string(),
        vec!["ec2:Describe*".to_string()],
        vec!["*".to_string()],
        None,
    );
    assert_eq!(merged_statement.unwrap(), expected_statement);
}

#[test]
fn test_merge_allow_and_deny_different() {
    let first_statement = PolicyStatement::new(
        "Allow".to_string(),
        vec!["ec2:*".to_string()],
        vec!["*".to_string()],
        None,
    );
    let second_statement = PolicyStatement::new(
        "Deny".to_string(),
        vec!["ec2:Describe*".to_string()],
        vec!["*".to_string()],
        None,
    );
    let merged_statement = merge_statements(&first_statement, &second_statement);

    assert!(merged_statement.is_none());
}

#[test]
fn test_merge_different_actions_and_resources() {
    let first_statement = PolicyStatement::new(
        "Allow".to_string(),
        vec!["ec2:Describe*".to_string()],
        vec!["arn:aws:ec2:us-east-1:123456789012:instance/*".to_string()],
        None,
    );
    let second_statement = PolicyStatement::new(
        "Allow".to_string(),
        vec!["ec2:*".to_string()],
        vec!["arn:aws:ec2:us-east-1:123456789012:volume/*".to_string()],
        None,
    );
    let merged_statement = merge_statements(&first_statement, &second_statement);

    assert!(merged_statement.is_none());
}

#[test]
fn test_merge_allow_all_same_with_condition() {
    let first_statement = PolicyStatement::new(
        "Allow".to_string(),
        vec!["ec2:*".to_string()],
        vec!["*".to_string()],
        Some(ConditionStatement {}),
    );
    let second_statement = PolicyStatement::new(
        "Allow".to_string(),
        vec!["ec2:*".to_string()],
        vec!["*".to_string()],
        Some(ConditionStatement {}),
    );

    let merged_statement = merge_statements(&first_statement, &second_statement);
    assert!(merged_statement.is_none());
}

#[test]
fn test_merge_allow_all_same_one_has_condition() {
    let first_statement = PolicyStatement::new(
        "Allow".to_string(),
        vec!["ec2:*".to_string()],
        vec!["*".to_string()],
        None,
    );
    let second_statement = PolicyStatement::new(
        "Allow".to_string(),
        vec!["ec2:*".to_string()],
        vec!["*".to_string()],
        Some(ConditionStatement {}),
    );

    let merged_statement = merge_statements(&first_statement, &second_statement);
    assert!(merged_statement.is_none());
}

#[test]
fn test_merge_multiple_actions_same_resource() {
    let first_statement = PolicyStatement::new(
        "Allow".to_string(),
        vec!["ec2:Describe*".to_string(), "ec2:Create*".to_string()],
        vec!["arn:aws:ec2:us-east-1:123456789012:instance/*".to_string()],
        None,
    );
    let second_statement = PolicyStatement::new(
        "Allow".to_string(),
        vec!["ec2:Describe*".to_string(), "ec2:Delete*".to_string()],
        vec!["arn:aws:ec2:us-east-1:123456789012:instance/*".to_string()],
        None,
    );

    let merged_statement = merge_statements(&first_statement, &second_statement);

    let expected_statement = PolicyStatement::new(
        "Allow".to_string(),
        vec![
            "ec2:Describe*".to_string(),
            "ec2:Create*".to_string(),
            "ec2:Delete*".to_string(),
        ],
        vec!["arn:aws:ec2:us-east-1:123456789012:instance/*".to_string()],
        None,
    );
    assert_eq!(merged_statement.unwrap(), expected_statement);
}

#[test]
fn merge_same_action_different_resources() {
    let first_statement = PolicyStatement::new(
        "Allow".to_string(),
        vec!["ec2:Describe*".to_string()],
        vec![
            "arn:aws:ec2:us-east-1:123456789012:instance/*".to_string(),
            "arn:aws:ec2:us-east-1:123456789012:volume/*".to_string(),
        ],
        None,
    );
    let second_statement = PolicyStatement::new(
        "Allow".to_string(),
        vec!["ec2:Describe*".to_string()],
        vec![
            "arn:aws:ec2:us-east-1:123456789012:instance/*".to_string(),
            "arn:aws:ec2:us-east-1:123456789012:subnet/*".to_string(),
        ],
        None,
    );

    let merged_statement = merge_statements(&first_statement, &second_statement);

    let expected_statement = PolicyStatement::new(
        "Allow".to_string(),
        vec!["ec2:Describe*".to_string()],
        vec![
            "arn:aws:ec2:us-east-1:123456789012:instance/*".to_string(),
            "arn:aws:ec2:us-east-1:123456789012:volume/*".to_string(),
            "arn:aws:ec2:us-east-1:123456789012:subnet/*".to_string(),
        ],
        None,
    );
    assert_eq!(merged_statement.unwrap(), expected_statement);
}