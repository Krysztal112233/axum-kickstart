use std::time::Duration;

use axum::Router;
use common::error::Error;
use log::error;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tap::Pipe;
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::{Scalar, Servable};

use crate::{config::DatabaseConfig, state::AppState};

mod config;
mod endpoints;
mod state;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let config = dbg!(config::BackendConfig::new().inspect_err(|e| error!("{e}"))?);

    let states = {
        let database = setup_database(&config.database)
            .await
            .inspect_err(|e| error!("{e}"))?;

        AppState {
            database,
            _unit: (),
        }
    };

    let (router, openapi) = OpenApiRouter::new().split_for_parts();

    let router: Router = router
        .merge(Scalar::with_url("/docs", openapi))
        .with_state(states);

    let listener = tokio::net::TcpListener::bind(config.addr)
        .await
        .inspect_err(|e| error!("{e}"))?;

    axum::serve(listener, router).await?;

    Ok(())
}

async fn setup_database(config: &DatabaseConfig) -> Result<DatabaseConnection, Error> {
    let DatabaseConfig {
        dsn,
        slow_statements_logging_threshold,
        max_connections,
        min_connections,
        migrate,
    } = config;

    let options = ConnectOptions::new(dsn)
        .pipe_borrow_mut(|it| match slow_statements_logging_threshold {
            Some(milis) => it.sqlx_slow_statements_logging_settings(
                log::LevelFilter::Warn,
                Duration::from_micros(*milis),
            ),
            _ => it,
        })
        .pipe_borrow_mut(|it| match max_connections {
            Some(c) => it.max_connections(*c),
            _ => it,
        })
        .pipe_borrow_mut(|it| match min_connections {
            Some(c) => it.min_connections(*c),
            _ => it,
        })
        .to_owned();

    let db = Database::connect(options).await?;

    if *migrate {
        Migrator::up(&db, None).await?;
    }

    Ok(db)
}
