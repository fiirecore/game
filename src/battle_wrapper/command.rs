use worldcli::worldlib::map::battle::BattleId;

pub enum BattleCommand {
    Faint(BattleId, Option<usize>),
    End,
}

impl super::BattleWrapper {
    pub fn process(result: String) -> Result<BattleCommand, &'static str> {
        let mut args = result.split_ascii_whitespace();

        let (command, mut args) = if let Some(command) = args.next() {
            (command, args)
        } else {
            return Err("Could not parse command!");
        };

        fn battleid(
            trainer: Option<&BattleId>,
            arg: Option<&str>,
        ) -> Result<BattleId, &'static str> {
            match arg {
                Some(arg) => match arg {
                    "local" => Ok(BattleId::Player),
                    "remote" => match trainer {
                        Some(trainer) => Ok(*trainer),
                        None => Ok(BattleId::Wild),
                    },
                    _ => Err("Unknown player ID!"),
                },
                None => Err("Please provide a player ID!"),
            }
        }

        match command {
            "end" => Ok(BattleCommand::End),
            "faint" => match battleid(None, args.next()) {
                Ok(id) => Ok(BattleCommand::Faint(
                    id,
                    args.next().and_then(|s| s.parse::<usize>().ok()),
                )),
                Err(err) => Err(err),
            },
            _ => Err("Unknown command."),
        }
    }
}
