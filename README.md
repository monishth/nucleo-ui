
# nucleo-ui

nucleo-ui is a simple interactive command-line interface (CLI) tool that provides fast and efficient fuzzy matching capabilities. Leveraging the power of the `nucleo` fuzzy matching library, nucleo offers a simple and intuitive text interface for searching and comparing strings with a degree of imprecision.
**Note**: This is an unoffical wrapper around nucleo.

## Features

- **Fast Fuzzy Matching**: Utilize the high-performance `nucleo` library for speedy searches.
- **Interactive CLI**: Straightforward commands and options for all your fuzzy matching needs.

## Installation

### From cargo

To install nucleo-ui from crates.io, ensure you have Rust and Cargo installed on your system. Follow these steps:

1. Install the tool:

  ```sh
  cargo install nucleo-ui
  ```

### From Source

To install nucleo-ui from source, ensure you have Rust and Cargo installed on your system. Follow these steps:

1. Clone the repository:

   ```sh
   git clone https://github.com/monishth/nucleo-ui.git
   ```

2. Navigate to the cloned directory:

   ```sh
   cd nucleo-ui
   ```

3. Build and install the tool:

   ```sh
   cargo install --path .
   ```

After installation, you can run `fuzzymatcher` from your command line.

## Usage

### Basic Command

With no arguments, nucleo-ui will list directories in the current path with min/max depth 1: 

```sh
nucleo 
```
[![asciicast](assets/noargs.gif)](https://asciinema.org/a/650969)

The output of nucleo-ui will be written to stdout.

### Options

- `-p, --path`: Specify path to search (default: '.')
- `-d, --directory`: Specify whether to only look for directories (default: false)
- `--min-depth <NUMBER>`: Specify the minimum depth to search (default: 1)
- `--max-depth <NUMBER>`: Specify the maximum depth to search (default: 1)
- `-h, --help`: Display help information.

### Using the finder

- `CTRL-C` / `ESC`: Quit
- `Enter`: Select the current item
- `Up` / `Down`: Move cursor up and down


## Examples

List paths in the home directory with min depth 2 and max depth 4:

```sh
nucleo -d ~/ --min-depth 2 --max-depth 4
```
### stdin

nucleo-ui will also read from STDIN
This will load the finder with the output of ls

```sh
ls | nucleo
```
[![asciicast](assets/stdin.gif)](https://asciinema.org/a/650972)

## Contributing

Contributions are welcome! If you're interested in improving nucleo-ui, see the steps below:

1. Fork the repository.
2. Create a new branch for your feature (`git checkout -b feature/AmazingFeature`).
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`).
4. Push to the branch (`git push origin feature/AmazingFeature`).
5. Open a pull request.

