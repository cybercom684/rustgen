mod project;
mod module;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rustgen")]
#[command(about = "Rust Code Generator")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    NewLib {
        name: String,
    },
    AddModule {
        name: String,
        #[arg(short, long)]
        project: String,
    },
    AddService {
        name: String,
        #[arg(short, long)]
        project: String,
        #[arg(short, long)]
        trait_name: Option<String>,
    },
}


fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::NewLib { name } => project::create_lib(&name),
        Commands::AddModule { name, project } => module::add_module(&project, &name),
        Commands::AddService { name, project, trait_name } => module::add_service(&project, &name, trait_name),
    }
}