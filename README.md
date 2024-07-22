# auditsetup_rust

## Overview

This Rust-based tool automates the setup process for auditing GitHub repositories. It streamlines the workflow by cloning the target repository, setting up the necessary environment, and preparing essential files for the audit process: https://x.com/nisedo_/status/1806303000509444097

## Features

- Clone GitHub repositories
- Set up environment variables
- Create an audit directory structure
- Generate NOTES.md, FINDINGS.md and DIAGRAMS.excalidraw template files
- Scan and list entry points from files in-scope
- Open files in-scope in Visual Studio Code and mark them 📌

## Prerequisites

- Rust (latest stable version)
- Git
- Visual Studio Code

### Ensure you have these VSCode extensions installed:

- https://marketplace.visualstudio.com/items?itemName=vquelque.markfiles
- https://marketplace.visualstudio.com/items?itemName=tintinweb.solidity-visual-auditor
- https://marketplace.visualstudio.com/items?itemName=pomdtr.excalidraw-editor (optional)

## Installation

1. Clone this repository:
```
git clone https://github.com/nisedo/auditsetup_rust.git && cd auditsetup_rust
```

2. Build the project:
```
cargo build
```

3. Complete the `config.toml` file in the root directory with your own configuration details.

4. Run the script:
```
cargo run
```

1. When prompted, paste the GitHub repository URL of the project you want to audit then press **ENTER**.

2. If a `scope.txt` file doesn't exist, you'll be prompted to input the files in scope. Copy/Paste the list of files in-scope, then PRESS ENTER **TWICE** to finish.

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
