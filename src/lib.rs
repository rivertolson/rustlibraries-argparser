//! # argparser
//! 
//! This crate will parse flags and arguments that are
//! passed into a Rust program. This crate will not 
//! handle all the logic associated with the flags
//! and arguments as that is program specific. It will
//! check and validate that the flags and arguments
//! passed in are valid.
//! 
//! ## Features
//! - Creates a parser with a set of flags and arguments
//! which will check and validate options passed in when
//! running the program.
//! - Creates a custom help function for all the arguments
//! and flags
//! - Will stop the program if the arguments passed in are
//! not valid.
//! 
//! ## Examples
//! ``` rust
//! use argparser::*;
//! 
//! fn main() {
//!     let mut flags: Vec<Flag> = Vec::new();
//!     flags.push(create_flag("a", "This is the a flag", vec!["some"]));
//!     flags.push(create_flag("b", "This is the b flag", vec!["some", "thing"]));
//!     flags.push(create_flag("c", "This is the c flag", vec!["some"]));
//!     flags.push(create_flag("d", "This is the d flag", vec![]));
//!
//!     let mut args: Vec<Argument> = Vec::new();
//!     args.push(create_arg("foo", "This is the foo argument"));
//!     args.push(create_arg("bar", "This is the bar argument"));
//!
//!     let arg_parser = create_parser("Test Parser", "Tests arguments", flags, args);
//! }
//! ```
//! 
//! calling `arg_parser.help()` will generate the following:
//! 
//! ``` txt
//! Test Parser, Tests arguments
//! Usage: -h for help:
//!
//! Options:
//!    -a <some> :
//!	        This is the a flag
//!    -b <some> <thing> :
//!	        This is the b flag
//!    -c <some> :
//!     	 This is the c flag
//!    -d :
//!	        This is the d flag
//!
//! Arguments:
//!    foo :
//!     	 This is the foo argument
//!    bar :
//!     	 This is the bar argument
//! ```
//! 
//! ## Modules
//! Args: essential for parsing arguments.
//! Process: used for ending the program early when an error occurs.

use std::env::Args;
use std::process;

/// A parser. This is responsible for the help function
/// as well as handeling argument logic.
/// 
/// # Examples
/// ``` rust
/// use argparser::*
/// 
/// fn main() {
///     let mut flags: Vec<Flag> = Vec::new();
///     flags.push(create_flag("a", "This is the a flag", vec!["some"]));
///     flags.push(create_flag("b", "This is the b flag", vec!["some", "thing"]));
///     flags.push(create_flag("c", "This is the c flag", vec!["some"]));
///     flags.push(create_flag("d", "This is the d flag", vec![]));
/// 
///     let mut args: Vec<Argument> = Vec::new();
///     args.push(create_arg("foo", "This is the foo argument"));
///     args.push(create_arg("bar", "This is the bar argument"));
/// 
///     let parser: Parser = Parser {
///         project_title: "Project Name",
///         project_description: "Project Description",
///         flags: flags,
///         arguments: args,
///     }
/// }
/// ```
pub struct Parser {
    project_title: String,
    project_description: String,
    flags: Vec<Flag>,
    arguments: Vec<Argument>,
}

/// A flag structure meant to be passed to the flags vec in a Parser.
/// 
/// # Examples
/// ``` rust
/// use argparser::*;
/// 
/// fn main() {
///     let a_flag: Flag = Flag {
///         title: "a",
///         description: "The a flag",
///         options: vec!["option".to_string()],
///     }
/// }
/// ```
pub struct Flag {
    title: String,
    description: String,
    options: Vec<String>,
}

