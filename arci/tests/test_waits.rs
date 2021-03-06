use arci::{
    CompleteCondition, DummyJointTrajectoryClient, EachJointDiffCondition, JointTrajectoryClient,
    TotalJointDiffCondition,
};

#[test]
fn test_total_condition() {
    let client = DummyJointTrajectoryClient::new(vec!["j1".to_owned(), "j2".to_owned()]);
    let c1 = TotalJointDiffCondition::new(1.0, 0.1);
    assert!(c1.wait(&client, &[0.0, 0.0]).is_ok());
    assert!(c1.wait(&client, &[0.5, 0.0]).is_ok());
    assert!(c1.wait(&client, &[-0.5, 0.0]).is_ok());
    assert!(c1.wait(&client, &[-0.5, 0.8]).is_err());
    tokio_test::block_on(
        client.send_joint_positions(vec![3.0, -10.0], std::time::Duration::from_millis(100)),
    )
    .unwrap();
    assert!(c1.wait(&client, &[-0.5, 0.8]).is_err());
    assert!(c1.wait(&client, &[3.0, -10.0]).is_ok());
    assert!(c1.wait(&client, &[3.0, -10.5]).is_ok());
}

#[test]
fn test_each_condition() {
    let client = DummyJointTrajectoryClient::new(vec!["j1".to_owned(), "j2".to_owned()]);
    let c1 = EachJointDiffCondition::new(vec![1.0, 0.1], 0.1);
    assert!(c1.wait(&client, &[0.0, 0.0]).is_ok());
    assert!(c1.wait(&client, &[0.5, 0.0]).is_ok());
    assert!(c1.wait(&client, &[-0.5, 0.0]).is_ok());
    assert!(c1.wait(&client, &[-1.5, 0.0]).is_err());
    assert!(c1.wait(&client, &[-0.5, 0.2]).is_err());
    tokio_test::block_on(
        client.send_joint_positions(vec![3.0, -10.0], std::time::Duration::from_millis(100)),
    )
    .unwrap();
    assert!(c1.wait(&client, &[3.0, 0.8]).is_err());
    assert!(c1.wait(&client, &[3.0, -9.95]).is_ok());
    assert!(c1.wait(&client, &[3.0, -10.0]).is_ok());
    assert!(c1.wait(&client, &[3.5, -10.0]).is_ok());
}
