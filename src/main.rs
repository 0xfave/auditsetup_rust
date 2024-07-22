use std::env; // Import the 'env' module from the standard library to work with environment variables
use std::fs::{self, File}; // Import the 'fs' module and 'File' struct for file system operations
use std::io::{self, BufRead, Write}; // Import 'io' module and traits for input/output operations
use std::path::{Path, PathBuf}; // Import 'Path' and 'PathBuf' for working with file paths
use std::process::Command; // Import 'Command' to execute system commands
use regex::Regex; // Import 'Regex' for pattern matching in strings
use serde::Deserialize; // Import 'Deserialize' trait for deserializing data
use config::Config; // Import 'Config' for reading configuration files

#[derive(Debug, Deserialize)]
struct Settings { // Define a struct to hold the main configuration settings
    base_dir: String, // Field to store the base directory path
    env: EnvSettings, // Field to store environment-specific settings
    content: ContentSettings, // Field to store content-related settings
}

#[derive(Debug, Deserialize)]
struct EnvSettings { // Define a struct to hold environment-specific settings
    eth_rpc_url: String, // Field to store Ethereum RPC URL
    arbitrum_rpc: String, // Field to store Arbitrum RPC URL
    private_key: String, // Field to store private key
    etherscan_api_key: String, // Field to store Etherscan API key
    tenderly_dev_net_rpc_url: String, // Field to store Tenderly DevNet RPC URL
}

#[derive(Debug, Deserialize)]
struct ContentSettings { // Define a struct to hold content-related settings
    notes: String, // Field to store notes content
    findings: String, // Field to store findings content
}

fn load_config() -> Result<Settings, config::ConfigError> { // Function to load configuration from a file
    let config = Config::builder() // Create a new Config builder
        .add_source(config::File::with_name("config.toml")) // Add a TOML file named "config.toml" as a source
        .build()?; // Build the configuration, returning an error if it fails

    config.try_deserialize() // Attempt to deserialize the configuration into the Settings struct
}

