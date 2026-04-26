use std::env;
use std::path::PathBuf;

use rusqlite::{params, Connection, Result};

use crate::project::Project;

/// Return a safe connection to the database.
/// A safe connection is one, where users can assume that all
/// key tables for the program to work are available.
/// Return an error if no such connection can be established.
pub fn get_safe_db_connection() -> Result<Connection> {
    let conn = Connection::open(db_filepath())?;
    conn.execute(
        "
        create table if not exists projects (
            id integer primary key,
            name text not null unique,
            directory text not null unique
        )",
        (),
    )?;

    return Ok(conn);
}

// Return a list of projects in the local database.
pub fn get_projects() -> Result<Vec<Project>> {
    let conn = get_safe_db_connection()?;

    let projects = {
        let mut stmt = conn.prepare(
            "SELECT name, directory
         FROM projects;",
        )?;

        let project_iter = stmt.query_map([], |row| {
            Ok(Project {
                name: row.get::<usize, String>(0)?,
                directory: row.get::<usize, String>(1)?.into(),
            })
        })?;

        project_iter.collect::<Result<Vec<Project>, _>>()?
    };

    conn.close().map_err(|(_, err)| err)?;

    Ok(projects)
}

pub fn add_project(project: Project) -> Result<()> {
    let conn = get_safe_db_connection()?;
    let mut stmt = conn.prepare(
        "INSERT INTO projects (name, directory) VALUES (?1, ?2)",
    )?;
    stmt.execute(params![project.name, project.directory.to_str()])?;
    Ok(())
}

pub fn remove_project(project: Project) -> Result<()> {
    let conn = get_safe_db_connection()?;
    let mut stmt = conn.prepare(
        "DELETE FROM projects WHERE name = ?1",
    )?;
    stmt.execute(params![project.name])?;
    Ok(())
}

/// Private function to resolve the true filepath
/// of the database based on environment configuration.
fn db_filepath() -> PathBuf {
    let path = env::var("FLOO_DB_PATH").ok().map(PathBuf::from);
    if path.is_some() {
        return path.unwrap();
    }

    let path = env::var("XDG_DATA_HOME").ok().map(PathBuf::from);
    if path.is_some() {
        return path.unwrap().join(".floo.db");
    }

    let home_directory = PathBuf::from(env::var("HOME").expect("Cannot deduce db path without HOME directory"));
    let floo_directory = home_directory.join(".local/share/floo/");

    if !floo_directory.exists() {
        std::fs::create_dir_all(&floo_directory).expect("Failed to create .floo directory");
    }

    return floo_directory.join(".floo.db");
}
