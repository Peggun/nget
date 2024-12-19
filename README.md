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
   cargo run -- -u [URL/S] -P [SAVE_DIR] -O [SAVE_FILE_NAME]
   ```

## Usage
To download a file, run the following command:
```bash
cargo run -- -u https://example.com/file.zip -P ./downloads/
```
- Replace `https://example.com/file.zip` with the URL of the file you want to download.
- Replace `./downloads/` with the directory where you want to save the file.

## Example Output
```bash
Saved to: test\test_again\index.html [========================================] 110.64 KiB/110.64 KiB (86.93 KiB/s) eta: (0s)
```

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
I would like to acknowledge and thank for use of all of the crates used in this project and all of the dependencies of the crates used.

Happy Downloading! 🚀
