/*
 * Copyright (c) 2024 Javier Martinez Canillas
 *
 * SPDX-License-Identifier: MIT
 */

extern crate clap;

use std::path;

fn cli() -> clap::Command {
    clap::Command::new("konfigura")
        .about("Tool to manage Linux kernel configuration files")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(clap::Command::new("init").about("Initialize configure file data store"))
        .subcommand(
            clap::Command::new("add")
                .about("Add a configure file")
                .arg_required_else_help(true)
                .arg(
                    clap::arg!(-n --name <NAME> ... "Name of the configuration file")
                        .required(false),
                )
                .arg(
                    clap::arg!(<PATH> ... "Path to the configuration file to add")
                        .value_parser(clap::value_parser!(path::PathBuf)),
                ),
        )
        .subcommand(
            clap::Command::new("update")
                .about("Update a configure file")
                .arg_required_else_help(true)
                .arg(clap::arg!(<NAME> ... "Name of the configuration file to update"))
                .arg(
                    clap::arg!(<PATH> ... "Path to the new configuration file")
                        .value_parser(clap::value_parser!(path::PathBuf))
                        .last(true),
                ),
        )
        .subcommand(
            clap::Command::new("remove")
                .about("Remove a configure file")
                .arg_required_else_help(true)
                .arg(clap::arg!(<NAME> ... "Name of the configuration file to remove")),
        )
        .subcommand(clap::Command::new("list").about("List configure files"))
        .subcommand(
            clap::Command::new("show")
                .about("Show a configure file")
                .arg_required_else_help(true)
                .arg(clap::arg!(<NAME> ... "Name of the configuration file to show")),
        )
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let matches = cli().get_matches();

    konfigura::run(matches);
}
