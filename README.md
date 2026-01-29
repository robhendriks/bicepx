# BicepX

A CLI tool for managing Azure Bicep modules in monorepos.

> [!IMPORTANT]
> The bicepx tool requires a local installation of [Azure CLI](https://learn.microsoft.com/en-us/azure/azure-resource-manager/bicep/install#azure-cli) and [Bicep](https://learn.microsoft.com/en-us/azure/azure-resource-manager/bicep/install).

## Installation

Build from source:

```bash
git clone <repo-url>
cd bicepx
cargo build --release
```

## Usage

### Initialize a repository

Scan and index Bicep modules in your repository:

```bash
bicepx init
```

Options:

- `-f, --force` - Force re-initialization
- `-m, --module-glob <PATTERN>` - Module pattern (default: `**/main.bicep`)
- `-r, --root <PATH>` - Repository root (default: current directory)

### List modules

```bash
bicepx list module
```

Options:

- `-p, --pretty` - Pretty print output
- `-r, --root <PATH>` - Repository root

### Show module details

```bash
bicepx show <module>
```

### Generate documentation

```bash
bicepx docs <module>
```

## Configuration

Set the repository root via environment variable:

```bash
export BICEPX_ROOT=/path/to/your/repo
```

Or use the `-r, --root` flag with any command.

## Example Workflow

```bash
# Initialize your Bicep monorepo
cd /path/to/infrastructure
bicepx init

# List all modules
bicepx list module --pretty

# View module details
bicepx show storage-account

# Generate docs for a module
bicepx docs storage-account
```
