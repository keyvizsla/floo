/*
 * Copyright (C) 2026 Leon Degel-Koehn
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use std::env;
use std::path::PathBuf;

use rusqlite::{Connection, Result, params};

use crate::{fireplace::Fireplace, utils::appdata_dir};

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
            directory text not null unique,
            notes text,
            last_accessed integer
        )",
        (),
    )?;

    Ok(conn)
}

// Return a list of projects in the local database.
pub fn get_projects() -> Result<Vec<Fireplace>> {
    let conn = get_safe_db_connection()?;

    let projects = {
        let mut stmt = conn.prepare(
            "SELECT name, directory, notes, last_accessed
         FROM projects;",
        )?;

        let project_iter = stmt.query_map([], |row| {
            Ok(Fireplace::new(
                row.get::<usize, String>(0)?,
                row.get::<usize, String>(1)?.into(),
                row.get::<usize, String>(2)?,
                row.get::<usize, i64>(3)?,
            ))
        })?;

        project_iter.collect::<Result<Vec<Fireplace>, _>>()?
    };

    conn.close().map_err(|(_, err)| err)?;

    Ok(projects)
}

pub fn add_project(project: Fireplace) -> Result<()> {
    let conn = get_safe_db_connection()?;
    let mut stmt = conn.prepare(
        "INSERT INTO projects (name, directory, notes, last_accessed) VALUES (?1, ?2, ?3, ?4)",
    )?;
    stmt.execute(params![
        project.name,
        project.get_directory().to_str(),
        project.notes,
        project.last_accessed,
    ])?;
    Ok(())
}

pub fn remove_project(project: Fireplace) -> Result<()> {
    let conn = get_safe_db_connection()?;
    let mut stmt = conn.prepare("DELETE FROM projects WHERE name = ?1")?;
    stmt.execute(params![project.name])?;
    Ok(())
}

pub fn change_notes(project: &Fireplace, new_notes: &str) -> Result<()> {
    let conn = get_safe_db_connection()?;
    let mut stmt = conn.prepare("UPDATE projects SET notes = ?1 WHERE name = ?2;")?;
    stmt.execute(params![new_notes, project.name,])?;
    Ok(())
}

/// Update the last_accessed property of the project to be now
pub fn set_last_accessed_to_now(project: &Fireplace) -> Result<()> {
    let conn = get_safe_db_connection()?;
    let mut stmt = conn.prepare("UPDATE projects SET last_accessed = ?1 WHERE name = ?2;")?;
    let seconds = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    stmt.execute(params![seconds, project.name,])?;
    Ok(())
}

/// Private function to resolve the true filepath
/// of the database based on environment configuration.
fn db_filepath() -> PathBuf {
    let path = env::var("FLOO_DB_PATH").ok().map(PathBuf::from);
    if let Some(resolved_path) = path {
        return resolved_path;
    }

    let floo_directory = appdata_dir();
    floo_directory.join(".floo.db")
}
