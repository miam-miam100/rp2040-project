//! Converts characters from a serial connection (baud rate 8000) into morse code
//! and then plays it through the buzzer.
//! Uses GPIO18 for the buzzer.
#![no_std]
#![no_main]

use bsp::entry;
use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac, pwm,
    sio::Sio,
    watchdog::Watchdog,
    Timer,
};
use defmt::*;
use defmt_rtt as _;
use embedded_hal::PwmPin;
use panic_probe as _;
// Provide an alias for our BSP so we can switch targets quickly.
// Uncomment the BSP you included in Cargo.toml, the rest of the code does not need to change.
use rp_pico as bsp;
use usb_device::{class_prelude::*, prelude::*};
use usbd_serial::SerialPort;

use crate::morse::MorseCode;

mod morse;

// use sparkfun_pro_micro_rp2040 as bsp;

#[entry]
fn main() -> ! {
    info!("Program start");
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let usb_bus = UsbBusAllocator::new(bsp::hal::usb::UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));

    // Set up the USB Communications Class Device driver
    let mut serial = SerialPort::new(&usb_bus);

    // Create a USB device with a fake VID and PID
    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x54aa, 0xfa2d))
        .manufacturer("Miam Inc.")
        .product("Serial port")
        .serial_number("TEST")
        .device_class(2) // from: https://www.usb.org/defined-class-codes
        .build();

    // Init PWMs
    let mut pwm_slices = pwm::Slices::new(pac.PWM, &mut pac.RESETS);

    // Configure PWM1
    let pwm = &mut pwm_slices.pwm1;
    pwm.set_ph_correct();
    pwm.enable();

    let channel = &mut pwm.channel_a;
    channel.output_to(pins.gpio18);
    channel.set_duty(morse::TONE);
    channel.disable();

    let timer = Timer::new(pac.TIMER, &mut pac.RESETS);
    let mut said_hello = false;

    loop {
        if !said_hello && timer.get_counter().ticks() >= 2_000_000 {
            said_hello = true;
            let _ = serial.write(b"Welcome to tiny morse, please enter your text so it can be transformed into morse code!\r\n");
            info!("Sent serial welcome message");
        }
        // Check for new data
        if usb_dev.poll(&mut [&mut serial]) {
            let mut buf = [0u8; 64];
            match serial.read(&mut buf) {
                Err(_) | Ok(0) => {
                    // Do nothing
                }
                Ok(count) => {
                    let mut morse = MorseCode::new(&buf[..count], channel);
                    while let Some(character) = morse.get_char() {
                        info!("Playing tone for character: {}", character);
                        morse.consume_tone(&mut delay);
                        delay.delay_ms(3 * morse::UNIT)
                    }
                }
            }
        }
    }
}
