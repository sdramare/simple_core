use spin::Mutex;
use uart_16550::SerialPort;

use crate::utils::Global;

pub static SERIAL1: Mutex<Global<SerialPort>> = Mutex::new(Global::uninit());

pub fn init_serial(port: u16) {
    let serial = unsafe { SerialPort::new(port) };
    SERIAL1.lock().set(serial);
}
