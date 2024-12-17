# nget

A modernized, multi-threaded download manager written in Rust. This is a modernized version of the well known tool wget. 

## Features
- Download files from a given URL.
- Display a progress bar to indicate download status.
- Automatically save the file to the specified directory.
- Support for multi-threaded downloads.
- Ability to resume downloads if download gets interrupted.
- HTTP/2 support.

### Planned Features
- Validation of file integrity (e.g., using checksums).
- Logging for completed downloads.
- Enhanced error handling and retries for failed downloads.
- and plenty more!

## Installation
### Prerequisites
- Rust programming language installed on your system. [Get Rust](https://www.rust-lang.org/tools/install)

### Steps
1. Clone this repository:
   ```bash
   git clone https://github.com/Peggun/nget.git
   cd nget
   ```
2. Build the project:
   ```bash
   cargo build --release
   ```
3. Run the tool:
   ```bash
   cargo run -- -u [URL] -o [SAVE_DIR]
   ```

## Usage
To download a file, run the following command:
```bash
cargo run -- -u https://example.com/file.zip -o ./downloads/
```
- Replace `https://example.com/file.zip` with the URL of the file you want to download.
- Replace `./downloads/` with the directory where you want to save the file.

## Example Output
```bash
Download complete. Saved to text/google.com [========================================] 0 B/0 B (0s)
```
This UI needs some fixing but it is ok for now.

## Contributing
Contributions are welcome! If you have suggestions for new features or find bugs, feel free to open an issue or create a pull request.

### How to contribute
1. Fork the repository.
2. Create a new branch for your feature or bugfix:
```bash
git checkout -b feature-name
```
3. Commit your changes:
```bash
git commit -m "Add feature-name"
```
4. Push to your branch:
```bash
git push origin feature-name
```
5. Open a pull request on GitHub

## License
This project is licensed under the GNU License. See the [LICENSE](https://github.com/Peggun/nget/blob/main/LICENSE) file for more details.

## Acknowledgments
[reqwest](https://docs.rs/reqwest/latest/reqwest/) for handling HTTP requests.<br>
[clap](https://docs.rs/clap/latest/clap/) for command-line argument parsing.<br>
[tokio](https://docs.rs/tokio/latest/tokio/) for asynchronous runtime and I/O operations.<br>
[thiserror](https://docs.rs/thiserror/latest/thiserror/) for custom error types.<br>
[url](https://docs.rs/url/latest/url/) for url parsing.<br>
[futures](https://docs.rs/futures/latest/futures/) for working with asynchronous computations.<br>
[futures-util](https://docs.rs/futures-util/latest/futures_util/) utilities for futures.<br>
[indicatif](https://docs.rs/indicatif/latest/indicatif/) for progress bars and spinner for the CLI.<br>
And all dependencies of those crates.

Happy Downloading! 🚀
