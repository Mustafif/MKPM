pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

use super::{data, AppType};
use std::io::{self, Cursor, Read};

pub async fn get(id: String) -> Result<()> {
    let d = data::Data::load_data().expect("Couldn't load data");
    for i in d {
        let mut path_suffix = String::new();
        match i.app_type {
            AppType::Book => {
                path_suffix = ".zip".to_owned();
            }
            AppType::Application => {
                if cfg!(windows) {
                    path_suffix = ".exe".to_owned();
                }
                if cfg!(linux) {
                    path_suffix = ".deb".to_owned();
                }
            }
            AppType::Unknown => {
                panic!()
            }
        };

        if id == i.id {
            let response = reqwest::get(i.url).await?;
            let mut file = std::fs::File::create(format!("{}{}", i.id, path_suffix))?;
            let mut content = Cursor::new(response.bytes().await?);
            io::copy(&mut content, &mut file)?;
        }
    }
    Ok(())
}
