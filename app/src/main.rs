#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m_rt::entry;
use embedded_hal::digital::v2::OutputPin;
use stm32f3xx_hal_v2 as hal;
use hal::{pac, delay::Delay, prelude::*};

#[entry]
fn main() -> ! {
   let timeout = 20_u16;
   let dp = pac::Peripherals::take().unwrap();
   let cp = cortex_m::Peripherals::take().unwrap();

   let mut rcc = dp.RCC.constrain();
   let mut flash = dp.FLASH.constrain();
   let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);

   let mut led = gpiob.pb3.into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

   let clocks = rcc.cfgr.sysclk(8.mhz()).freeze(&mut flash.acr);

   let mut delay = Delay::new(cp.SYST, clocks);

   loop {
      led.set_high().ok();
      delay.delay_ms(timeout);
      led.set_low().ok();
      delay.delay_ms(1_000_u16);
   }
}
