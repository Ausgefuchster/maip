use maip::policy::{merge_statements, PolicyStatement};

#[test]
fn test_merge_allow_statements() {
    let first_statement = PolicyStatement::new(
        "Allow".to_string(),
        vec!["ec2:Describe*".to_string()],
        vec!["*".to_string()],
    );
    let second_statement = PolicyStatement::new(
        "Allow".to_string(),
        vec!["rds:Describe*".to_string()],
        vec!["*".to_string()],
    );
    let merged_statement = merge_statements(&first_statement, &second_statement);

    assert!(merged_statement.is_some());

    let expected_statement = PolicyStatement::new(
        "Allow".to_string(),
        vec!["ec2:Describe*".to_string(), "rds:Describe*".to_string()],
        vec!["*".to_string()],
    );
    assert_eq!(merged_statement.unwrap(), expected_statement);
}

#[test]
fn test_merge_different_resource() {
    let first_statement = PolicyStatement::new(
        "Allow".to_string(),
        vec!["ec2:Describe*".to_string()],
        vec!["*".to_string()],
    );
    let second_statement = PolicyStatement::new(
        "Allow".to_string(),
        vec!["ec2:Describe*".to_string()],
        vec!["arn:aws:ec2:us-east-1:123456789012:instance/*".to_string()],
    );
    let merged_statement = merge_statements(&first_statement, &second_statement);

    let expected_statement = PolicyStatement::new(
        "Allow".to_string(),
        vec!["ec2:Describe*".to_string()],
        vec!["*".to_string()],
    );
    assert_eq!(merged_statement.unwrap(), expected_statement);
}

#[test]
fn test_merge_allow_and_deny_same() {
    let first_statement = PolicyStatement::new(
        "Allow".to_string(),
        vec!["ec2:Describe*".to_string()],
        vec!["*".to_string()],
    );
    let second_statement = PolicyStatement::new(
        "Deny".to_string(),
        vec!["ec2:Describe*".to_string()],
        vec!["*".to_string()],
    );
    let merged_statement = merge_statements(&first_statement, &second_statement);

    let expected_statement = PolicyStatement::new(
        "Deny".to_string(),
        vec!["ec2:Describe*".to_string()],
        vec!["*".to_string()],
    );
    assert_eq!(merged_statement.unwrap(), expected_statement);
}

#[test]
fn test_merge_allow_and_deny_different() {
    let first_statement = PolicyStatement::new(
        "Allow".to_string(),
        vec!["ec2:*".to_string()],
        vec!["*".to_string()],
    );
    let second_statement = PolicyStatement::new(
        "Deny".to_string(),
        vec!["ec2:Describe*".to_string()],
        vec!["*".to_string()],
    );
    let merged_statement = merge_statements(&first_statement, &second_statement);

    assert!(merged_statement.is_none());
}