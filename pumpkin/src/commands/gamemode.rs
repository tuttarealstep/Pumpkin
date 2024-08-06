use super::Command;

pub struct GamemodeCommand {}

impl GamemodeCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl<'a> Command<'a> for GamemodeCommand {
    const NAME: &'a str = "gamemode";

    const DESCRIPTION: &'a str = "Changes the gamemode for a Player";

    fn on_execute(
        sender: &mut super::CommandSender<'a>,
        command: String,
        _server: &mut crate::server::Server,
    ) {
        let player = sender.as_mut_player().unwrap();
        let args: Vec<&str> = command.split_whitespace().collect();

        if args.len() != 2 {
            // todo red
            player.send_message("Usage: /gamemode <mode>".into());
            return;
        }

        let mode_str = args[1].to_lowercase();
        match mode_str.parse() {
            Ok(mode) => {
                player.set_gamemode(mode);
                player.send_message(format!("Set own game mode to {mode_str}").into());
            }
            Err(_) => {
                // todo red
                player.send_message("Invalid gamemode".into());
            }
        }
    }

    // todo: support console, (name required)
    fn player_required() -> bool {
        true
    }
}