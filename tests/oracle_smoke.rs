use ramanujan_oracle_tu::{config::AppConfig, db::OracleClient};

fn assert_valid_issuers(issuers: &[String]) {
    assert!(issuers.iter().all(|issuer| issuer == "GOLD" || issuer == "SILVER"));
}

#[tokio::test]
async fn f_015_player_pkg_increment_points_increments_points() {
    let config =
        AppConfig::load_default().expect("env.toml must exist and contain Oracle connection data");
    let client = OracleClient::new(config.oracle);

    let points = client
        .increment_points_roundtrip_for_test()
        .await
        .expect("player_pkg.increment_points should increment points");

    assert_eq!(points, 1);
}

#[tokio::test]
async fn f_015_player_pkg_increment_points_generates_awards_every_three_points() {
    let config =
        AppConfig::load_default().expect("env.toml must exist and contain Oracle connection data");
    let client = OracleClient::new(config.oracle);

    let result = client
        .increment_points_with_awards_roundtrip_for_test(2)
        .await
        .expect("player_pkg.increment_points should generate awards on 3-point boundaries");

    assert_eq!(result.points, 3);
    assert_eq!(result.awards, 1);
    assert_valid_issuers(&result.issuers);
}

#[tokio::test]
async fn f_015_player_pkg_increment_points_case_1() {
    let config =
        AppConfig::load_default().expect("env.toml must exist and contain Oracle connection data");
    let client = OracleClient::new(config.oracle);

    let results = client
        .increment_points_steps_for_test(0, 3)
        .await
        .expect("increment_points case 1 should execute");

    assert_eq!(results.len(), 3);
    assert_eq!(results[0].points, 1);
    assert_eq!(results[0].awards, 0);
    assert_valid_issuers(&results[0].issuers);
    assert_eq!(results[1].points, 2);
    assert_eq!(results[1].awards, 0);
    assert_valid_issuers(&results[1].issuers);
    assert_eq!(results[2].points, 3);
    assert_eq!(results[2].awards, 1);
    assert_valid_issuers(&results[2].issuers);
}

#[tokio::test]
async fn f_015_player_pkg_increment_points_case_2() {
    let config =
        AppConfig::load_default().expect("env.toml must exist and contain Oracle connection data");
    let client = OracleClient::new(config.oracle);

    let results = client
        .increment_points_steps_for_test(0, 10)
        .await
        .expect("increment_points case 2 should execute");

    let final_result = results
        .last()
        .expect("case 2 should return at least one result");

    assert_eq!(final_result.points, 10);
    assert_eq!(final_result.awards, 3);
    assert_valid_issuers(&final_result.issuers);
}

#[tokio::test]
async fn f_015_player_pkg_increment_points_case_3() {
    let config =
        AppConfig::load_default().expect("env.toml must exist and contain Oracle connection data");
    let client = OracleClient::new(config.oracle);

    let results = client
        .increment_points_steps_for_test(24, 3)
        .await
        .expect("increment_points case 3 should execute");

    assert_eq!(results.len(), 3);
    assert_eq!(results[0].points, 25);
    assert_eq!(results[0].awards, 8);
    assert_valid_issuers(&results[0].issuers);
    assert_eq!(results[2].points, 27);
    assert_eq!(results[2].awards, 9);
    assert_valid_issuers(&results[2].issuers);
}

#[tokio::test]
async fn f_015_player_pkg_increment_points_case_4() {
    let config =
        AppConfig::load_default().expect("env.toml must exist and contain Oracle connection data");
    let client = OracleClient::new(config.oracle);

    let error = client
        .increment_points_error_for_id(-1)
        .await
        .expect("increment_points case 4 should capture the Oracle error");

    let message = error.to_string();

    assert!(message.contains("ORA-20003"));
    assert!(message.contains("Player not found for id -1"));
}

#[tokio::test]
async fn f_015_player_pkg_increment_points_case_5() {
    let config =
        AppConfig::load_default().expect("env.toml must exist and contain Oracle connection data");
    let client = OracleClient::new(config.oracle);

    let error = client
        .increment_points_error_for_null_id()
        .await
        .expect("increment_points case 5 should capture the Oracle error");

    let message = error.to_string();

    assert!(message.contains("ORA-20001"));
    assert!(message.contains("p_player_id must not be null"));
}

#[tokio::test]
async fn f_016_player_pkg_decrease_points_case_1() {
    let config =
        AppConfig::load_default().expect("env.toml must exist and contain Oracle connection data");
    let client = OracleClient::new(config.oracle);

    let result = client
        .decrease_points_with_awards_roundtrip_for_test(1, 0)
        .await
        .expect("decrease_points case 1 should execute");

    assert_eq!(result.points, 0);
    assert_eq!(result.awards, 0);
    assert_valid_issuers(&result.issuers);
}

#[tokio::test]
async fn f_016_player_pkg_decrease_points_case_2() {
    let config =
        AppConfig::load_default().expect("env.toml must exist and contain Oracle connection data");
    let client = OracleClient::new(config.oracle);

    let result = client
        .decrease_points_with_awards_roundtrip_for_test(3, 1)
        .await
        .expect("decrease_points case 2 should execute");

    assert_eq!(result.points, 2);
    assert_eq!(result.awards, 0);
    assert_valid_issuers(&result.issuers);
}

#[tokio::test]
async fn f_016_player_pkg_decrease_points_case_3() {
    let config =
        AppConfig::load_default().expect("env.toml must exist and contain Oracle connection data");
    let client = OracleClient::new(config.oracle);

    let result = client
        .decrease_points_with_awards_roundtrip_for_test(24, 8)
        .await
        .expect("decrease_points case 3 should execute");

    assert_eq!(result.points, 23);
    assert_eq!(result.awards, 7);
    assert_valid_issuers(&result.issuers);
}

#[tokio::test]
async fn f_016_player_pkg_decrease_points_case_4() {
    let config =
        AppConfig::load_default().expect("env.toml must exist and contain Oracle connection data");
    let client = OracleClient::new(config.oracle);

    let error = client
        .decrease_points_error_for_id(-1)
        .await
        .expect("decrease_points case 4 should capture the Oracle error");

    let message = error.to_string();

    assert!(message.contains("ORA-20003"));
    assert!(message.contains("Player not found for id -1"));
}

#[tokio::test]
async fn f_016_player_pkg_decrease_points_case_5() {
    let config =
        AppConfig::load_default().expect("env.toml must exist and contain Oracle connection data");
    let client = OracleClient::new(config.oracle);

    let error = client
        .decrease_points_error_for_null_id()
        .await
        .expect("decrease_points case 5 should capture the Oracle error");

    let message = error.to_string();

    assert!(message.contains("ORA-20001"));
    assert!(message.contains("p_player_id must not be null"));
}

#[tokio::test]
async fn f_016_player_pkg_decrease_points_case_6() {
    let config =
        AppConfig::load_default().expect("env.toml must exist and contain Oracle connection data");
    let client = OracleClient::new(config.oracle);

    let error = client
        .decrease_points_error_for_state(0, 0)
        .await
        .expect("decrease_points case 6 should capture the Oracle error");

    let message = error.to_string();

    assert!(message.contains("ORA-20004"));
    assert!(message.contains("Player points cannot be negative"));
}
