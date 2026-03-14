use anyhow::Context;
use oracle::Connection;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::config::OracleConfig;

#[derive(Clone)]
pub struct OracleClient {
    config: OracleConfig,
}

pub struct IncrementGoalsTestResult {
    pub goals: i64,
    pub awards: i64,
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

    pub async fn increment_goals_roundtrip_for_test(&self) -> anyhow::Result<i64> {
        Ok(self
            .increment_goals_with_awards_roundtrip_for_test(0)
            .await?
            .goals)
    }

    pub async fn increment_goals_with_awards_roundtrip_for_test(
        &self,
        initial_goals: i64,
    ) -> anyhow::Result<IncrementGoalsTestResult> {
        Ok(self
            .increment_goals_steps_for_test(initial_goals, 1)
            .await?
            .into_iter()
            .next()
            .context("expected one increment_goals result")?)
    }

    pub async fn increment_goals_steps_for_test(
        &self,
        initial_goals: i64,
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
                "insert into player (name, goals) values (:1, :2)",
                &[&player_name, &initial_goals],
            )
            .context("failed to insert temporary player for increment_goals test")?;

            let player_id: i64 = conn
                .query_row_as(
                    "select id from player where name = :1 fetch first 1 row only",
                    &[&player_name],
                )
                .context("failed to fetch temporary player id")?;

            let mut results = Vec::with_capacity(steps);

            for _ in 0..steps {
                conn.execute("begin player_pkg.increment_goals(:1); end;", &[&player_id])
                    .context("failed to execute player_pkg.increment_goals")?;

                let goals: i64 = conn
                    .query_row_as("select goals from player where id = :1", &[&player_id])
                    .context("failed to read updated goals")?;
                let awards: i64 = conn
                    .query_row_as(
                        "select count(*) from award where player_id = :1",
                        &[&player_id],
                    )
                    .context("failed to count generated awards")?;

                results.push(IncrementGoalsTestResult { goals, awards });
            }

            conn.rollback()
                .context("failed to rollback increment_goals test transaction")?;

            Ok(results)
        })
        .await
        .context("Oracle query task panicked or was cancelled")?
    }

    pub async fn increment_goals_error_for_id(
        &self,
        player_id: i64,
    ) -> anyhow::Result<anyhow::Error> {
        let config = self.config.clone();

        tokio::task::spawn_blocking(move || {
            let conn =
                Connection::connect(&config.username, &config.password, config.connect_string())
                    .context("failed to connect to Oracle")?;

            match conn.execute("begin player_pkg.increment_goals(:1); end;", &[&player_id]) {
                Ok(_) => anyhow::bail!("expected player_pkg.increment_goals to fail"),
                Err(error) => Ok(anyhow::Error::new(error)),
            }
        })
        .await
        .context("Oracle query task panicked or was cancelled")?
    }

    pub async fn increment_goals_error_for_null_id(&self) -> anyhow::Result<anyhow::Error> {
        let config = self.config.clone();

        tokio::task::spawn_blocking(move || {
            let conn =
                Connection::connect(&config.username, &config.password, config.connect_string())
                    .context("failed to connect to Oracle")?;

            match conn.execute(
                "begin player_pkg.increment_goals(:1); end;",
                &[&Option::<i64>::None],
            ) {
                Ok(_) => anyhow::bail!("expected player_pkg.increment_goals to fail"),
                Err(error) => Ok(anyhow::Error::new(error)),
            }
        })
        .await
        .context("Oracle query task panicked or was cancelled")?
    }
}
