# Cget

`Cget` is a command-line tool, a simplified clone of `wget` written in Rust. It's designed for downloading files from a given URL and saving them on your local machine. This tool also supports resuming incomplete downloads.

## Features

1. Downloading files from a given URL.
2. Resuming incomplete downloads.
3. Displaying a progress bar to track the download progress and estimated completion time.
4. Saving downloaded files with the correct file name and extension, if provided in the URL.

## Installation

Clone the repository to your local machine and navigate into the project directory. Build the project using `cargo`:

```sh
cargo build --release
```

## Usage

```sh
cargo run -- <url>
```

Replace `<url>` with the URL of the file you want to download. The file will be saved in your default downloads directory.

## Dependencies

The program uses several libraries:

- `clap` for command-line argument parsing.
- `tokio` for asynchronous programming.
- `reqwest` for making HTTP requests.
- `url` for URL parsing.
- `indicatif` for progress bar handling.
- `dirs` for handling file paths.

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.


## License

[MIT](https://choosealicense.com/licenses/mit/)

Please note: This is a simplified tool and does not include several features of full-fledged download utilities, such as handling redirects, retries, or parallel downloads.
