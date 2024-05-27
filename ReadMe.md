# Rouge

**Rouge** is an HTTP proxy that logs client IP addresses. 

## Usage

1. **Clone the repository**:
    ```sh
    git clone https://github.com/yourusername/rouge.git
    cd rouge
    ```

2. **Build**
    ```sh
    cargo build
    ```

3. **Run the proxy server**
    ```sh
    cargo run {ip} {port}
    ```

The proxy will start listening to the specified IP address and port, displaying incoming traffic in the terminal. If the IP address and port entered cannot be proxied, an error will be returned to the terminal.

---

Rouge is intended for educational purposes **only**. The author is not liable for any misuse of this software. Use it responsibly and ensure compliance with all applicable laws and regulations.
