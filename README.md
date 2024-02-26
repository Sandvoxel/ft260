# FT260 Rust Project

This Rust project interacts with the FT260 USB-to-UART bridge chip using the `rusb` crate. It provides functionalities to enable UART communication, configure UART settings, receive data, and write data to the USB.

## Usage

To use this project, follow these steps:

1. Ensure you have Rust installed on your system.
2. Clone this repository to your local machine.
3. Include the `ft260` crate in your Rust project's dependencies.
4. Instantiate an `FT260` object using `FT260::new()`.
5. Use the various methods provided by the `FT260` struct to interact with the FT260 chip.

## Example

```rust
use ft260::FT260;

fn main() {
    // Create a new FT260 instance
    let mut ft260 = match FT260::new() {
        Some(ft260) => ft260,
        None => {
            println!("Failed to initialize FT260 device.");
            return;
        }
    };

    // Enable UART communication
    match ft260.enable_uart() {
        Ok(_) => println!("UART enabled successfully."),
        Err(e) => {
            println!("Error enabling UART: {}", e);
            return;
        }
    }

    // Configure UART settings
    match ft260.configure_uart(9600, 8, 0, 1, 0) {
        Ok(_) => println!("UART configured successfully."),
        Err(e) => {
            println!("Error configuring UART: {}", e);
            return;
        }
    }

    // Receive data from the FT260 chip
    match ft260.receive_data() {
        Ok(data) => println!("Received data: {:?}", data),
        Err(e) => println!("Error receiving data: {}", e),
    }

    // Write data to the FT260 chip
    let data_to_write = b"Hello, FT260!";
    match ft260.write_usb(data_to_write) {
        Ok(_) => println!("Data written successfully."),
        Err(e) => println!("Error writing data: {}", e),
    }
}
