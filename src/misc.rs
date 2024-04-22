use std::path::Path;
use std::sync::Arc;
use chrono::{Duration};
use tracing::{error, info};
use crate::AppState;
use crate::handler::STORE_DIR;
use crate::{
    file::File,
};

pub fn get_random_hash(len: i32) -> String {
        use rand::Rng;
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz";
        let hash_length: usize = len as usize;
        let mut rng = rand::thread_rng();

        let hash: String = (0..hash_length)
            .map(|_| {
                    let idx = rng.gen_range(0..CHARSET.len());
                    CHARSET[idx] as char
            })
            .collect();
        return hash;
}

pub fn parse_duration(input: &str) -> anyhow::Result<Duration> {
    let (number_str, specifier) = input.split_at(input.len() - 1);
    let number = number_str.parse::<i64>()?;

    match specifier {
        "h" => Ok(Duration::hours(number)),
        "d" => Ok(Duration::days(number)),
        "m" => Ok(Duration::minutes(number)),
        "s" => Ok(Duration::seconds(number)),
        _ => Err(anyhow::anyhow!("Invalid duration specifier")),
    }
}

pub async fn cleaner(app_state: Arc<AppState>) {

    loop {
        let query_result = sqlx::query_as!(
            File,
            "SELECT * FROM files WHERE expires_at < $1 AND deleted_at IS NULL",
            chrono::Utc::now().naive_utc()
        ).fetch_all(&app_state.db).await;

        if let Ok(files) = query_result {
            for file in files {
                let path = Path::new(&*STORE_DIR).join(file.hash.clone().unwrap());

                match tokio::fs::metadata(&path).await {
                    Ok(_) => {
                        info!("Removing file: {:?}", path);
                        if let Err(err) = tokio::fs::remove_file(&path).await {
                            error!("Error removing file {:?}: {:?}", path, err);
                        }

                        let query_result = sqlx::query!(
                            "UPDATE files SET deleted_at = $1 WHERE hash = $2",
                            chrono::Utc::now().naive_utc(),
                            file.hash.unwrap()
                        ).execute(&app_state.db).await;
                    },
                    Err(_) => {
                        info!("File {:?} does not exist", path);
                    }
                }
            }
        } else if let Err(err) = query_result {
            error!("Error querying database: {:?}", err);
        }

        let a = std::time::Duration::from_secs(5);
        tokio::time::sleep(a).await;
    }
}
