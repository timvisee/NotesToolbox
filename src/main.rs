extern crate clap;
#[cfg(test)]
extern crate tempdir;

use clap::App;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Error;
use std::path::Path;
use std::path::PathBuf;

const APP_NAME: &'static str = "NotesToolbox";
const APP_VERSION_NAME: &'static str = "v0.1.0-alpha";
#[allow(dead_code)]
const APP_VERSION_CODE: i32 = 1;
const APP_AUTHOR: &'static str = "Tim Visee <timvisee@gmail.com>";
const APP_DESCRIPTION: &'static str = "Toolbox project for compiling notes into PDF's, slides and \
        some other formats!";
#[cfg(windows)]
#[allow(dead_code)]
const ENV_PATH_DELIMITER: &'static str = ";";
#[cfg(not(windows))]
#[allow(dead_code)]
const ENV_PATH_DELIMITER: &'static str = ":";

/// Start the application.
fn main() {
    // Handle command line arguments for help
    handle_arguments();

    // Load a file
    let file = load_file_vec(Path::new("./res/testfile.txt"))
        .expect("failed to load file");

    // Print the file length and it's contents (bytes)
    println!("File length: {:?}", file.len());
    println!("File data: {:?}", file);
}

/// Load the given file as vector, where `path` is the path the file is loaded from.
///
/// # Examples
///
/// Load a file, and print it's file lines and raw bytes:
/// ```no_run
/// let file = load_file(Path::new("~/myfile"))
///         .expect("failed to load file");
///
/// println!("File length: {:?}", file.len());
/// println!("File data: {:?}", file);
/// ```
fn load_file_vec(path: &Path) -> Result<Vec<u8>, &'static str> {
    // Try to open the file
    let file_result = File::open(path);

    // Handle errors
    if file_result.is_err() {
        return Err("failed to open file");
    }

    // Get the actual file, and create a new data buffer
    let mut file = file_result.unwrap();
    let mut data = Vec::new();

    // Read the actual file
    if file.read_to_end(&mut data).is_err() {
        return Err("failed to read file");
    }

    // Return the data vector containing the loaded file
    Ok(data)
}

/// Handle program arguments passed along with the command line to show things like help pages.
///
/// # Examples
///
/// Parse application parameters, and show relevant information such as help files in the console:
/// ```no_run
/// handle_arguments();
/// ```
fn handle_arguments() {
    // Configure the application object with help information, and show matches
    App::new(APP_NAME)
        .version(APP_VERSION_NAME)
        .author(APP_AUTHOR)
        .about(APP_DESCRIPTION)
        .get_matches();
}

/// Get the PATH variable from the OS environment variables.
///
/// # Examples
///
/// Fetch and print PATH variable to console:
/// ```no_run
/// println!("PATH: {:?}", get_env_path().unwrap());
/// ```
#[allow(dead_code)]
fn get_env_path() -> Option<String> {
    // Get and return the PATH variable
    env::var("PATH").ok()
}

/// Find the paths programs may be installed in on this system.
///
/// # Examples
///
/// Get all possible program paths, output them to the console:
/// ```
/// println!("Program paths:");
/// for path in get_program_paths(None) {
///     println!(" - {:?}", path);
/// }
/// ```
#[allow(dead_code)]
fn get_program_paths(dir: Option<String>) -> Vec<String> {
    // Create a vector to put the paths in
    let mut paths: Vec<String> = Vec::new();

    // Get the directory string
    let dir_str;
    if dir.is_some() {
        dir_str = dir.unwrap();
    } else {
        dir_str = String::from("");
    }

    // Get the application paths for Windows
    if cfg!(target_os = "windows") {
        // Determine whether we should test the default path
        let mut test_default = true;

        // Create an array of possible program files environment variables for 64-bit systems
        let env_vars = ["ProgramW6432", "ProgramFiles(x86)"];

        // Loop through the variables
        for var_name in env_vars.iter() {
            let prog_path = env::var(var_name);
            if prog_path.is_ok() {
                // Unwrap the path, add it to the vector
                paths.push(
                    Path::new(&prog_path.unwrap())
                        .join(&dir_str)
                        .to_str().unwrap().to_string()
                );

                // Set the test_default flag to false, because we already found a path
                test_default = false;
            }
        }

        // Test the default path, as no path was found yet
        if test_default {
            // Get the default program files path
            let prog_default = env::var("ProgramFiles");
            if prog_default.is_ok() {
                paths.push(
                    Path::new(&prog_default.unwrap())
                        .join(&dir_str)
                        .to_str().unwrap().to_string()
                );
            }
        }
    }

    // Get all paths from the PATH environment variable (cross-platform)
    match get_env_path() {
        Some(env_paths) => {
            for path in env::split_paths(&env_paths) {
                paths.push(path.to_str().unwrap().to_string());
            }
        },
        _ => {}
    }

    // Return the vector of paths
    paths
}

/// Force remove a directory structure (including all of it's contents).
/// The regular `fs::remove_dir_all(...)` function might inconsistently fail on Windows machines.
/// This function forces to delete the directory, by repeatedly invoking the remove command until
/// the file is successfully removed. The remove command is attempted a maximum number of times to
/// prevent locking.
///
/// True is returned if the directory was successfully removed. False is returned if the directory
/// structure couldn't be removed, and the maximum number of remove attempts is reached.
///
/// # Example
///
/// Force remove a directory structure on Windows:
/// ```no_run
/// remove_dir_all_force(Path::new("~/myfile");
/// ```
pub fn remove_dir_all_force(path: PathBuf) -> bool {
    // Create a variable to define the number of attempts we've left
    // TODO: Don't hardcode this value in the function itself, create a constant?
    let mut attempt: u8 = 8;

    // Keep trying to remove the directory until attempts are drained
    while attempt > 0 {
        // Try to remove the directory, and return on success
        if fs::remove_dir_all(&path).is_ok() {
            return true;
        }

        // Decrease the number of attempts
        attempt -= 1;
    }

    // We failed
    false
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::File;
    use std::io::Write;

    use tempdir::TempDir;

    use super::get_env_path;
    use super::get_program_paths;
    use super::load_file_vec;
    use super::remove_dir_all_force;

    #[test]
    fn get_env_path_test() {
        // Should always get the environment path variable
        assert!(get_env_path().is_some());
    }

    #[test]
    fn get_program_paths_test() {
        // Should always get at least one path
        assert!(get_program_paths(None).len() > 0);
        assert!(get_program_paths(Some("ProgramName".to_string())).len() > 0);
    }

    #[test]
    fn load_file_vec_test() {
        // Create a new vector
        let base_vec = vec![60, 61, 62];

        // Create a temporary directory for testing
        let temp_dir = TempDir::new("notes_toolbox")
                .expect("failed to create temporary notes_toolbox directory");

        // Determine the path for the test file
        let file_path = temp_dir.path().join("test_file.txt");
        println!("Path: {:?}", file_path);

        // Create and write the file
        let mut file = File::create(&file_path).expect("failed to create temporary file");
        // TODO: Write the `base_vec` values instead of the current string
        file.write_all(b"Hello, world!").unwrap();
        file.sync_all().unwrap();

        // Load the vector
        let mut file_vec = load_file_vec(&file_path).unwrap();
        println!("Test vec: {:?}", file_vec.len());

        // TODO: Test whether the file contents are correct

        // Remove the temporarily created file
        fs::remove_file(&file_path).unwrap();

        // Remove the temporary directory
        remove_dir_all_force(temp_dir.into_path());

        // TODO: Remove debug messages
    }
}