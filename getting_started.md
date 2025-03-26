# Getting Started with GAWs Generator

GAWs (GitHub Actions Workflows Generator) is a command-line tool that helps you generate GitHub Actions workflow files. This guide will walk you through the installation process, basic usage, and an overview of its capabilities.

## Installation

To install GAWs Generator, follow these steps:

1. Ensure you have Rust and Cargo installed on your system. If not, you can install them from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

2. Clone the GAWs Generator repository:

   ```bash
   git clone https://github.com/your-username/gaws-generator.git
   cd gaws-generator
   ```

3. Build the project using Cargo:

   ```bash
   cargo build --release
   ```

4. The compiled binary will be available in the `target/release` directory. You can add this directory to your PATH or move the binary to a location in your PATH for easier access.

## Basic Usage

GAWs Generator is a command-line tool that accepts various arguments to customize the generated workflow. Here's the basic syntax:

```bash
gaws-generator [OPTIONS] --name <NAME>
```

### Required Arguments

- `--name <NAME>` or `-n <NAME>`: Specifies the name of the workflow.

### Optional Arguments

- `--language <LANGUAGE>` or `-l <LANGUAGE>`: Specifies the programming language for the workflow.
- `--extension <EXTENSION>` or `-e <EXTENSION>`: Specifies the file extension for the output file.
- `--action <ACTION>` or `-a <ACTION>`: Specifies the GitHub Action to be used.

### Examples

1. Generate a basic workflow:

   ```bash
   gaws-generator --name "Basic CI"
   ```

2. Generate a workflow for a Python project:

   ```bash
   gaws-generator --name "Python CI" --language python --extension yml
   ```

3. Generate a workflow with a specific GitHub Action:

   ```bash
   gaws-generator --name "Node.js CI" --language javascript --action "actions/setup-node@v2"
   ```

## Capabilities Overview

GAWs Generator offers the following capabilities:

1. **Custom Workflow Names**: You can specify a custom name for your workflow using the `--name` option.

2. **Language-specific Workflows**: By using the `--language` option, you can generate workflows tailored to specific programming languages.

3. **File Extension Customization**: The `--extension` option allows you to specify the desired file extension for the generated workflow file.

4. **GitHub Action Integration**: You can include specific GitHub Actions in your workflow using the `--action` option.

5. **YAML Generation**: The tool generates valid YAML files for GitHub Actions workflows, handling the structure and syntax automatically.

6. **Dependency Management**: The generated workflows can include both runtime and development dependencies, making it easier to set up comprehensive CI/CD pipelines.

7. **Version Control**: The tool supports specifying versions for the project and its dependencies, ensuring reproducibility and consistency across different environments.

## Next Steps

Now that you're familiar with the basics of GAWs Generator, you can start creating custom workflows for your GitHub projects. Experiment with different combinations of options to generate workflows that suit your specific needs.

For more advanced usage and detailed information about each feature, please refer to the other sections of our documentation.