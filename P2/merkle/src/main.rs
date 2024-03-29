use clap::{Parser, Subcommand};

mod prover;
mod verifier;
mod util;

#[derive(Parser)]
struct Cli {
   #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(name = "prove")]
    Prove { leaf_pos: usize },
    #[clap(name = "verify")]
    Verify { proof_file: String },
}


fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Prove { leaf_pos } => {
            prover::run(*leaf_pos);
        }
        Commands::Verify { proof_file } => {
            verifier::run(proof_file)
        }
    }
}
