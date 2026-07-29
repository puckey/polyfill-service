//! Builds the SQLite polyfill store consumed by the service.
//!
//! Walks `polyfill-libraries/<version>/<feature>/{raw,min}.js` and writes each
//! file into a `files_<version_with_underscores>` table, mirroring the schema
//! the service queries at runtime.
//!
//! Usage:
//!   build-db [--libraries DIR] [--db PATH] [--versions all|3.111.0,3.25.1]

use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let mut libraries = PathBuf::from("polyfill-libraries");
    let mut db = PathBuf::from("polyfills.db");
    let mut versions: Option<Vec<String>> = None; // None = all

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--libraries" => {
                libraries = PathBuf::from(args.next().expect("--libraries needs a value"));
            }
            "--db" => db = PathBuf::from(args.next().expect("--db needs a value")),
            "--versions" => {
                let value = args.next().expect("--versions needs a value");
                if value != "all" {
                    versions = Some(value.split(',').map(|v| v.trim().to_owned()).collect());
                }
            }
            other => {
                eprintln!("unknown argument: {other}");
                eprintln!(
                    "usage: build-db [--libraries DIR] [--db PATH] [--versions all|3.111.0,3.25.1]"
                );
                std::process::exit(2);
            }
        }
    }

    let mut version_dirs = fs::read_dir(&libraries)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", libraries.display()))
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_dir())
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .filter(|name| versions.as_ref().is_none_or(|list| list.contains(name)))
        .collect::<Vec<_>>();
    version_dirs.sort();

    if let Some(requested) = &versions {
        for version in requested {
            assert!(
                version_dirs.contains(version),
                "version {version} not found in {}",
                libraries.display()
            );
        }
    }
    assert!(
        !version_dirs.is_empty(),
        "no library versions found in {}",
        libraries.display()
    );

    let mut conn = rusqlite::Connection::open(&db).expect("failed to open database");
    conn.pragma_update(None, "journal_mode", "OFF").unwrap();
    conn.pragma_update(None, "synchronous", "OFF").unwrap();

    for version in &version_dirs {
        let table = format!("files_{}", version.replace('.', "_"));
        conn.execute_batch(&format!(
            "DROP TABLE IF EXISTS {table};
             CREATE TABLE {table} (name TEXT PRIMARY KEY, value BLOB NOT NULL);"
        ))
        .unwrap();

        let tx = conn.transaction().unwrap();
        let mut count = 0_usize;
        {
            let mut insert = tx
                .prepare(&format!("INSERT INTO {table} (name, value) VALUES (?, ?)"))
                .unwrap();

            let features = fs::read_dir(libraries.join(version))
                .unwrap_or_else(|err| panic!("failed to read version {version}: {err}"))
                .filter_map(Result::ok)
                .filter(|entry| entry.path().is_dir());

            for feature in features {
                let feature_name = feature.file_name().to_string_lossy().into_owned();
                for file_name in ["raw.js", "min.js", "meta.json"] {
                    let file = feature.path().join(file_name);
                    if let Ok(contents) = fs::read(&file) {
                        insert
                            .execute(rusqlite::params![
                                format!("/{feature_name}/{file_name}"),
                                contents
                            ])
                            .unwrap_or_else(|err| {
                                panic!("failed to insert {}: {err}", file.display())
                            });
                        count += 1;
                    }
                }
            }

            let aliases_file = libraries.join(version).join("aliases.json");
            let contents = fs::read(&aliases_file)
                .unwrap_or_else(|err| panic!("failed to read {}: {err}", aliases_file.display()));
            insert
                .execute(rusqlite::params!["/aliases.json", contents])
                .unwrap_or_else(|err| panic!("failed to insert {}: {err}", aliases_file.display()));
            count += 1;
        }
        tx.commit().unwrap();
        println!("{version}: {count} files -> {table}");
    }

    println!(
        "wrote {} version(s) to {}",
        version_dirs.len(),
        db.display()
    );
}
