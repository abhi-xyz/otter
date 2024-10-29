use dirs::{config_dir, download_dir};
use log::trace;
use serde::Deserialize;
use std::{
    env::current_dir,
    error::Error,
    fs::{self, create_dir_all},
    path::{Path, PathBuf},
};

// Configuration structure to deserialize from TOML.
#[derive(Deserialize, Debug)]
pub struct Config {
    pub directories: Directories,
}

/// Define directories for each file type in the configuration.
#[derive(Deserialize, Debug)]
pub struct Directories {
    pub images: String,
    pub documents: String,
    pub videos: String,
}

fn get_config_file() -> PathBuf {
    let xdg_path = config_dir().unwrap().join("otter/config.toml");
    if !config_dir().unwrap().join("otter").exists() {
        trace!("otter dir doesn't exist");
        fs::create_dir(config_dir().unwrap().join("otter")).unwrap();
    }
    if !xdg_path.is_file() {
        let content = toml::toml! {
            [directories]
            images = "/home/abhi/pics/"
            documents = "/home/abhi/docs/"
            videos = "/home/abhi/videos/"
        };
        let c = content.to_string();
        // fs::write(config_dir().unwrap().join("otter/config.toml"),"heelo").unwrap();
        fs::write(config_dir().unwrap().join("otter/config.toml"), c).unwrap();
        trace!("writing default config to {:?}", xdg_path);
    }
    let dev_path = current_dir().unwrap().join("config.toml");
    if dev_path.exists() {
        let config_file = dev_path;
        trace!("using config file from dev path {:?}", &config_file);
        return config_file;
    } else {
        let config_file = xdg_path;
        trace!("using config file from xdg path {:?}", &config_file);
        return config_file;
    }
}

pub fn parse_toml() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(get_config_file())?;
    let config: Config = toml::from_str(&content)?;
    trace!("{:#?}", config);
    // temp main fn
    let input = download_dir().unwrap();
    organize_files(input, config)?;
    Ok(())
}

fn organize_files(input_dirs: PathBuf, config: Config) -> Result<(), Box<dyn Error>> {
    println!("I am going to clean {:?} directory", &input_dirs);
    for i in fs::read_dir(input_dirs)? {
        let i = i?;
        let path = i.path();

        // Only proceed if it's a file
        if path.is_file() {
            match path.extension().and_then(|ext| ext.to_str()) {
                Some("jpg" | "png" | "gif") => move_file(&path, &config.directories.images)?,
                Some("pdf" | "docx" | "txt") => move_file(&path, &config.directories.documents)?,
                Some("mp4" | "mov" | "avi") => move_file(&path, &config.directories.videos)?,
                _ => println!("Unsupported file type: {:?}", path),
            }
        }
    }
    Ok(())
}

fn move_file(file: &Path, destination_dir: &str) -> Result<(), Box<dyn Error>> {
    let destination_path = Path::new(destination_dir);
    create_dir_all(destination_path)?;

    let file_name = file
        .file_name()
        .ok_or("Wooh Wooh. What a file name is that!")?;
    let dest_path = destination_path.join(file_name);

    fs::rename(file, dest_path)?;
    println!("Moved file: {:?}", file);
    Ok(())
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
