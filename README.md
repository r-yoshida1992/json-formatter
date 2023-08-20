# JSON Formatter and Minifier
This program provides a simple utility to format (prettify) or minify a JSON file. It is written in Rust and makes use of standard libraries for file operations and command-line argument parsing.

## Features:
1. Formatting JSON: Converts a compact JSON string into a human-readable format.
2. Minifying JSON: Removes unnecessary whitespaces from a formatted JSON string to make it compact.

## Usage:

```sh
./json-formatter [-m] <filename>
```

- If the -m flag is provided, the tool will minify the JSON file.
- Without the -m flag, the tool will format (prettify) the JSON file.

## Examples:
- To format a JSON file named sample.json:


```sh
./json-formatter sample.json
```

- To minify the same JSON file:

```sh
./json-formatter -m sample.json
```

