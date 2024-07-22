# auditsetup_rust

## Overview

This Rust-based tool automates the setup process for auditing GitHub repositories. It streamlines the workflow by cloning the target repository, setting up the necessary environment, and preparing essential files for the audit process.

## Features

- Clone GitHub repositories
- Set up environment variables
- Create an audit directory structure
- Generate initial audit documentation files
- Scan and list public/external functions from smart contracts
- Open relevant files in Visual Studio Code

## Prerequisites

- Rust (latest stable version)
- Git
- Visual Studio Code

## Installation

1. Clone this repository:
```
git clone https://github.com/your-username/auditsetup_rust.git
cd auditsetup_rust
```

2. Build the project:
```
cargo build --release
```

3. The executable will be available in `target/release/auditsetup_rust`

## Making the Script Executable from Anywhere

To run the script with a single command from any directory, follow these steps:

1. Decide on a name for your command (e.g., `auditsetup`).

2. Create a symbolic link to the executable in a directory that's in your system's PATH. On most Unix-like systems (including macOS), you can use `/usr/local/bin`:
`sudo ln -s /path/to/your/target/release/auditsetup_rust/usr/local/bin/auditsetup_rust`

Replace `/path/to/your/target/release/auditsetup_rust` with the actual path to your built executable.

3. Make sure the original executable has the right permissions:
`chmod +x /path/to/your/target/release/auditsetup_rust`

4. Now you can run the tool from anywhere by typing:
`auditsetup_rust`

## Configuration

Complete the `config.toml` file in the root directory with your own configuration details.

## Usage

1. Run the executable:
```
./target/release/auditsetup_rust
```

1. When prompted, paste the GitHub repository URL of the project you want to audit then press ENTER.

2. If a `scope.txt` file doesn't exist, you'll be prompted to input the files in scope. Enter each filename on a new line, then PRESS ENTER *TWICE* to finish.

3. The tool will set up the audit environment and open relevant files in Visual Studio Code.

## Generated Structure

The tool creates the following structure in your specified base directory:

```
base_dir/
└── repository_name/
    ├── .env
    ├── scope.txt
    └── AUDIT/
        ├── NOTES.md
        ├── FINDINGS.md
        └── DIAGRAMS.excalidraw
```

## Features in Detail

1. **Repository Cloning**: Clones the specified GitHub repository to your local machine.

2. **Environment Setup**: Creates a `.env` file with necessary configuration details.

3. **Audit Directory**: Generates an `AUDIT` directory with initial `NOTES.md`, `FINDINGS.md`, and `DIAGRAMS.excalidraw` files.

4. **Scope Management**: Creates or updates a `scope.txt` file listing the files to be audited.

5. **Function Scanning**: Analyzes the smart contract files in scope and lists all public/external functions (excluding view/pure functions) in the `NOTES.md` file.

6. **VS Code Integration**: Opens the project and relevant files in Visual Studio Code for immediate start of the audit process.

# Ensure you have these VSCode extensions installed:

- https://marketplace.visualstudio.com/items?itemName=pomdtr.excalidraw-editor
- https://marketplace.visualstudio.com/items?itemName=vquelque.markfiles
- https://marketplace.visualstudio.com/items?itemName=tintinweb.solidity-visual-auditor