/// A collection of the flags, and associated options, as well as
/// the arguments that come from the parser.parse() function.
/// 
/// # Examples
/// ``` rust
/// use argparser::*;
/// 
/// fn main() {
///     let mut flags: Vec<Flag> = Vec::new();
///     flags.push(create_flag("a", "This is the a flag", vec!["some"]));
///     flags.push(create_flag("b", "This is the b flag", vec!["some", "thing"]));
///     flags.push(create_flag("c", "This is the c flag", vec!["some"]));
///     flags.push(create_flag("d", "This is the d flag", vec![]));
/// 
///     let mut args: Vec<Argument> = Vec::new();
///     args.push(create_arg("foo", "This is the foo argument"));
///     args.push(create_arg("bar", "This is the bar argument"));
/// 
///     let arg_parser = create_parser("Test Parser", "Tests arguments", flags, args);
/// 
///     let parsed_args: ParsedArgs = arg_parser.parse(&mut std::env::args());
/// }
/// ```
pub struct ParsedArgs {
    pub flags: Vec<(String, String)>,
    pub arguments: Vec<String>,
}

impl Flag {
    /// Creates a new and empty flag to be edited further.
    /// 
    /// # Examples
    /// ``` rust
    /// use argparser::*;
    /// 
    /// fn main() {
    ///     let new_flag: Flag = Flag::new();
    /// }
    /// ```
    pub fn new() -> Flag{
        Flag {
            title: String::new(),
            description: String::new(),
            options: Vec::new(),
        }
    }
}

/// A flag structure meant to be passed to the flags vec in a Parser.
/// 
/// # Example
/// ``` rust
/// use argparser::*;
/// 
/// fn main() {
///     let arg1: Argument = Argument {
///         title: "argument",
///         description: "The description for the argument",
///     }
/// }
/// ```
pub struct Argument {
    title: String,
    description: String,
}

impl Argument {
    /// Creates a new and empty argument to be edited further.
    /// 
    /// # Examples
    /// ``` rust
    /// use argparser::*;
    /// 
    /// fn main() {
    ///     let new_argument: Argument = Argument::new();
    /// }
    /// ```
    pub fn new() -> Argument{
        Argument {
            title: String::new(),
            description: String::new(),
        }
    }
}

