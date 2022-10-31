//! The Argument Parser of this Project.

use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "projectgen",
    author,
    version,
    about,
    long_about = "A project generator that generates from a template."
)]
/// projectgen is a project generator that generates from a template.
pub struct Args {
    /// The subcommand to use.
    #[command(subcommand)]
    pub command: Commands,
}

/// Enum that holds every subcommand of [Args]
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Creates a new Directory and then inits the Project with the chossen template.
    #[command(arg_required_else_help = true)]
    New {
        /// The name of the Template
        template: String,

        /// The directory to create the template in.
        path: PathBuf,
    },

    /// Initalizes the Project in the current directory with a chossen template.
    #[command(arg_required_else_help = true)]
    Init {
        /// The name of the Template
        template: String,
    },
}
