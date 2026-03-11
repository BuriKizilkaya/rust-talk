// ============================================
// DEMO 5: Embedded / no_std — Embassy auf STM32
// ============================================
// Zeigt: no_std, async/await auf bare-metal, typsichere GPIO
// Echte Projekte: Linux-Kernel (seit 6.1!), Google Android (Binder),
//                 Microsoft Azure Sphere, Infineon (Automotive)
//
// Setup:
//   rustup target add thumbv7em-none-eabihf
//   cargo add embassy-stm32 embassy-executor embassy-time
//   cargo build --target thumbv7em-none-eabihf
//   probe-rs run --chip STM32F411RETx

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::{Duration, Timer};

// Echter async/await auf Mikrocontroller — kein FreeRTOS, kein Heap
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    // Typsicher: falscher Pin → Compile Error, nicht Runtime Error
    let mut led  = Output::new(p.PB7, Level::High, Speed::Low);
    let mut led2 = Output::new(p.PB14, Level::Low, Speed::Low);

    // Zwei unabhängige Tasks — kein pthread, kein RTOS-Task
    spawner.spawn(blink_fast(led2)).unwrap();

    loop {
        led.set_high();
        Timer::after(Duration::from_millis(500)).await;
        led.set_low();
        Timer::after(Duration::from_millis(500)).await;
    }
}

// Eigener async Task — läuft nebenläufig, kein Heap nötig
#[embassy_executor::task]
async fn blink_fast(mut led: Output<'static>) {
    loop {
        led.toggle();
        Timer::after(Duration::from_millis(100)).await;
    }
}
