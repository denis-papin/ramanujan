use anyhow::Context;
use oracle::Connection;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::config::OracleConfig;

#[derive(Clone)]
pub struct OracleClient {
    config: OracleConfig,
}

pub struct IncrementGoalsTestResult {
    pub points: i64,
    pub awards: i64,
    pub issuers: Vec<String>,
}

#[derive(Debug)]
pub struct DecreaseGoalsTestResult {
    pub points: i64,
    pub awards: i64,
    pub issuers: Vec<String>,
}

impl OracleClient {
    pub fn new(config: OracleConfig) -> Self {
        Self { config }
    }

    pub async fn select_one_from_dual(&self) -> anyhow::Result<i64> {
        let config = self.config.clone();

        tokio::task::spawn_blocking(move || {
            let conn =
                Connection::connect(&config.username, &config.password, config.connect_string())
                    .context("failed to connect to Oracle")?;

            let value: i64 = conn
                .query_row_as("select 1 from dual", &[])
                .context("failed to execute 'select 1 from dual'")?;

            Ok(value)
        })
        .await
        .context("Oracle query task panicked or was cancelled")?
    }

    pub async fn increment_points_roundtrip_for_test(&self) -> anyhow::Result<i64> {
        Ok(self
            .increment_points_with_awards_roundtrip_for_test(0)
            .await?
            .points)
    }

    pub async fn increment_points_with_awards_roundtrip_for_test(
        &self,
        initial_points: i64,
    ) -> anyhow::Result<IncrementGoalsTestResult> {
        Ok(self
            .increment_points_steps_for_test(initial_points, 1)
            .await?
            .into_iter()
            .next()
            .context("expected one increment_points result")?)
    }

    pub async fn increment_points_steps_for_test(
        &self,
        initial_points: i64,
        steps: usize,
    ) -> anyhow::Result<Vec<IncrementGoalsTestResult>> {
        let config = self.config.clone();

        tokio::task::spawn_blocking(move || {
            let conn =
                Connection::connect(&config.username, &config.password, config.connect_string())
                    .context("failed to connect to Oracle")?;

            let unique_suffix = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .context("system clock is before UNIX_EPOCH")?
                .as_nanos();
            let player_name = format!("rust-tu-{unique_suffix}");

            conn.execute(
                "insert into player (name, points) values (:1, :2)",
                &[&player_name, &initial_points],
            )
            .context("failed to insert temporary player for increment_points test")?;

            let player_id: i64 = conn
                .query_row_as(
                    "select id from player where name = :1 fetch first 1 row only",
                    &[&player_name],
                )
                .context("failed to fetch temporary player id")?;

            let mut results = Vec::with_capacity(steps);

            for _ in 0..steps {
                conn.execute("begin player_pkg.increment_points(:1); end;", &[&player_id])
                    .context("failed to execute player_pkg.increment_points")?;

                let points: i64 = conn
                    .query_row_as("select points from player where id = :1", &[&player_id])
                    .context("failed to read updated points")?;
                let awards: i64 = conn
                    .query_row_as(
                        "select count(*) from award where player_id = :1",
                        &[&player_id],
                    )
                    .context("failed to count generated awards")?;
                let mut issuers = Vec::new();
                let rows = conn
                    .query(
                        "select issuer from award where player_id = :1 order by id",
                        &[&player_id],
                    )
                    .context("failed to read generated award issuers")?;
                for row_result in rows {
                    let row = row_result.context("failed to read generated award issuer row")?;
                    issuers.push(
                        row.get::<usize, String>(0)
                            .context("failed to decode generated award issuer")?,
                    );
                }

                results.push(IncrementGoalsTestResult {
                    points,
                    awards,
                    issuers,
                });
            }

            conn.rollback()
                .context("failed to rollback increment_points test transaction")?;

            Ok(results)
        })
        .await
        .context("Oracle query task panicked or was cancelled")?
    }

    pub async fn increment_points_error_for_id(
        &self,
        player_id: i64,
    ) -> anyhow::Result<anyhow::Error> {
        let config = self.config.clone();

        tokio::task::spawn_blocking(move || {
            let conn =
                Connection::connect(&config.username, &config.password, config.connect_string())
                    .context("failed to connect to Oracle")?;

            match conn.execute("begin player_pkg.increment_points(:1); end;", &[&player_id]) {
                Ok(_) => anyhow::bail!("expected player_pkg.increment_points to fail"),
                Err(error) => Ok(anyhow::Error::new(error)),
            }
        })
        .await
        .context("Oracle query task panicked or was cancelled")?
    }

    pub async fn increment_points_error_for_null_id(&self) -> anyhow::Result<anyhow::Error> {
        let config = self.config.clone();

        tokio::task::spawn_blocking(move || {
            let conn =
                Connection::connect(&config.username, &config.password, config.connect_string())
                    .context("failed to connect to Oracle")?;

            match conn.execute(
                "begin player_pkg.increment_points(:1); end;",
                &[&Option::<i64>::None],
            ) {
                Ok(_) => anyhow::bail!("expected player_pkg.increment_points to fail"),
                Err(error) => Ok(anyhow::Error::new(error)),
            }
        })
        .await
        .context("Oracle query task panicked or was cancelled")?
    }

    pub async fn decrease_points_with_awards_roundtrip_for_test(
        &self,
        initial_points: i64,
        initial_awards: i64,
    ) -> anyhow::Result<DecreaseGoalsTestResult> {
        Ok(self
            .decrease_points_steps_for_test(initial_points, initial_awards, 1)
            .await?
            .into_iter()
            .next()
            .context("expected one decrease_points result")?)
    }

