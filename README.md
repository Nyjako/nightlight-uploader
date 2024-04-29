# Nightlight Uploader

Nightlight Uploader is a simple tool written in Rust to automate the process of uploading Dead by Daylight end game scoreboard screenshots to the Nightlight website. It eliminates the manual effort required to capture and upload screenshots, providing convenience for Linux users who cannot use the Windows-only desktop app.

## Features

- Takes screenshots of the Dead by Daylight end game scoreboard.
- Uploads the screenshots to the Nightlight website.
- Simple configuration using environment variables or directly in the code.

## Building

### Prerequisites

Before building Nightlight Uploader on linux, ensure you have the following dependencies installed:

- **libxcb**
- **libxrandr**
- **dbus**
- **OpenSSL**

### Building from Source

Nightlight Uploader is built using Cargo, the Rust package manager. Follow these steps to build and install the application:

1. Clone the repository:

    ```bash
    git clone https://github.com/Nyjako/nightlight-uploader.git
    ```

2. Navigate to the project directory:

    ```bash
    cd nightlight-uploader
    ```

3. Set `NIGHTLIGHT_API_KEY` varible with your Nightlight API key:
    - Export before starting app:
        ```bash
        export NIGHTLIGHT_API_KEY="your_api_key"
        ```
    
    - In the `.cargo/config.toml` file:
        ```toml
        [env]
        NIGHTLIGHT_API_KEY = "your_api_key"
        ```
    
    - directly in the code:
        ```rust
        const API_KEY: &str = "your api key";
        ```

4. Build the project:

    ```bash
    cargo build --release
    ```
    
5. Run the application:

    ```bash
    cargo run --release
    ```

## Usage

1. Press the configured hotkey (default: CTRL + Numpad1) when the end game scoreboard is visible to take a screenshot.
2. Nightlight Uploader will automatically upload the screenshot to the Nightlight website.
3. To exit the application, press the exit hotkey (default: CTRL + Numpad2).
4. You need to verify your uploads on [NightLight](https://nightlight.gg/matches) website

## Configuration

### Hotkeys

Hotkeys for capturing screenshots and exiting the application can be customized by modifying the code:
```rust
let capture_hotkey = HotKey::new(Some(Modifiers::CONTROL), Code::Numpad1);
let exit_hotkey    = HotKey::new(Some(Modifiers::CONTROL), Code::Numpad2);
```

## Planned Features

- Automatic detection of the end game scoreboard to streamline the process further.

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request on GitHub.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.