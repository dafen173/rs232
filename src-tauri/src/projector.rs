use serialport::{SerialPort, SerialPortInfo};
use std::io::{self, Write};
use std::time::Duration;

pub struct OptomaProjector {
    port: Box<dyn SerialPort>,
}

impl OptomaProjector {
    pub fn new(port_name: &str) -> Result<Self, serialport::Error> {
        let port = serialport::new(port_name, 9600)
            .timeout(Duration::from_millis(1000))
            .data_bits(serialport::DataBits::Eight)
            .stop_bits(serialport::StopBits::One)
            .parity(serialport::Parity::None)
            .open()?;

        Ok(OptomaProjector { port })
    }

    pub fn send_command(&mut self, command: &str) -> io::Result<String> {
        // Optoma commands end with CR (\r)
        let full_command = format!("{}\r", command);
        self.port.write_all(full_command.as_bytes())?;

        let mut buf = vec![0; 100];
        let bytes_read = self.port.read(&mut buf)?;
        
        Ok(String::from_utf8_lossy(&buf[..bytes_read]).trim().to_string())
    }

    // Projector-specific commands
    pub fn power_on(&mut self) -> io::Result<String> {
        self.send_command("~00PON")
    }

    pub fn power_off(&mut self) -> io::Result<String> {
        self.send_command("~00POF")
    }

    pub fn input_hdmi1(&mut self) -> io::Result<String> {
        self.send_command("~00SIN01")
    }
}

pub fn list_ports() -> Vec<SerialPortInfo> {
    serialport::available_ports().unwrap_or_default()
}
