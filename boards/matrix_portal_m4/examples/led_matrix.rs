#![no_std]
#![no_main]

use matrix_portal_m4 as bsp;

use bsp::{entry, hal, pac, Pins};
use embedded_graphics::{pixelcolor::Rgb565, prelude::*, primitives::Rectangle};
use embedded_hal::pwm;
use hal::{clock::GenericClockController, delay::Delay, prelude::*};
use hub75::Hub75;

use pac::{CorePeripherals, Peripherals};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();

    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );

    let mut delay = Delay::new(core.SYST, &mut clocks);
    delay.delay_ms(400u16);

    let pins = Pins::new(peripherals.PORT).split();

    // Initialize matrix pins
    let gpio_pins = (
        pins.mtx.r1,
        pins.mtx.g1,
        pins.mtx.b1,
        pins.mtx.r2,
        pins.mtx.g2,
        pins.mtx.b2,
        pins.mtx.addra,
        pins.mtx.addrb,
        pins.mtx.addrc,
        pins.mtx.addrd,
        pins.mtx.addre,
        pins.mtx.clk,
        pins.mtx.lat,
        pins.mtx.oe,
    );

    let brightness_bits = 1;
    let mut matrix = Hub75::new(gpio_pins, brightness_bits);

    loop {
        matrix.clear();

        matrix.draw(
            Rectangle::new(Coord::new(0, 0), Coord::new(63, 63))
                .fill(Some(Rgb565::from((0xFF, 0x00, 0x00)))),
        );
        matrix.output(&mut delay);

        // delay.delay_ms(10u32);
    }
}
