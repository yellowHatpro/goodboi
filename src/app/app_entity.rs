use colorize::*;
use std::env;
use tokio::fs;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub struct AppEntity {
    pub dir: String,
}

impl AppEntity {
    pub async fn init() -> AppEntity {
        let home_dir = env::var("HOME").unwrap();
        let i_remember_dir = format!("{}/.i-remember", home_dir);
        let data_file = format!("{}/data.json", i_remember_dir);

        // Create .i-remember directory if it doesn't exist
        if !fs::metadata(&i_remember_dir).await.is_ok() {
            fs::create_dir(&i_remember_dir).await.unwrap();
            println!("{} {}", "Created directory".green(), i_remember_dir);
        }

        // Create data.json if it doesn't exist
        if !fs::metadata(&data_file).await.is_ok() {
            let mut file = File::create(&data_file).await.unwrap();
            file.write_all(b"{\"data\":[]}").await.unwrap();
            println!("{} {}", "Created file".green(), data_file);
        }

        AppEntity { dir: home_dir }
    }
}
