# bicepx

A simple CLI tool that helps you manage Bicep modules in monorepos, written in Rust.

## Installation

t.b.d.

## Usage

> [!IMPORTANT]
> The bicepx tool requires a local installation of [Azure CLI](https://learn.microsoft.com/en-us/azure/azure-resource-manager/bicep/install#azure-cli) and [Bicep](https://learn.microsoft.com/en-us/azure/azure-resource-manager/bicep/install).

```bash
bicepx [OPTIONS] <COMMAND>
```

## Commands

`build`

Builds Bicep modules based on your configuration.

```bash
bicepx build
```

`help`

Prints help information for bicepx or a specific subcommand.

```bash
bicepx help
bicepx help build
```

## Global Options

`-w, --working-dir <WORKING_DIR>`

Specifies the working directory for the operation.

- Default: `.` (current directory)
- Environment variable: `BICEPX_WORKING_DIR`

```bash
bicepx --working-dir ./infrastructure build
# or
export BICEPX_WORKING_DIR=./infrastructure
bicepx build
```

`-c, --config-file <CONFIG_FILE>`

Specifies the path to the configuration file.

- Default: `bicepx.toml`
- Environment variable: `BICEPX_CONFIG_FILE`

```bash
bicepx --config-file custom-config.toml build
# or
export BICEPX_CONFIG_FILE=custom-config.toml
bicepx build
```

`-h, --help`

Prints help information.

```bash
bicepx --help
```

## Configuration File

BicepX uses a TOML configuration file (default: `bicepx.toml`) to define build settings.

### Example Configuration

```toml
[modules]
include_pattern = "**/main.bicep"
```

`[modules]`

Configuration for Bicep module discovery and processing.

`include_pattern`

A glob pattern that specifies which Bicep files to include in the build.

- Type: String (glob pattern)
- Example: `"**/main.bicep"` - Matches all `main.bicep` files in any subdirectory

## Examples

### Basic Usage

Build all modules in the current directory using default settings:

```bash
bicepx build
```