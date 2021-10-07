use twilight_model::application::command::{ChoiceCommandOptionData, Command, CommandOption};

pub fn commands() -> Vec<Command> {
    vec![
        Command {
            id: None,
            guild_id: None,
            application_id: None,
            name: "trinity".to_owned(),
            description: "Gets information abount the trinity.".to_owned(),
            options: vec![],
            default_permission: None,
        }
    ]
}