use snafu::prelude::*;

use std::env::current_dir;
use std::os::unix::process::CommandExt;
use std::process::{Command, Stdio};
use std::result::Result;

const ZELLIJ_COMMAND: &str = "zellij";

#[derive(Debug, Snafu)]
pub enum ZxrError {
    #[snafu(display("Unable to load current_dir {:#?}", error))]
    LoadDirectoryError { error: std::io::Error },
    #[snafu(display("Unable to execute zellij command {:#?}", error))]
    ZellijExecutingZellijError { error: std::io::Error },
    #[snafu(display("Unable to get sessions  {:#?}", error))]
    ZellijSessionLoadError { error: std::io::Error },
    #[snafu(display("Unable to get filename"))]
    FolderError,
    #[snafu(display("Unable to convert to path to folder"))]
    PathError,
}

fn main() -> std::io::Result<()> {
    let x = run();
    match x {
        Ok(()) => print!("Done what you asked"),
        Err(err) => eprintln!("Error :::  {}", err),
    }

    Ok(())
}

fn zellij_sessions() -> Result<Vec<String>, ZxrError> {
    let output = Command::new(ZELLIJ_COMMAND)
        .arg("list-sessions")
        .output()
        .map_err(|err| ZxrError::ZellijExecutingZellijError { error: err })?;

    let session_list = String::from_utf8_lossy(&output.stdout).to_string();

    let all_sessions: Vec<String> = session_list
        .lines()
        .map(|line| line.replace("(current)", "").trim_end().to_string())
        .collect();
    Ok(all_sessions)
}

fn get_folder_name() -> Result<String, ZxrError> {
    current_dir()
        .map_err(|err| ZxrError::LoadDirectoryError { error: err })
        .and_then(|dir| {
            return dir.file_name().map_or(Err(ZxrError::FolderError), |r| {
                Ok(r.to_str().map(String::from))
            });
        })
        .and_then(|r| return r.map_or(Err(ZxrError::PathError), |r| return Ok(r)))
}

fn run() -> Result<(), ZxrError> {
    // let cur_dir: PathBuf =
    //     current_dir().map_err(|err| ZxrError::LoadDirectoryError { error: err })?;

    //     let session_name = cur_dir
    //     .file_name()
    //     .map_or(Err(ZxrError::FolderError), |r| {
    //         Ok(r.to_str().unwrap().to_owned())
    //     })?;

    // let session_name: String = current_dir()
    // .map_err(|err| ZxrError::LoadDirectoryError { error: err })
    // .and_then(|dir| {
    //     return dir.file_name().map_or(Err(ZxrError::FolderError), |r| {
    //         r.to_str()
    //             .map_or(Err(ZxrError::PathError), |r| return Ok(r.to_owned()))
    //     });
    // })?;

    // let all_sessions: Vec<String> = all_sessions

    let all_sessions: Vec<String> = zellij_sessions()?;

    let session_name: String = get_folder_name()?;

    //

    if all_sessions.contains(&session_name) {
        let err_while_attahcing_session = Command::new(ZELLIJ_COMMAND)
            .arg("attach")
            .arg("--create")
            .arg(session_name)
            .exec();

        return Err(ZxrError::ZellijSessionLoadError {
            error: err_while_attahcing_session,
        });
    } else {
        let err_while_creating_session = Command::new(ZELLIJ_COMMAND)
            .stderr(Stdio::null())
            .arg("--layout")
            .arg("code")
            .arg("--session")
            .arg(session_name)
            .exec();
        return Err(ZxrError::ZellijSessionLoadError {
            error: err_while_creating_session,
        });
    }
}
