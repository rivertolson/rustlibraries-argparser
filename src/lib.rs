use std::env::Args;
use std::process;

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

pub struct ParsedArgs {
    pub flags: Vec<(String, String)>,
    pub arguments: Vec<String>,
}

impl Flag {
    pub fn new() -> Flag{
        Flag {
            title: String::new(),
            desc: String::new(),
            options: Vec::new(),
        }
    }
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
        if self.flags.len() > 0 {
            flag_str.push_str(" Options:\n");
            for flag in &self.flags {
                flag_str.push_str(&("    -".to_owned() + &flag.title + " "));
                for option in &flag.options {
                    flag_str.push_str(&("<".to_owned() + option + "> "));
                }
                flag_str.push_str(&(":\n\t ".to_owned() + &flag.desc + "\n"));
            }
            flag_str.push_str("\n");
        }
        
        // Get the arguments
        let mut args_str = String::new();
        if self.arguments.len() > 0 {
            args_str.push_str(" Arguments:\n");
            for arg in &self.arguments {
                args_str.push_str(&("    ".to_owned() + &arg.title + " :\n\t " + &arg.desc + "\n"));
            }  
        }
        
        // Create the help message
        let mut help_msg = 
            self.proj_title.clone() + ", " + &self.proj_desc.clone() +
            "\nUsage: -h for help:\n\n";
        help_msg.push_str(&flag_str);
        help_msg.push_str(&args_str);
        help_msg
    }

    // parse command
    pub fn parse(&self, args: &mut Args) -> ParsedArgs {
        // First argument is the programs path. Skip it.
        args.next();

        // Set up "globals" and return Options
        let mut is_option = false;
        let mut parsed = false;
        let mut current_flag: &Flag = &Flag::new();
        let mut used_flags: Vec<&Flag> = Vec::new();
        let mut used_args: Vec<&Argument> = Vec::new();
        let mut options: ParsedArgs = ParsedArgs { flags: Vec::new(), arguments: Vec::new() };

        'args: for arg in args{
            // First character of arg is '-', meaning it's a flag
            if arg.chars().nth(0) == Some('-') && !is_option {
                // Since we are at an option, check is_option to true.
                is_option = true;
                let arg_to_lower = arg[1..].to_ascii_lowercase();
                for flag in &self.flags {
                    // Check to make sure the flag hasn't been used already.
                    for used_flag in &used_flags {
                        if arg_to_lower == used_flag.title {
                            println!("Flags may only be used once, duplicate flag: -{}...\n{}", arg_to_lower, self.help());
                            process::exit(1);
                        }
                    }
                    if arg_to_lower == *flag.title {
                        current_flag = flag;
                        used_flags.push(&flag);
                        parsed = false;
                        continue 'args;
                    }
                }
                if arg_to_lower == "h" {
                    println!("{}", self.help());
                    process::exit(1);
                }
                else {
                    println!("Invalid flag: '{}'...\n{}", arg_to_lower, self.help());
                    process::exit(1);
                }
            }
            // Flags may only be followed by another flag if they don't take any arguments
            else if arg.chars().nth(0) == Some('-') && is_option {
                options.flags.push((current_flag.title.clone(), String::new()));
                // parsed = true;
                let arg_to_lower = arg[1..].to_ascii_lowercase();
                for flag in &self.flags {
                    // Check to make sure the flag hasn't been used already.
                    for used_flag in &used_flags {
                        if arg_to_lower == used_flag.title {
                            println!("Flags may only be used once, duplicate flag: -{}...\n{}", arg_to_lower, self.help());
                            process::exit(1);
                        }
                    }
                    if arg_to_lower == *flag.title {
                        current_flag = flag;
                        used_flags.push(&flag);
                        parsed = false;
                        continue 'args;
                    }
                }
                if arg_to_lower == "h" {
                    println!("{}", self.help());
                    process::exit(1);
                }
                else {
                    println!("Invalid flag, '{}'...\n{}", arg_to_lower, self.help());
                    process::exit(1);
                }
            }
            // Flags that do take arguments, check to make sure the option following is correct.
            else if arg.chars().nth(0) != Some('-') && is_option {
                if current_flag.options.len() == 0 {
                    println!("-{} does not take any arguments...\n{}", current_flag.title, self.help());
                    process::exit(1);
                }
                let arg_to_lower = arg[..].to_ascii_lowercase();
                options.flags.push((current_flag.title.clone(), arg_to_lower));
                parsed = true;
                is_option = false;
            }
            // Check if an arguemnt is passed in.
            else {
                let arg_to_lower = arg[..].to_ascii_lowercase();
                for parser_arg in &self.arguments {
                    // Check to make sure the argument hasn't been used already.
                    for used_arg in &used_args {
                        if arg_to_lower == used_arg.title {
                            println!("Arguments may only be used once, duplicate argument: {}...\n{}", arg_to_lower, self.help());
                            process::exit(1);
                        }
                    }
                    if parser_arg.title == arg {
                        options.arguments.push(parser_arg.title.clone());
                        used_args.push(&parser_arg);
                        break;
                    }
                }
                println!("Uknown arg: '{}'...\n{}", arg, self.help());
                process::exit(1);
            }
        }
        if !parsed {
            options.flags.push((current_flag.title.clone(), String::new()));
        }
        options
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
    fn third_flag_is_c() {
        let mut flags: Vec<Flag> = Vec::new();
        flags.push(create_flag("a", "This is the a flag", vec!["some"]));
        flags.push(create_flag("b", "This is the b flag", vec!["some", "thing"]));
        flags.push(create_flag("c", "This is the c flag", vec!["some"]));
        flags.push(create_flag("d", "This is the d flag", vec![]));

        let mut args: Vec<Argument> = Vec::new();
        args.push(create_arg("foo", "This is the foo argument"));
        args.push(create_arg("bar", "This is the bar argument"));

        let arg_parser = create_parser("Test Parser", "Tests arguments", flags, args);
        assert_eq!(arg_parser.flags[2].title, "c");
    }

    #[test]
    fn fourth_flag_has_zero_options() {
        let mut flags: Vec<Flag> = Vec::new();
        flags.push(create_flag("a", "This is the a flag", vec!["some"]));
        flags.push(create_flag("b", "This is the b flag", vec!["some", "thing"]));
        flags.push(create_flag("c", "This is the c flag", vec!["some"]));
        flags.push(create_flag("d", "This is the d flag", vec![]));

        let mut args: Vec<Argument> = Vec::new();
        args.push(create_arg("foo", "This is the foo argument"));
        args.push(create_arg("bar", "This is the bar argument"));

        let arg_parser = create_parser("Test Parser", "Tests arguments", flags, args);
        assert_eq!(arg_parser.flags[3].options.len(), 0);
    }
}
