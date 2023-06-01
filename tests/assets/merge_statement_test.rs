use maip::policy::{merge_statements, PolicyStatement};

#[test]
fn test_merge_statements_succsessful() {
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

// #[test]
// fn test_merge_statement_successful() {
//     let ec2_policy = File::open("./tests/assets/AmazonEC2FullAccessPolicy.json").unwrap();
//     let ec2_policy: serde_json::Value =
//         serde_json::from_reader(ec2_policy).expect("file should be proper JSON");

//     let ec2_policy: PolicyDocument = serde_json::from_value(ec2_policy).unwrap();

//     let rds_policy = File::open("./tests/assets/AmazonRDSFullAccessPolicy.json").unwrap();
//     let rds_policy: serde_json::Value =
//         serde_json::from_reader(rds_policy).expect("file should be proper JSON");

//     let rds_policy: PolicyDocument = serde_json::from_value(rds_policy).unwrap();

//     println!("{:#?}", rds_policy);
//     println!("{:#?}", ec2_policy);
// }
