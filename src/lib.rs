pub struct Parser {
    proj_title: String,
    proj_desc: String,
    flags: Vec<Flag>,
    arguments: Vec<Argument>
}

pub struct Flag {
    title: String,
    desc: String,
    options: Vec<String>,
}

pub struct Argument {
    title: String,
    desc: String
}

impl Parser {
    // help command
    pub fn help(&self) -> String {
        // Get the flags
        let mut flag_str: String = String::new();
        for flag in &self.flags {
            flag_str.push_str(&("    -".to_owned() + &flag.title + " "));
            for option in &flag.options {
                flag_str.push_str(&("<".to_owned() + option + "> "));
            }
            flag_str.push_str(&(":\n\t ".to_owned() + &flag.desc + "\n"));
        }
        flag_str.push_str("\n");

        // Get the arguments
        let mut args_str: String = String::from(" Arguments:\n");
        for arg in &self.arguments {
            args_str.push_str(&("    ".to_owned() + &arg.title + " :\n\t " + &arg.desc + "\n"));
        }

        // Create the help message
        let mut help_msg = 
            self.proj_title.clone() + ", " + &self.proj_desc.clone() +
            "\nUsage: -h for help:\n\n \
            Options:\n";
        help_msg.push_str(&flag_str);
        help_msg.push_str(&args_str);
        help_msg
    }
}

pub fn create_flag(title: &str, desc: &str, options: Vec<&str>) -> Flag {
    let mut return_vec: Vec<String> = Vec::new();
    for option in options {
        return_vec.push(option.to_string());
    }
    Flag {
        title: title.to_string(),
        desc: desc.to_string(),
        options: return_vec,
    }
}

pub fn create_arg(title: &str, desc: &str) -> Argument {
    Argument { title: title.to_string(), desc: desc.to_string() }
}

pub fn create_parser(
    project_title: &str,
    project_description: &str,
    flags: Vec<Flag>,
    arguments: Vec<Argument> ) -> Parser {
        Parser {
            proj_title: project_title.to_string(),
            proj_desc: project_description.to_string(),
            flags: flags,
            arguments: arguments,
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        
        let mut flags: Vec<Flag> = Vec::new();
        flags.push(create_flag("a", "This is the a flag", vec!["some"]));
        flags.push(create_flag("b", "This is the b flag", vec!["some", "thing"]));
        flags.push(create_flag("c", "This is the c flag", vec!["some"]));
        flags.push(create_flag("d", "This is the d flag", vec!["some"]));

        let mut args: Vec<Argument> = Vec::new();
        args.push(create_arg("foo", "This is the foo argument"));
        args.push(create_arg("bar", "This is the bar argument"));

        let arg_parser = create_parser("Test Parser", "Tests arguments", flags, args);

        println!("{}", arg_parser.help());
        assert_eq!(true, false);
    }
}
