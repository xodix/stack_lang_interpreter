use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::ExecutionMode;

#[derive(Debug, Parser)]
#[command(name = "stack_lang")]
#[command(about = "Compiler and interpreter for stack_lang.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Run code.
    #[command(arg_required_else_help = true)]
    Run {
        /// Should the code be ran code in binary mode.
        #[arg(long)]
        bin: bool,
        /// Path to code.
        #[arg(required = true)]
        path: PathBuf,
    },

    /// Build code to binary format.
    #[command(arg_required_else_help = true)]
    Build {
        // Path to output
        #[arg(long, short)]
        output: Option<PathBuf>,
        /// Path to source code.
        #[arg(required = true)]
        input_file: PathBuf,
    },
}

pub fn get_execution_mode() -> crate::ExecutionMode {
    let args = Cli::parse();

    match args.command {
        Commands::Run { bin, path } => {
            if bin {
                ExecutionMode::RunBinary(path)
            } else {
                ExecutionMode::Run(path)
            }
        }
        Commands::Build { output, input_file } => ExecutionMode::Build(input_file, output),
    }
}
