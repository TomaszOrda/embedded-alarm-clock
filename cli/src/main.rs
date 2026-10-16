use core::time;
use std::{io::{self, BufRead, Write}, thread::sleep};
use serialport::{SerialPort, SerialPortInfo, SerialPortType::UsbPort};

fn main() {
    let stdin = io::stdin();
    println!("Looking for device. Connect and press reset.");
    let mut port = loop{
        if let Some(port_info) = get_usb_serial_port(){
            break serialport::new(port_info.port_name, 9600).open().unwrap();
        }
        print!(".");
        io::stdout().flush().unwrap();
        sleep(time::Duration::from_millis(500));
    };
    port.set_timeout(time::Duration::from_millis(500)).unwrap();
    println!("Device found! {}. Pairing.", port.name().unwrap());
    while !can_ping(&mut *port){
        print!(".");
        io::stdout().flush().unwrap();
        sleep(time::Duration::from_millis(500));
    }
    println!("Pairing established. Write help for list of available commands.");
    
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        port.write_all(format!("{line}\n").as_bytes()).unwrap();
        let mut reply = String::new();
        let mut buf = [0u8; 64];
        while let Ok(n) = port.read(&mut buf) {
            if n == 0 { break; }
            reply.push_str(&String::from_utf8_lossy(&buf[..n]));
        }
        println!("{reply}");
    }
}

fn get_usb_serial_port() -> Option<SerialPortInfo>{
    let ports = serialport::available_ports().expect("No ports found!");
    ports.iter().find(|&port| is_espressif_port(port)).cloned()
}

fn is_espressif_port(port: &SerialPortInfo) -> bool{
    match port.port_type{
        UsbPort(ref usb_port_info) => usb_port_info.vid == 0x303A,
        _ => false
    }
}

fn ping(port: &mut dyn SerialPort) -> Option<String>{
    if port.write_all(b"ping\n").is_err(){
        return None
    };
    let mut buf = [0u8; 64];
    let n = port.read(&mut buf).unwrap_or(0);
    if n==0{
        return None
    }
    return Some(String::from_utf8_lossy(&buf[..n]).into_owned())

}

fn can_ping(port: &mut dyn SerialPort) -> bool{
    return ping(port).is_some()
}