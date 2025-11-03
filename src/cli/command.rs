use crate::errors::InvalidCommandError;

pub(crate) enum Command {
    SHA { path: String },
    Help,
}

impl TryFrom<Vec<String>> for Command {
    type Error = InvalidCommandError;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        if value.len() == 0 {
            return Err(InvalidCommandError::new("No command entered. Type \"casper help\" to see available options."))
        }

        let command: &str = value.get(0).expect("The previous check guarantees that index 0 exists.");

        return match command {
            "sha" => {
                if value.len() != 2 {
                    return Err(InvalidCommandError::new("The \"sha\" command requires a <PATH> argument pointing to the .gh file that you want to get the SHA of."));
                }

                let path: &str = value.get(1).expect("The previous check guarantees that index 1 exists.");

                if !path.ends_with(".gh") {
                    return Err(InvalidCommandError::new("The <PATH> argument for the \"sha\" command must end with the .gh suffix (which identifies a casper code file)."));
                }

                Ok(Self::SHA { path: path.to_string() })
            }
            "help" => Ok(Self::Help),
            _ => Err(InvalidCommandError::new(format!("{} is not a valid casper command. Type \"help\" to see available options.", command).as_str()))
        }       
    }
}