fn main() -> io::Result<()> { // Main function that returns an IO Result
    let config = load_config().expect("Failed to load configuration"); // Load the configuration, panicking if it fails

    let mut input = String::new(); // Create a new empty string to store user input
    println!("Paste the GitHub repo URL: "); // Prompt the user to input a GitHub repository URL
    io::stdin().read_line(&mut input)?; // Read a line of input from the user
    let repo_url = input.trim(); // Remove whitespace from the input

    let repo_name = repo_url.split('/').last().unwrap().replace(".git", ""); // Extract the repository name from the URL
    let clone_path = PathBuf::from(&config.base_dir).join(&repo_name); // Create a path for cloning the repository

    if clone_path.exists() { // Check if the clone path already exists
        println!("Directory {:?} already exists. Please choose a different repository or remove the existing directory.", clone_path); // Print an error message
        return Ok(()); // Return early if the directory already exists
    }

    Command::new("git") // Create a new Command to execute git
        .args(&["clone", repo_url, clone_path.to_str().unwrap()]) // Set the arguments for git clone
        .status()?; // Execute the command and check its status

    println!("Repository cloned successfully to {:?}", clone_path); // Print a success message

    env::set_current_dir(&clone_path)?; // Change the current working directory to the cloned repository

    let env_content = format!( // Create the content for the .env file
        "ETH_RPC_URL={}\n\
         ARBITRUM_RPC={}\n\
         PRIVATE_KEY={}\n\
         ETHERSCAN_API_KEY={}\n\
         TENDERLY_DEV_NET_RPC_URL={}\n",
        config.env.eth_rpc_url,
        config.env.arbitrum_rpc,
        config.env.private_key,
        config.env.etherscan_api_key,
        config.env.tenderly_dev_net_rpc_url
    );
    fs::write(".env", env_content)?; // Write the .env file with the formatted content

    let audit_dir = clone_path.join("AUDIT"); // Create a path for the AUDIT directory
    fs::create_dir_all(&audit_dir)?; // Create the AUDIT directory and any necessary parent directories
    fs::write(audit_dir.join("NOTES.md"), &config.content.notes)?; // Write the NOTES.md file
    fs::write(audit_dir.join("FINDINGS.md"), &config.content.findings)?; // Write the FINDINGS.md file
    File::create(audit_dir.join("DIAGRAMS.excalidraw"))?; // Create an empty DIAGRAMS.excalidraw file

    let scope_file = clone_path.join("scope.txt"); // Create a path for the scope.txt file
    let mut scope_content = Vec::new(); // Create a vector to store the scope content

    if !scope_file.exists() { // Check if the scope file doesn't exist
        File::create(&scope_file)?; // Create the scope file
        println!("Paste the files in scope (then press ENTER twice):"); // Prompt the user to input scope files
        let stdin = io::stdin(); // Get a handle to the standard input
        for line in stdin.lock().lines() { // Iterate over lines of user input
            let line = line?; // Get the line, propagating any errors
            if line.is_empty() { // Check if the line is empty (user pressed ENTER twice)
                break; // Exit the loop if the line is empty
            }
            scope_content.push(line); // Add the line to the scope content
        }
        fs::write(&scope_file, scope_content.join("\n") + "\n")?; // Write the scope content to the file
    } else { // If the scope file already exists
        let content = fs::read_to_string(&scope_file)?; // Read the existing content
        scope_content = content.lines().map(|s| s.trim_start_matches("./").to_string()).collect(); // Process and collect the content
        fs::write(&scope_file, scope_content.join("\n") + "\n")?; // Write the processed content back to the file
    }

    let mut notes_file = fs::OpenOptions::new().append(true).open(audit_dir.join("NOTES.md"))?; // Open the NOTES.md file in append mode
    writeln!(notes_file, "\ncode {}", scope_content.join(" "))?; // Write the scope content to the notes file

    writeln!(notes_file, "\n\n# Public/External Functions (excluding view/pure)")?; // Write a header for public/external functions
    for file in &scope_content { // Iterate over each file in the scope
        let file_path = clone_path.join(file); // Create the full path to the file
        if file_path.exists() { // Check if the file exists
            let functions = scan_for_functions(&file_path)?; // Scan the file for functions
            if !functions.is_empty() { // If functions were found
                writeln!(notes_file, "\n## {}", file)?; // Write the file name as a subheader
                for func in functions { // Iterate over each found function
                    writeln!(notes_file, "- {}", func)?; // Write the function signature
                }
            }
        } else { // If the file doesn't exist
            println!("Warning: File {} not found.", file); // Print a warning
        }
    }

    Command::new("code").arg(".").status()?; // Open the current directory in VS Code
    Command::new("code").args(&scope_content).status()?; // Open the scope files in VS Code

    Ok(()) // Return Ok to indicate successful execution
}

fn scan_for_functions<P: AsRef<Path>>(file_path: P) -> io::Result<Vec<String>> {
    // Read the contents of the file specified by file_path into a string
    let content = fs::read_to_string(file_path)?;
    
    // Create a new Regex object to match function signatures
    // This regex looks for function declarations that are public or external, optionally including view or pure modifiers
    let re = Regex::new(r"\b(function\s+\w+\s*\([^)]*\)\s*(public|external)(\s+(view|pure))?)").unwrap();
    
    // Apply the regex to the file content, process the matches, and return the result
    Ok(re.captures_iter(&content)
        // Use filter_map to process each regex capture and keep only the desired matches
        .filter_map(|cap| {
            // Convert the full match (entire function signature) to a string
            let full_match = cap[0].to_string();
            // If the function is not view or pure, keep it; otherwise, discard it
            if !full_match.contains("view") && !full_match.contains("pure") {
                Some(full_match)
            } else {
                None
            }
        })
        // Collect the filtered results into a Vec<String>
        .collect())
}