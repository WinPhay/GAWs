# Command Line Usage

This guide provides comprehensive information on how to use the GAWs Generator from the command line. The GAWs Generator is a versatile tool that allows you to perform various operations based on the provided arguments.

## Basic Usage

The basic syntax for using the GAWs Generator is as follows:

```
gaws_generator [OPTIONS] --name <NAME>
```

The `--name` option is required for all operations.

## Available Options

The GAWs Generator supports the following command-line options:

| Option | Short | Description |
|--------|-------|-------------|
| `--name` | `-n` | Specifies the name (required) |
| `--language` | `-l` | Specifies the programming language (optional) |
| `--extension` | `-e` | Specifies the output file extension (optional) |
| `--action` | `-a` | Specifies the action to perform (optional) |

## Detailed Option Descriptions

### --name (-n)

The `--name` option is required and specifies the name to be used in the generator. This could be the name of your project, the output file, or any identifier relevant to your use case.

Example:
```
gaws_generator --name MyProject
```

### --language (-l)

The `--language` option allows you to specify the programming language for your project or operation. This is optional and can be used to customize the generator's behavior based on the language.

Example:
```
gaws_generator --name MyProject --language rust
```

### --extension (-e)

The `--extension` option lets you specify the file extension for the output. This is optional and can be used when generating files or templates.

Example:
```
gaws_generator --name MyProject --extension .rs
```

### --action (-a)

The `--action` option allows you to specify a particular action for the generator to perform. This is optional and can be used to trigger specific functionality within the generator.

Example:
```
gaws_generator --name MyProject --action build
```

## Usage Examples

1. Basic usage with only the required name:
   ```
   gaws_generator --name MyAwesomeProject
   ```

2. Specifying a language and file extension:
   ```
   gaws_generator --name MyRustProject --language rust --extension .rs
   ```

3. Performing a specific action:
   ```
   gaws_generator --name MyProject --action generate --language python
   ```

4. Using short option names:
   ```
   gaws_generator -n QuickProject -l java -e .java -a compile
   ```

## YAML Output

The GAWs Generator also produces YAML output based on the provided input. This output includes project details such as name, version, dependencies, and dev-dependencies. The YAML structure is automatically generated and printed to the console.

Example YAML output:
```yaml
name: Example Project
version: 1.0.0
dependencies:
  anyhow: "1.0"
  serde: "1.0"
  yaml-rust: "0.4"
dev-dependencies:
  clap: "2.33"
  serde: "1.0"
```

This YAML output can be useful for project configuration, dependency management, or further processing in your development workflow.

## Conclusion

The GAWs Generator command-line interface provides a flexible and powerful way to interact with the tool. By combining different options, you can customize its behavior to suit your specific needs. Remember to always provide the required `--name` option, and explore the various combinations of language, extension, and action options to maximize the utility of the GAWs Generator in your projects.