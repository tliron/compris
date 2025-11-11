use super::commands::*;

use {clap::*, kutil::cli::log::*, problemo::*};

pub fn run() -> Result<(), Problem> {
    let root = Root::parse();

    if !root.quiet {
        root.output_colorize.initialize();
        initialize_tracing(root.verbose + 2, root.log_path.as_ref())?;
    }

    match &root.subcommand {
        None => root.convert()?,
        Some(SubCommand::Version(version)) => version.run::<Root>(),
        Some(SubCommand::Completion(completion)) => completion.run::<Root>(),
        Some(SubCommand::Manual(manual)) => manual.run::<Root>()?,
    }

    Ok(())
}
