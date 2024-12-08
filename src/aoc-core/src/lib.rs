pub mod char_grid;
pub mod dijkstra;
pub mod directed_graph;
pub mod int_grid;
pub mod primitives;
pub mod text_map;
pub mod undirected_graph;

use std::{io, time::Duration};

use thiserror::Error;

/// Errors that can happen for the input for the puzzle.
#[derive(Error, Debug)]
pub enum InputError {
    /// There was an I/O error while opening the input file.
    #[error("There was an I/O error while opening the input file '{1}'.")]
    OpenInputFile(#[source] io::Error, String),

    /// The environment variable `AOC_SESSION_COOKIE` is not set.
    /// This is needed to download the input for the puzzle.
    /// You can set it by running `export AOC_SESSION_COOKIE=your_cookie`.
    /// You can find your session cookie by logging into adventofcode.com and looking at the cookie in your browser.
    #[error("The environment variable 'AOC_SESSION_COOKIE' is not set. This is needed to download the input for the puzzle. You can set it by running 'export AOC_SESSION_COOKIE=your_cookie'. You can find your session cookie by logging into adventofcode.com and looking at the cookie in your browser.")]
    AocSessionCookieEnvVarNotSet,

    /// There was an error while downloading the input for the puzzle.
    #[error(
        "There was an error while downloading the input for the puzzle of day {1} for year {2}."
    )]
    DownloadInput(#[source] ureq::Error, u32, u32),

    /// There was an I/O error while reading the input body.
    #[error("There was an I/O error while reading the input body.")]
    ReadInputBody(#[source] io::Error),

    /// There was an I/O error while creating the data directory.
    #[error("There was an I/O error while creating the data directory '{1}'.")]
    CreateDataDir(#[source] io::Error, String),
}

const INPUT_FILENAME: &str = r"input.txt";
const AOC_DATA_SUBDIR: &str = r"aoc";

/// Is getting the input for the puzzle of `year` and `day`.
/// If the input file already exists, it reads the input from the file.
/// Otherwise, it downloads the input from adventofcode.com and saves it to the file.
/// The input file is saved in the data directory of the user.
///
/// # Arguments
///
/// - `year` - The year of the puzzle.
/// - `day` - The day of the puzzle.
pub fn get_input(year: u32, day: u32) -> Result<String, InputError> {
    let data_dir = provision_data_dir(year, day)?;
    let input_file = data_dir.join(INPUT_FILENAME);

    if input_file.exists() {
        println!("Reading input from file: {:?}", input_file);

        Ok(std::fs::read_to_string(&input_file)
            .map_err(|e| InputError::OpenInputFile(e, format!("{:?}", input_file)))?)
    } else {
        println!("Downloading input for year {} and day {}...", year, day);

        let input = download_input(year, day)?;

        println!("Saving input to file: {:?}", input_file);
        
        std::fs::write(&input_file, &input)
            .map_err(|e| InputError::OpenInputFile(e, format!("{:?}", input_file)))?;

        Ok(input)
    }
}

fn provision_data_dir(year: u32, day: u32) -> Result<std::path::PathBuf, InputError> {
    let mut data_dir = dirs::data_dir().expect("Data directory not found");

    data_dir.push(AOC_DATA_SUBDIR);
    data_dir.push(year.to_string());
    data_dir.push(day.to_string());

    if !data_dir.exists() {
        std::fs::create_dir_all(&data_dir)
            .map_err(|e| InputError::CreateDataDir(e, format!("{:?}", data_dir)))?;
    }

    Ok(data_dir)
}

fn download_input(year: u32, day: u32) -> Result<String, InputError> {
    let aoc_input_url = aoc_input_url(year, day);
    let aoc_session_cookie = aoc_session_cookie()?;

    let agent = ureq::AgentBuilder::new()
        .timeout_read(Duration::from_secs(5))
        .timeout_write(Duration::from_secs(5))
        .build();

    let input_body = agent
        .get(&aoc_input_url)
        .set("Cookie", &aoc_session_cookie)
        .call()
        .map_err(|e| InputError::DownloadInput(e, year, day))?
        .into_string()
        .map_err(InputError::ReadInputBody)?;

    Ok(input_body)
}

fn aoc_input_url(year: u32, day: u32) -> String {
    format!("https://adventofcode.com/{}/day/{}/input", year, day)
}

fn aoc_session_cookie() -> Result<String, InputError> {
    match std::env::var("AOC_SESSION_COOKIE") {
        Ok(cookie) => Ok(format!("session={}", cookie)),
        Err(_) => {
            let mut aoc_session_cookie_filepath = dirs::data_dir().expect("Data directory not found");
            aoc_session_cookie_filepath.push(AOC_DATA_SUBDIR);
            aoc_session_cookie_filepath.push("session.cookie");

            match std::fs::read_to_string(&aoc_session_cookie_filepath) {
                Ok(cookie) => Ok(format!("session={}", cookie.trim())),
                Err(_) => Err(InputError::AocSessionCookieEnvVarNotSet),
            }
        }
    }
}
