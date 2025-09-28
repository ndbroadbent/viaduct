use std::path::{Path, PathBuf};

use anyhow::{Context, Result, anyhow};
use clap::{Args, Parser, Subcommand};
use walkdir::WalkDir;

use via_core::{codegen, parser, writer};

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Gen(args) => run_gen(args),
        Commands::Check(args) => run_check(args),
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about = "Via CLI (Viaduct MVP)", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Parse Via files and emit generated Rust code
    Gen(GenArgs),
    /// Parse Via files and report errors without emitting files
    Check(CheckArgs),
}

#[derive(Args, Debug)]
struct GenArgs {
    /// Directory containing .via files (defaults to ./app)
    #[arg(long, default_value = "app")]
    app: PathBuf,

    /// Output directory for generated Rust code (defaults to ./generated)
    #[arg(long, default_value = "generated")]
    out: PathBuf,

    /// Optional path for serialized IR (defaults to <out>/via.ir.json)
    #[arg(long)]
    ir: Option<PathBuf>,

    /// Parse and report resources without writing files
    #[arg(long)]
    dry_run: bool,
}

#[derive(Args, Debug)]
struct CheckArgs {
    /// Directory containing .via files (defaults to ./app)
    #[arg(long, default_value = "app")]
    app: PathBuf,
}

fn run_gen(args: GenArgs) -> Result<()> {
    let files = collect_via_files(&args.app)?;
    if files.is_empty() {
        println!("No .via files found under {}", args.app.display());
        return Ok(());
    }

    let mut resources = Vec::new();
    for file in files {
        let mut parsed = parser::parse_file(&file)?;
        resources.append(&mut parsed);
    }

    println!("Parsed {} resource(s)", resources.len());

    if args.dry_run {
        for resource in &resources {
            println!(" - {} (from {})", resource.name, resource.file_path);
        }
        return Ok(());
    }

    writer::clean_output_root(&args.out)?;

    let generation = codegen::generate(&resources)?;
    writer::write_files(&args.out, &generation.files)?;

    let ir_path = args.ir.unwrap_or_else(|| args.out.join("via.ir.json"));
    let ir_json = serde_json::to_string_pretty(&resources)?;
    writer::write_ir_file(&ir_path, &ir_json)?;

    println!(
        "Wrote {} generated file(s) into {}",
        generation.files.len(),
        args.out.display()
    );
    println!("IR written to {}", ir_path.display());

    Ok(())
}

fn run_check(args: CheckArgs) -> Result<()> {
    let files = collect_via_files(&args.app)?;
    if files.is_empty() {
        println!("No .via files found under {}", args.app.display());
        return Ok(());
    }

    let mut total = 0usize;
    for file in files {
        let parsed = parser::parse_file(&file)?;
        total += parsed.len();
    }

    println!("OK: parsed {} resource(s)", total);
    Ok(())
}

fn collect_via_files(root: &Path) -> Result<Vec<PathBuf>> {
    if !root.exists() {
        return Err(anyhow!("Via directory not found: {}", root.display()));
    }

    let mut files = Vec::new();
    for entry in WalkDir::new(root) {
        let entry = entry.with_context(|| "Failed to walk directory entry")?;
        if entry.file_type().is_file() && entry.path().extension().is_some_and(|ext| ext == "via") {
            files.push(entry.into_path());
        }
    }
    files.sort();
    Ok(files)
}
