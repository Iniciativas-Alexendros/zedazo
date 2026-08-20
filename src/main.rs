use clap::{CommandFactory, Parser};
use tracing_subscriber::EnvFilter;
use zedazo::application::audit;
use zedazo::application::cribar;
use zedazo::application::stats;
use zedazo::interfaces::cli::{Cli, Command};

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("zedazo=info".parse()?))
        .init();

    let cli = Cli::parse();

    match cli.command {
        Command::Cribar {
            input,
            output,
            audit: audit_path,
            config,
            source,
            dry_run,
            strict,
        } => {
            let (stats, _contacts) = cribar::execute(
                &input,
                output.as_deref(),
                audit_path.as_deref(),
                config.as_deref(),
                &source,
                dry_run,
                strict,
            )?;

            println!("{}", stats);
        }
        Command::Audit {
            input,
            output,
            config,
        } => {
            audit::execute(&input, output.as_deref(), config.as_deref(), "auto")?;
        }
        Command::Stats { input, format } => {
            stats::execute(&input, &format)?;
        }
        Command::Export {
            input,
            output,
            format,
        } => {
            let (_stats, contacts) =
                cribar::execute(&input, None, None, None, "auto", true, false)?;
            match format.as_str() {
                "json" => zedazo::infrastructure::json_writer::export_json(&contacts, &output)?,
                _ => zedazo::infrastructure::csv_writer::export_csv(&contacts, &output)?,
            }
        }
        Command::Completions { shell } => {
            let mut cmd = Cli::command();
            let name = cmd.get_name().to_string();
            clap_complete::generate(shell, &mut cmd, name, &mut std::io::stdout());
        }
    }

    Ok(())
}
