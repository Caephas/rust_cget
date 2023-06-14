# Cget

`Cget` is a simplified `wget` clone written in Rust, aimed at downloading files from a given URL and saving them locally. It uses asynchronous programming model powered by `tokio`, allowing for efficient handling of I/O-bound tasks.

## Features

1. Downloading files from a given URL
2. Support for resuming incomplete downloads
3. Progress bar showing download progress and estimated time of completion
4. Saving downloaded files with the correct file name and extension, if provided in the URL

## Usage

```sh
cargo run <url>
```

Replace `<url>` with the URL of the file you want to download.

## Dependencies

The program uses several libraries:

- `clap` for command-line argument parsing
- `tokio` for async programming
- `reqwest` for making HTTP requests
- `url` for URL parsing
- `indicatif` for progress bar handling
- `dirs` for handling file paths

## Code Overview

The entry point of the program is the `main` function. This function uses `clap` to parse the command-line arguments and expects a URL. The URL is then parsed and passed to the `download` function.

The `download` function first determines the name of the file based on the URL. If the URL does not include a filename, it defaults to `tmp.zip`.

The function then checks if the file already exists on disk and finds its size if it does. It uses this size to send a range request to the server, asking for only the part of the file that it doesn't have yet. This allows the program to continue incomplete downloads.

It sets up a progress bar using the total size of the file (the sum of the size of the part it already has and the size of the remaining part) and updates the progress bar as it downloads the remaining part of the file.

The downloaded data is written to the file in chunks as they are received, which allows the program to use memory efficiently, even when downloading large files.

The `?` operator is used throughout the program for error handling, which allows it to write linear code without lots of error handling boilerplate. Both the `main` and `download` functions return a `Result` - if everything executes successfully, they return `Ok(())`. If an error occurs, they return `Err(error)`.

Note: This program is a simplified example and does not include many features of full-fledged download utilities, such as handling redirects, retries, or parallel downloads.