# Rust argparser

This is a crate that parses arguments that are passed into a rust program. Supports arguments/commands and flags which can take options. Comes with a custom help function that uses the `-h` flag.

## Features
Creating a new `Parser` with flags and arguments will create a custom help function on the parser.

Handles all the parsing logic using the rules stated below.

> [!note]
> You will need to handle the rest of the logic associated with the flags and arguments/commands (such as determining if an argument can be parsed as `i32`) as this is obviously program specific.

## Examples
``` rust
use argparser::*;
 
 fn main() {
     let mut flags: Vec<Flag> = Vec::new();
     flags.push(create_flag("a", "This is the a flag", vec!["some"]));
     flags.push(create_flag("b", "This is the b flag", vec!["some", "thing"]));
     flags.push(create_flag("c", "This is the c flag", vec!["some"]));
     flags.push(create_flag("d", "This is the d flag", vec![]));

     let mut args: Vec<Argument> = Vec::new();
     args.push(create_arg("foo", "This is the foo argument"));
     args.push(create_arg("bar", "This is the bar argument"));

     let arg_parser: Parser = create_parser("Test Parser", "Tests arguments", flags, args);
 }
 ```
 
 Calling `arg_parser.help()` will generate the following:
 
 ``` txt
 Test Parser, Tests arguments
 Usage: -h for help:

 Options:
    -a <some> :
	        This is the a flag
    -b <some> <thing> :
	        This is the b flag
    -c <some> :
     	 This is the c flag
    -d :
	        This is the d flag

 Arguments:
    foo :
     	 This is the foo argument
    bar :
     	 This is the bar argument
 ```

 Parsers need Flags and Arguments which are structs which define flags and arguments/commands that you would want to use in your program.

 ### Flag
 ``` rust
 pub struct Flag {
    title: String,
    description: String,
    options: Vec<String>,
}
```

 ### Argument
 ``` rust
 pub struct Argument {
    title: String,
    description: String,
}
```

Flags and Arguments both support the `New()` function which create an empty Flag or Argument to be further edited.

### Parser
``` rust
pub struct Parser {
    project_title: String,
    project_description: String,
    flags: Vec<Flag>,
    arguments: Vec<Argument>,
}
```
The Flags, Arguments, and Parser can be more easily created by using the `create_flag`, `create_argument`, and `create_parser` functions.

The parser takes the Flags and Arguments as Vectors and creates a parser. This parser contains the `help` and `parse` functions.

- `help` will call the custom help function that is created when the Parser is created.
- `parse` will take the arguments passed in, and attempt to parse them. If the arguments cannot be parsed (as defined below), the help message will be displayed, and the program will be terminated. Otherwise, the parsed arguments will be returned for further processing.

### The `parse` Function
The Parser's `parse` function takes the arguments that are passed in when calling the program.

``` rust
let parsed_args: ParsedArgs = arg_parser.parse(&mut std::env::args());
```

### ParsedArgs
ParsedArgs is the result of parsing the passed in arguments using the `parse` function on a Parser.
``` rust
pub struct ParsedArgs {
    pub flags: Vec<(String, String)>,
    pub arguments: Vec<String>,
}
```

In the vector of String tuples in the flags the first String is a Flag title, and the second String is the option that was passed with the flag. For example, passing the `a` flag with the option `3` into a program called program_name such as:

`program_name -a 3`

The vector of String tuples will contain the following:

`[("a", "3")]`

The purpose of ParsedArgs is to assist in further logic processing of the flags and arguments. It is possible that the second String in the tuple is empty, as flags are not required to take options. In this case, the second String will be `""`.

## Rules for the parser
- Flags and arguments/commands can only be used once.
- Flags that do take options can only take one option.
- Cannot pass in flags or arguments/commands that weren't created with the Parser.

The parser will display the help function and end the program if the arguments passes in do not follow the rules.

The following examples will not work:

- `program_name -a -a`
- `program_name -a option1 option2` (option2 will be treated like an argument/command, not an option for the `-a` flag)
- `program_name -b` (Only if the -b flag wasn't created with the parser) 
