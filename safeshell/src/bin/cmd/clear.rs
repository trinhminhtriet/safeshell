use anyhow::Result;
use clap::{Arg, ArgMatches, Command};

use safeshell::clearer::Clearer;
use safeshell::{config::Config, engine, Emojis, ShellContext};

pub fn command() -> Command<'static> {
    Command::new("clear")
        .about("Remove or mask the findings from shell history")
        .arg(
            Arg::new("backup")
                .long("backup")
                .help("Backup history file before delete commands")
                .takes_value(false),
        )
        .arg(
            Arg::new("remove")
                .long("remove")
                .help("Remove history that contains secrets")
                .takes_value(false),
        )
}

pub fn run(
    matches: &ArgMatches,
    shells_context: &Vec<ShellContext>,
    config: &Config,
) -> Result<safeshell::data::CmdExit> {
    let en = engine::PatternsEngine::with_config(config)?;

    for shell_context in shells_context {
        if matches.is_present("backup") {
            match shell_context.backup() {
                Ok(path) => log::debug!("history backup successful: {}", path),
                Err(e) => {
                    return Ok(safeshell::data::CmdExit {
                        code: 1,
                        message: Some(format!(
                            "could not backup shell {:?} history. err: {e:?}",
                            shell_context.history.shell,
                        )),
                    });
                }
            }
        }
    }

    let commands = en.find_history_commands_from_shell_list(shells_context)?;

    let emojis = Emojis::default();

    Clearer::write_findings(shells_context, &commands, matches.is_present("remove"))?;

    let sensitive_commands = commands.get_commands_with_secrets();

    if sensitive_commands.is_empty() {
        return Ok(safeshell::data::CmdExit {
            code: exitcode::OK,
            message: Some(format!(
                "{} Your shell is clean from sensitive data!",
                emojis.confetti
            )),
        });
    };

    let message = format!(
        " {} safeshell cleared {} sensitive commands",
        emojis.alarm,
        sensitive_commands.len()
    );

    Ok(safeshell::data::CmdExit {
        code: exitcode::OK,
        message: Some(message),
    })
}