    pub async fn decrease_points_steps_for_test(
        &self,
        initial_points: i64,
        initial_awards: i64,
        steps: usize,
    ) -> anyhow::Result<Vec<DecreaseGoalsTestResult>> {
        let config = self.config.clone();

        tokio::task::spawn_blocking(move || {
            let conn =
                Connection::connect(&config.username, &config.password, config.connect_string())
                    .context("failed to connect to Oracle")?;

            let unique_suffix = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .context("system clock is before UNIX_EPOCH")?
                .as_nanos();
            let player_name = format!("rust-tu-decrease-{unique_suffix}");

            conn.execute(
                "insert into player (name, points) values (:1, :2)",
                &[&player_name, &initial_points],
            )
            .context("failed to insert temporary player for decrease_points test")?;

            let player_id: i64 = conn
                .query_row_as(
                    "select id from player where name = :1 fetch first 1 row only",
                    &[&player_name],
                )
                .context("failed to fetch temporary player id")?;

            for award_index in 0..initial_awards {
                let issuer = if award_index % 2 == 0 {
                    "GOLD"
                } else {
                    "SILVER"
                };
                let award_year = 1980_i64 + (award_index % 51) as i64;

                conn.execute(
                    "insert into award (player_id, issuer, award_year) values (:1, :2, :3)",
                    &[&player_id, &issuer, &award_year],
                )
                .context("failed to insert initial award for decrease_points test")?;
            }

            let mut results = Vec::with_capacity(steps);

            for _ in 0..steps {
                conn.execute("begin player_pkg.decrease_points(:1); end;", &[&player_id])
                    .context("failed to execute player_pkg.decrease_points")?;

                let points: i64 = conn
                    .query_row_as("select points from player where id = :1", &[&player_id])
                    .context("failed to read updated points after decrease")?;
                let awards: i64 = conn
                    .query_row_as(
                        "select count(*) from award where player_id = :1",
                        &[&player_id],
                    )
                    .context("failed to count awards after decrease")?;
                let mut issuers = Vec::new();
                let rows = conn
                    .query(
                        "select issuer from award where player_id = :1 order by id",
                        &[&player_id],
                    )
                    .context("failed to read award issuers after decrease")?;
                for row_result in rows {
                    let row = row_result.context("failed to read award issuer row after decrease")?;
                    issuers.push(
                        row.get::<usize, String>(0)
                            .context("failed to decode award issuer after decrease")?,
                    );
                }

                results.push(DecreaseGoalsTestResult {
                    points,
                    awards,
                    issuers,
                });
            }

            conn.rollback()
                .context("failed to rollback decrease_points test transaction")?;

            Ok(results)
        })
        .await
        .context("Oracle query task panicked or was cancelled")?
    }

    pub async fn decrease_points_error_for_id(
        &self,
        player_id: i64,
    ) -> anyhow::Result<anyhow::Error> {
        let config = self.config.clone();

        tokio::task::spawn_blocking(move || {
            let conn =
                Connection::connect(&config.username, &config.password, config.connect_string())
                    .context("failed to connect to Oracle")?;

            match conn.execute("begin player_pkg.decrease_points(:1); end;", &[&player_id]) {
                Ok(_) => anyhow::bail!("expected player_pkg.decrease_points to fail"),
                Err(error) => Ok(anyhow::Error::new(error)),
            }
        })
        .await
        .context("Oracle query task panicked or was cancelled")?
    }

    pub async fn decrease_points_error_for_null_id(&self) -> anyhow::Result<anyhow::Error> {
        let config = self.config.clone();

        tokio::task::spawn_blocking(move || {
            let conn =
                Connection::connect(&config.username, &config.password, config.connect_string())
                    .context("failed to connect to Oracle")?;

            match conn.execute(
                "begin player_pkg.decrease_points(:1); end;",
                &[&Option::<i64>::None],
            ) {
                Ok(_) => anyhow::bail!("expected player_pkg.decrease_points to fail"),
                Err(error) => Ok(anyhow::Error::new(error)),
            }
        })
        .await
        .context("Oracle query task panicked or was cancelled")?
    }

    pub async fn decrease_points_error_for_state(
        &self,
        initial_points: i64,
        initial_awards: i64,
    ) -> anyhow::Result<anyhow::Error> {
        let config = self.config.clone();

        tokio::task::spawn_blocking(move || {
            let conn =
                Connection::connect(&config.username, &config.password, config.connect_string())
                    .context("failed to connect to Oracle")?;

            let unique_suffix = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .context("system clock is before UNIX_EPOCH")?
                .as_nanos();
            let player_name = format!("rust-tu-decrease-error-{unique_suffix}");

            conn.execute(
                "insert into player (name, points) values (:1, :2)",
                &[&player_name, &initial_points],
            )
            .context("failed to insert temporary player for decrease_points error test")?;

            let player_id: i64 = conn
                .query_row_as(
                    "select id from player where name = :1 fetch first 1 row only",
                    &[&player_name],
                )
                .context("failed to fetch temporary player id for decrease_points error test")?;

            for award_index in 0..initial_awards {
                let issuer = if award_index % 2 == 0 {
                    "GOLD"
                } else {
                    "SILVER"
                };
                let award_year = 1980_i64 + (award_index % 51) as i64;

                conn.execute(
                    "insert into award (player_id, issuer, award_year) values (:1, :2, :3)",
                    &[&player_id, &issuer, &award_year],
                )
                .context("failed to insert initial award for decrease_points error test")?;
            }

            match conn.execute("begin player_pkg.decrease_points(:1); end;", &[&player_id]) {
                Ok(_) => anyhow::bail!("expected player_pkg.decrease_points to fail"),
                Err(error) => Ok(anyhow::Error::new(error)),
            }
        })
        .await
        .context("Oracle query task panicked or was cancelled")?
    }
}
