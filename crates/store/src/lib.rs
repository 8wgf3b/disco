use std::{fs::create_dir_all, path::Path, str::FromStr};

use disco_core::AppError;
use sqlx::{
    SqlitePool,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};
use tracing::{debug, error, info};

pub async fn get_pools(cntstr: &str) -> Result<SqlitePool, sqlx::Error> {
    info!("Creating db conn pools from {cntstr:?}");
    let path_str = cntstr
        .trim_start_matches("sqlite://")
        .trim_end_matches("sqlite:");
    if let Some(pardir) = Path::new(path_str).parent() {
        debug!("Idempotent db parent {:?} folder creation", pardir);
        let _ = create_dir_all(pardir).inspect_err(|e| error!("DB folder creation error: {e:#}"));
    }
    let conopts = SqliteConnectOptions::from_str(cntstr)?.create_if_missing(true);
    SqlitePoolOptions::new()
        .max_connections(6)
        .connect_with(conopts)
        .await
}