impl Parser {
    /// The help command for the Parser. Comes with a custom help
    /// function that can be called on a parser, and will get called
    /// if arguments passed into the program are invalid.
    /// 
    /// # Examples
    /// ``` rust
    /// use argparser::*;
    /// 
    /// fn main() {
    ///     let mut flags: Vec<Flag> = Vec::new();
    ///     flags.push(create_flag("a", "This is the a flag", vec!["some"]));
    ///     flags.push(create_flag("b", "This is the b flag", vec!["some", "thing"]));
    ///     flags.push(create_flag("c", "This is the c flag", vec!["some"]));
    ///     flags.push(create_flag("d", "This is the d flag", vec![]));
    ///
    ///     let mut args: Vec<Argument> = Vec::new();
    ///     args.push(create_arg("foo", "This is the foo argument"));
    ///     args.push(create_arg("bar", "This is the bar argument"));
    ///
    ///     let arg_parser = create_parser("Test Parser", "Tests arguments", flags, args);
    /// }
    /// ```
    /// 
    /// calling `arg_parser.help()` will generate the following:
    /// 
    /// ``` txt
    /// Test Parser, Tests arguments
    /// Usage: -h for help:
    ///
    /// Options:
    ///    -a <some> :
    ///	        This is the a flag
    ///    -b <some> <thing> :
    ///	        This is the b flag
    ///    -c <some> :
    ///     	 This is the c flag
    ///    -d :
    ///	        This is the d flag
    ///
    /// Arguments:
    ///    foo :
    ///     	 This is the foo argument
    ///    bar :
    ///     	 This is the bar argument
    /// ```
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
                flag_str.push_str(&(":\n\t ".to_owned() + &flag.description + "\n"));
            }
            flag_str.push_str("\n");
        }
        
        // Get the arguments
        let mut args_str = String::new();
        if self.arguments.len() > 0 {
            args_str.push_str(" Arguments:\n");
            for arg in &self.arguments {
                args_str.push_str(&("    ".to_owned() + &arg.title + " :\n\t " + &arg.description + "\n"));
            }  
        }
        
        // Create the help message
        let mut help_msg = 
            self.project_title.clone() + ", " + &self.project_description.clone() +
            "\nUsage: -h for help:\n\n";
        help_msg.push_str(&flag_str);
        help_msg.push_str(&args_str);
        help_msg
    }

    /// Parses the arguemnts that are passed into the program by
    /// comparing them to the Parser arguments. If the parsing
    /// fails then the program terminates. If the parsing is
    /// successful, then it returns the parsed args.
    /// 
    /// # Arguments
    /// - args: &mut Args
    /// 
    /// # Returns
    /// ParsedArgs
    /// 
    /// # Examples
    /// ``` rust
    /// use argparser::*;
    /// 
    /// fn main() {
    ///     let mut flags: Vec<Flag> = Vec::new();
    ///     flags.push(create_flag("a", "This is the a flag", vec!["some"]));
    ///     flags.push(create_flag("b", "This is the b flag", vec!["some", "thing"]));
    ///     flags.push(create_flag("c", "This is the c flag", vec!["some"]));
    ///     flags.push(create_flag("d", "This is the d flag", vec![]));
    ///
    ///     let mut args: Vec<Argument> = Vec::new();
    ///     args.push(create_arg("foo", "This is the foo argument"));
    ///     args.push(create_arg("bar", "This is the bar argument"));
    ///
    ///     let arg_parser = create_parser("Test Parser", "Tests arguments", flags, args);
    /// 
    ///     let parsed_args: ParsedArgs = arg_parser.parse(&mut std::env::args());
    /// }
    /// ```
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
                    if parser_arg.title == arg_to_lower {
                        options.arguments.push(parser_arg.title.clone());
                        used_args.push(&parser_arg);
                        continue 'args;
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

/// Creates a Flag which contains a title, descriptions, and the options that it takes.
/// 
/// # Arguments
/// - title: &str
/// - description: &str
/// - options: Vec<&str>
/// 
/// # Returns
/// Flag
/// 
/// # Examples
/// ``` rust
/// use argparser::*;
/// 
/// fn main() {
///     let a_flag: Flag = create_flag("a", "This is the a flag", vec!["option"]);
/// }
/// ```
pub fn create_flag(title: &str, description: &str, options: Vec<&str>) -> Flag {
    let mut return_vec: Vec<String> = Vec::new();
    for option in options {
        return_vec.push(option.to_string());
    }
    Flag {
        title: title.to_string(),
        description: description.to_string(),
        options: return_vec,
    }
}

/// Creates an Arguemnt that contains a title and description.
/// 
/// # Arguments
/// - title: &str
/// - description: &str
/// 
/// # Returns
/// Argument
/// 
/// # Examples
/// ``` rust
/// use argparser::*;
/// 
/// fn main() {
///     let arg1: Argument = create_arg("argument", "This is an argument");
/// }
/// ```
pub fn create_arg(title: &str, description: &str) -> Argument {
    Argument { title: title.to_string(), description: description.to_string() }
}

/// Creates the Parser which contains the custom help function and the
/// parse function.
/// 
/// # Arguments
/// - project_title: &str
/// - project_description: &str
/// - flags: Vec<Flag>
/// - arguments: Vec<Argument>
/// 
/// # Returns
/// Parser
/// 
/// # Examples
/// ``` rust
/// use argparser::*;
/// 
/// fn main() {
///     let mut flags: Vec<Flag> = Vec::new();
///     flags.push(create_flag("a", "This is the a flag", vec!["some"]));
///     flags.push(create_flag("b", "This is the b flag", vec!["some", "thing"]));
///     flags.push(create_flag("c", "This is the c flag", vec!["some"]));
///     flags.push(create_flag("d", "This is the d flag", vec![]));
///
///     let mut args: Vec<Argument> = Vec::new();
///     args.push(create_arg("foo", "This is the foo argument"));
///     args.push(create_arg("bar", "This is the bar argument"));
///
///     let arg_parser = create_parser("Test Parser", "Tests arguments", flags, args);
/// }
/// ```
pub fn create_parser(
    project_title: &str,
    project_description: &str,
    flags: Vec<Flag>,
    arguments: Vec<Argument> ) -> Parser {
        Parser {
            project_title: project_title.to_string(),
            project_description: project_description.to_string(),
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
        println!("{}", arg_parser.help());
        // assert_eq!(arg_parser.flags[2].title, "c");
        assert_eq!(true, false);
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
