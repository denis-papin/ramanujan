use ramanujan_oracle_tu::{config::AppConfig, db::OracleClient};

#[tokio::test]
async fn f_015_player_pkg_increment_goals_increments_goals() {
    let config =
        AppConfig::load_default().expect("env.toml must exist and contain Oracle connection data");
    let client = OracleClient::new(config.oracle);

    let goals = client
        .increment_goals_roundtrip_for_test()
        .await
        .expect("player_pkg.increment_goals should increment goals");

    assert_eq!(goals, 1);
}

#[tokio::test]
async fn f_015_player_pkg_increment_goals_generates_awards_every_three_goals() {
    let config =
        AppConfig::load_default().expect("env.toml must exist and contain Oracle connection data");
    let client = OracleClient::new(config.oracle);

    let result = client
        .increment_goals_with_awards_roundtrip_for_test(2)
        .await
        .expect("player_pkg.increment_goals should generate awards on 3-goal boundaries");

    assert_eq!(result.goals, 3);
    assert_eq!(result.awards, 1);
}

#[tokio::test]
async fn f_015_player_pkg_increment_goals_case_1() {
    let config =
        AppConfig::load_default().expect("env.toml must exist and contain Oracle connection data");
    let client = OracleClient::new(config.oracle);

    let results = client
        .increment_goals_steps_for_test(0, 3)
        .await
        .expect("increment_goals case 1 should execute");

    assert_eq!(results.len(), 3);
    assert_eq!(results[0].goals, 1);
    assert_eq!(results[0].awards, 0);
    assert_eq!(results[1].goals, 2);
    assert_eq!(results[1].awards, 0);
    assert_eq!(results[2].goals, 3);
    assert_eq!(results[2].awards, 1);
}

#[tokio::test]
async fn f_015_player_pkg_increment_goals_case_2() {
    let config =
        AppConfig::load_default().expect("env.toml must exist and contain Oracle connection data");
    let client = OracleClient::new(config.oracle);

    let results = client
        .increment_goals_steps_for_test(0, 10)
        .await
        .expect("increment_goals case 2 should execute");

    let final_result = results
        .last()
        .expect("case 2 should return at least one result");

    assert_eq!(final_result.goals, 10);
    assert_eq!(final_result.awards, 3);
}

#[tokio::test]
async fn f_015_player_pkg_increment_goals_case_3() {
    let config =
        AppConfig::load_default().expect("env.toml must exist and contain Oracle connection data");
    let client = OracleClient::new(config.oracle);

    let results = client
        .increment_goals_steps_for_test(24, 3)
        .await
        .expect("increment_goals case 3 should execute");

    assert_eq!(results.len(), 3);
    assert_eq!(results[0].goals, 25);
    assert_eq!(results[0].awards, 8);
    assert_eq!(results[2].goals, 27);
    assert_eq!(results[2].awards, 9);
}

#[tokio::test]
async fn f_015_player_pkg_increment_goals_case_4() {
    let config =
        AppConfig::load_default().expect("env.toml must exist and contain Oracle connection data");
    let client = OracleClient::new(config.oracle);

    let error = client
        .increment_goals_error_for_id(-1)
        .await
        .expect("increment_goals case 4 should capture the Oracle error");

    let message = error.to_string();

    assert!(message.contains("ORA-20003"));
    assert!(message.contains("Player not found for id -1"));
}

#[tokio::test]
async fn f_015_player_pkg_increment_goals_case_5() {
    let config =
        AppConfig::load_default().expect("env.toml must exist and contain Oracle connection data");
    let client = OracleClient::new(config.oracle);

    let error = client
        .increment_goals_error_for_null_id()
        .await
        .expect("increment_goals case 5 should capture the Oracle error");

    let message = error.to_string();

    assert!(message.contains("ORA-20001"));
    assert!(message.contains("p_player_id must not be null"));
}
