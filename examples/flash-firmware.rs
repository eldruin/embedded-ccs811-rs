use embedded_ccs811::{prelude::*, Ccs811Awake, Ccs811Device, SlaveAddr};
use linux_embedded_hal::{Delay, I2cdev};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut delay = Delay {};
    let address = SlaveAddr::default();
    let mut sensor = Ccs811Awake::new(dev, address);
    println!("Current status:");
    print_status(&mut sensor);

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!(
            "Invalid number of arguments.\n\
                The path to the firmware binary file must be provided for flashing.\n\
                Probably called 'CCS811_SW000246_1-00.bin'."
        );
    }
    let data = read_firmware(&PathBuf::from(&args[1]));
    println!("Read firmware file. Length: {} bytes", data.len());

    println!("Starting update process: Reset, erase, download, verify...");
    let result = sensor.update_application(&data, &mut delay);
    match result {
        Err(e) => println!("An error occurred: {:?}", e),
        Ok(_) => println!("Update was successful!"),
    }

    println!("Status:");
    print_status(&mut sensor);
}

fn print_status<E: core::fmt::Debug, DEV: Ccs811Device<Error = E>>(sensor: &mut DEV) {
    let hw_id = sensor.hardware_id().unwrap();
    let hw_ver = sensor.hardware_version().unwrap();
    let fw_boot_ver = sensor.firmware_bootloader_version().unwrap();
    let fw_app_ver = sensor.firmware_application_version().unwrap();
    let valid_app = sensor.has_valid_app().unwrap();

    println!("Hardware ID: {}, hardware version: {:?}", hw_id, hw_ver);
    println!("Firmware boot version: {:?}", fw_boot_ver);
    println!("Firmware application version: {:?}", fw_app_ver);
    println!("Has valid firmware application: {}", valid_app);
}

fn read_firmware(path: &PathBuf) -> Vec<u8> {
    let mut file = File::open(path).expect("Failed to open firmware file");
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data)
        .expect("Failed to read firmware file");
    data
}
