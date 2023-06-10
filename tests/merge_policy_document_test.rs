use std::fs::read_to_string;

use maip::policy::{merge_policy_documents, ConditionStatement, PolicyDocument, PolicyStatement};

#[test]
fn test_merge_ec2_and_rds_policy() {
    let ec2_policy = read_to_string("./tests/assets/AmazonEC2FullAccessPolicy.json").unwrap();
    let ec2_policy: PolicyDocument = serde_json::from_str(&ec2_policy).unwrap();

    let rds_policy = read_to_string("./tests/assets/AmazonRDSFullAccessPolicy.json").unwrap();
    let rds_policy: PolicyDocument = serde_json::from_str(&rds_policy).unwrap();

    let merged_policy_document = merge_policy_documents(&[ec2_policy, rds_policy]);

    let expected_policy_document = PolicyDocument::new(
        "2012-10-17".to_string(),
        vec![
            PolicyStatement::new(
                "Allow".to_string(),
                vec![
                    "application-autoscaling:DeleteScalingPolicy".to_string(),
                    "application-autoscaling:DeregisterScalableTarget".to_string(),
                    "application-autoscaling:DescribeScalableTargets".to_string(),
                    "application-autoscaling:DescribeScalingActivities".to_string(),
                    "application-autoscaling:DescribeScalingPolicies".to_string(),
                    "application-autoscaling:PutScalingPolicy".to_string(),
                    "application-autoscaling:RegisterScalableTarget".to_string(),
                    "autoscaling:*".to_string(),
                    "cloudwatch:*".to_string(),
                    "cloudwatch:DeleteAlarms".to_string(),
                    "cloudwatch:DescribeAlarms".to_string(),
                    "cloudwatch:GetMetricData".to_string(),
                    "cloudwatch:GetMetricStatistics".to_string(),
                    "cloudwatch:ListMetrics".to_string(),
                    "cloudwatch:PutMetricAlarm".to_string(),
                    "devops-guru:GetResourceCollection".to_string(),
                    "ec2:*".to_string(),
                    "ec2:DescribeAccountAttributes".to_string(),
                    "ec2:DescribeAvailabilityZones".to_string(),
                    "ec2:DescribeCoipPools".to_string(),
                    "ec2:DescribeInternetGateways".to_string(),
                    "ec2:DescribeLocalGatewayRouteTablePermissions".to_string(),
                    "ec2:DescribeLocalGatewayRouteTables".to_string(),
                    "ec2:DescribeLocalGatewayRouteTableVpcAssociations".to_string(),
                    "ec2:DescribeLocalGateways".to_string(),
                    "ec2:DescribeSecurityGroups".to_string(),
                    "ec2:DescribeSubnets".to_string(),
                    "ec2:DescribeVpcAttribute".to_string(),
                    "ec2:DescribeVpcs".to_string(),
                    "ec2:GetCoipPoolUsage".to_string(),
                    "elasticloadbalancing:*".to_string(),
                    "logs:DescribeLogStreams".to_string(),
                    "logs:GetLogEvents".to_string(),
                    "outposts:GetOutpostInstanceTypes".to_string(),
                    "rds:*".to_string(),
                    "sns:ListSubscriptions".to_string(),
                    "sns:ListTopics".to_string(),
                    "sns:Publish".to_string(),
                ],
                vec!["*".to_string()],
                None,
            ),
            PolicyStatement::new(
                "Allow".to_string(),
                vec![
                    "devops-guru:ListAnomaliesForInsight".to_string(),
                    "devops-guru:SearchInsights".to_string(),
                ],
                vec!["*".to_string()],
                Some(ConditionStatement {}),
            ),
            PolicyStatement::new(
                "Allow".to_string(),
                vec!["iam:CreateServiceLinkedRole".to_string()],
                vec!["*".to_string()],
                Some(ConditionStatement {}),
            ),
            PolicyStatement::new(
                "Allow".to_string(),
                vec!["iam:CreateServiceLinkedRole".to_string()],
                vec!["*".to_string()],
                Some(ConditionStatement {}),
            ),
            PolicyStatement::new(
                "Allow".to_string(),
                vec!["pi:*".to_string()],
                vec!["arn:aws:pi:*:*:metrics/rds/*".to_string()],
                None,
            ),
        ],
    );

    assert_eq!(merged_policy_document.unwrap(), expected_policy_document);
}
