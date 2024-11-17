/*
 * Copyright (c) 2024 Javier Martinez Canillas
 *
 * SPDX-License-Identifier: MIT
 */

use std::fs;
use std::io::{Read, Write};
use std::path;
use std::process;

use clap::ArgMatches;

fn create_app_repo_dir() -> Result<path::PathBuf, Box<dyn std::error::Error>> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("konfigura")?;
    let repo_dir = xdg_dirs.create_data_directory("repo")?;
    Ok(repo_dir)
}

fn get_app_repo_dir() -> path::PathBuf {
    let xdg_dirs =
        xdg::BaseDirectories::with_prefix("konfigura").expect("failed to get XDG directories");

    xdg_dirs
        .find_data_file("repo")
        .expect("filed to get application data store")
}

fn handle_init(_matches: &ArgMatches) {
    let path = create_app_repo_dir().expect("failed to create the application data store");
    git2::Repository::init(path).expect("failed to init the git repository");
}

fn handle_show(matches: &ArgMatches) {
    let path = get_app_repo_dir();
    let config_name = matches.get_one::<String>("NAME").unwrap();

    let mut file = fs::File::open(format!("{}/{}.config", path.display(), config_name))
        .unwrap_or_else(|e| {
            log::error!("failed to open {}.config: {}", config_name, e);
            process::exit(1);
        });

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("failed to read the file");
    println!("{}", contents);
}

fn handle_list(_matches: &ArgMatches) {
    let path = get_app_repo_dir();

    fs::read_dir(path)
        .unwrap()
        .filter_map(Result::ok)
        .filter(|d| {
            if let Some(e) = d.path().extension() {
                e == "config"
            } else {
                false
            }
        })
        .for_each(|f| {
            println!(
                "{}",
                f.path()
                    .with_extension("")
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
            )
        });
}

fn commit_config_file(
    repo_path: path::PathBuf,
    config_name: &str,
    src_path: &path::PathBuf,
    dst_path: &path::PathBuf,
    action: &str,
) {
    let mut src_file = fs::File::open(src_path).unwrap_or_else(|e| {
        log::error!("failed to open source {}: {}", src_path.display(), e);
        process::exit(1);
    });

    let mut dst_file = fs::File::create(dst_path).unwrap_or_else(|e| {
        log::error!("failed to open destination {}: {}", config_name, e);
        process::exit(1);
    });

    let mut contents = String::new();
    src_file.read_to_string(&mut contents).unwrap_or_else(|e| {
        log::error!(
            "failed to read content of source file {}: {}",
            config_name,
            e
        );
        process::exit(1);
    });

    for line in contents.lines() {
        if !line.starts_with('#') && !line.is_empty() {
            writeln!(dst_file, "{}", line).unwrap_or_else(|e| {
                log::error!("failed to write to destination {}: {}", config_name, e);
                process::exit(1);
            });
        }
    }

    let repo = git2::Repository::open(repo_path).expect("failed to open the git repository");

    let mut index = repo.index().expect("failed to get the git repo index");

    index
        .add_path(path::Path::new(&format!("{}.config", config_name)))
        .expect("failed to add config to git repo index");

    index.write().unwrap();

    let oid = index
        .write_tree()
        .expect("failed to write git repo index as a tree");

    let signature = repo.signature().expect("failed to get commit signature");
    let tree = repo.find_tree(oid).expect("failed to find the repo tree");
    let message = format!("{}: {} config file", config_name, action);

    let parent = repo
        .head()
        .ok()
        .and_then(|head| head.peel_to_commit().ok())
        .into_iter()
        .collect::<Vec<_>>();

    let parent_refs: Vec<&git2::Commit> = parent.iter().collect();

    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        &message,
        &tree,
        &parent_refs,
    )
    .expect("failed to commit to git repo");
}

fn handle_add(matches: &ArgMatches) {
    let repo_path = get_app_repo_dir();
    let src_path = matches.get_one::<path::PathBuf>("PATH").unwrap();
    let config_file_name = src_path.with_extension("");
    let mut config_name = String::from(config_file_name.file_name().unwrap().to_str().unwrap());

    if let Some(name) = matches.get_one::<String>("name") {
        config_name = name.to_string();
    }

    let config_path = format!("{}/{}.config", repo_path.display(), config_name);
    let dst_path_buf = path::PathBuf::from(config_path);
    let dst_path = &dst_path_buf;

    if fs::metadata(dst_path).is_ok() {
        log::error!("{}.config already exists", config_name);
        process::exit(1);
    };

    commit_config_file(repo_path, &config_name, src_path, dst_path, "add");
}

fn handle_update(matches: &ArgMatches) {
    let repo_path = get_app_repo_dir();
    let src_path = matches.get_one::<path::PathBuf>("PATH").unwrap();
    let config_name = String::from(matches.get_one::<String>("NAME").unwrap());

    let config_path = format!("{}/{}.config", repo_path.display(), config_name);
    let dst_path_buf = path::PathBuf::from(config_path);
    let dst_path = &dst_path_buf;

    if fs::metadata(dst_path).is_err() {
        log::error!("{}.config does not exists", config_name);
        process::exit(1);
    };

    commit_config_file(repo_path, &config_name, src_path, dst_path, "update");
}

fn handle_remove(matches: &ArgMatches) {
    let path = get_app_repo_dir();
    let config_name = matches.get_one::<String>("NAME").unwrap();

    fs::remove_file(format!("{}/{}.config", path.display(), config_name)).unwrap_or_else(|e| {
        log::error!("failed to remove {}.config: {}", config_name, e);
        process::exit(1);
    });
}

pub fn run(matches: ArgMatches) {
    match matches.subcommand() {
        Some(("init", sub_matches)) => handle_init(sub_matches),
        Some(("add", sub_matches)) => handle_add(sub_matches),
        Some(("update", sub_matches)) => handle_update(sub_matches),
        Some(("remove", sub_matches)) => handle_remove(sub_matches),
        Some(("list", sub_matches)) => handle_list(sub_matches),
        Some(("show", sub_matches)) => handle_show(sub_matches),
        _ => unreachable!(),
    }
}
