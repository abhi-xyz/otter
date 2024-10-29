use dirs::config_dir;
use log::trace;
use serde::Deserialize;
use std::{
    error::Error,
    fs::{self, create_dir_all},
    path::{Path, PathBuf},
};

// Configuration structure to deserialize from TOML.
#[derive(Deserialize, Debug, Clone)] // Deriving Clone for Config
pub struct Config {
    pub directories: Directories,
    pub input_dirs: InputDirs, // New field to hold input directories
}

/// Define directories with path and tree option for each file type.
#[derive(Deserialize, Debug, Clone)] // Deriving Clone for Directories
pub struct Directories {
    pub images: DirectoryConfig,
    pub documents: DirectoryConfig,
    pub videos: DirectoryConfig,
    pub archives: DirectoryConfig,
}

/// Struct for input directories
#[derive(Deserialize, Debug, Clone)] // Deriving Clone for InputDirs
pub struct InputDirs {
    pub dirs: Vec<String>, // Change to hold a list of directory paths
}

/// Struct for each directory's configuration, specifying the path and tree behavior.
#[derive(Deserialize, Debug, Clone)] // Deriving Clone for DirectoryConfig
pub struct DirectoryConfig {
    pub path: String,
    pub tree: bool, // If true, create subdirectories based on file extensions.
}

// Function to determine or create the config file path.
fn get_config_file() -> PathBuf {
    let xdg_path = config_dir().unwrap().join("otter/config.toml");

    // Create the 'otter' directory if it doesn't exist
    if !config_dir().unwrap().join("otter").exists() {
        trace!("otter dir doesn't exist");
        fs::create_dir(config_dir().unwrap().join("otter")).unwrap();
    }

    // Write default config if no config file exists
    if !xdg_path.is_file() {
        let content = toml::toml! {
            [directories]
            images = { path = "/home/abhi/pics/pictures/images", tree = false }
            documents = { path = "/home/abhi/docs/lib", tree = false }
            videos = { path = "/home/abhi/videos", tree = true }
            archives = { path = "/home/abhi/archive", tree = false }

            [input_dirs]
            dirs = ["/home/abhi/downloads", "/home/abhi/videos"]  // List of input directories
        };
        let c = content.to_string();
        fs::write(&xdg_path, c).unwrap();
        trace!("writing default config to {:?}", xdg_path);
    }

    // If a local config file is found, use it for development; otherwise, use XDG path.
    let dev_path = std::env::current_dir().unwrap().join("config.toml");
    if dev_path.exists() {
        trace!("using config file from dev path {:?}", &dev_path);
        return dev_path;
    } else {
        trace!("using config file from xdg path {:?}", &xdg_path);
        return xdg_path;
    }
}

// Function to parse TOML configuration and start file organization.
pub fn parse_toml() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(get_config_file())?;
    let config: Config = toml::from_str(&content)?;
    trace!("{:#?}", config);

    // Initialize organization with each input directory specified in the configuration.
    for input in &config.input_dirs.dirs {
        // Accessing dirs within input_dirs
        let input_path = PathBuf::from(input);
        println!("{:?}", &input);
        organize_files(input_path, config.clone())?; // Pass the cloned config to maintain state for each input directory
    }
    Ok(())
}

// Function to organize files according to their types and configuration paths.
fn organize_files(input_dirs: PathBuf, config: Config) -> Result<(), Box<dyn Error>> {
    println!("I am going to clean {:?} directory", &input_dirs);

    for entry in fs::read_dir(input_dirs)? {
        let entry = entry?;
        let path = entry.path();

        // Only proceed if it's a file
        if path.is_file() {
            match path.extension().and_then(|ext| ext.to_str()) {
                Some("jpg" | "png" | "gif") => move_file(&path, &config.directories.images)?,
                Some("pdf" | "docx" | "txt") => move_file(&path, &config.directories.documents)?,
                Some("mp4" | "mov" | "avi") => move_file(&path, &config.directories.videos)?,
                Some("zip" | "xz" | "7z") => move_file(&path, &config.directories.archives)?,
                _ => println!("Unsupported file type: {:?}", path),
            }
        }
    }
    Ok(())
}

// Function to move a file to the specified destination with optional subdirectories for extensions.
fn move_file(file: &Path, dir_config: &DirectoryConfig) -> Result<(), Box<dyn Error>> {
    // Base directory specified in the configuration.
    let mut destination_path = Path::new(&dir_config.path).to_path_buf();

    // If 'tree' is true, create a subdirectory based on the file extension.
    if dir_config.tree {
        if let Some(ext) = file.extension().and_then(|e| e.to_str()) {
            destination_path = destination_path.join(ext);
        }
    }

    // Ensure the directory exists before moving the file.
    create_dir_all(&destination_path)?;

    // Create the full path for the new file location.
    let file_name = file.file_name().ok_or("Invalid file name!")?;
    let dest_path = destination_path.join(file_name);

    // Move the file and output its path.
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
