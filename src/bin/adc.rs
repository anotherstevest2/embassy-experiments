#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::adc::Adc;
use embassy_stm32::peripherals;
use embassy_stm32::time::Hertz;
use embassy_stm32::{adc, bind_interrupts, Config};
use embassy_time::{Delay, Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(pub struct Irqs {
    ADC1_2 => adc::InterruptHandler<peripherals::ADC1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut config = Config::default();
    // default is not sufficient for clocking the adc so we manually configure it
    {
        use embassy_stm32::rcc::*;
        config.rcc.hse = Some(Hertz::mhz(8));
        config.rcc.bypass_hse = true;
        config.rcc.sysclk = Some(Hertz::mhz(48));
        config.rcc.hclk = Some(Hertz::mhz(48));
        config.rcc.pclk1 = Some(Hertz::mhz(24));
        config.rcc.pclk2 = Some(Hertz::mhz(48));
        config.rcc.pll48 = true;
        config.rcc.adc = Some(AdcClockSource::Pll(Adcpres::DIV1));  // 48MHz -> 20.83333 ns
        config.rcc.adc34 = None;
    }
    let p = embassy_stm32::init(config);
    info!("Hello World!");

    debug!("create ADC...");
    let mut adc = Adc::new(p.ADC1, Irqs, &mut Delay);
    adc.sample_time_for_us(3);  // >= 2.2 us per 6.3.22 in STMicrosystems doc DS9118 Rev 14
    debug!("done");

    let mut temperature = adc.enable_temperature();
    let mut vrefint = adc.enable_vref(&mut Delay);

    fn convert_to_millivolts(sample: u16, vrefint_sample: u16) -> f64 {
        let vrefint_cal = 0x05f8; // nominal 1.23V ref factory saved reading at 3.3Vdda, manually read from 0x1ffff7ba on my discovery board
        let vdda_mv = f64::from(adc::VDDA_CALIB_MV) * f64::from(vrefint_cal) / f64::from(vrefint_sample) ;
        let mv_per_count = vdda_mv / f64::from(adc::ADC_MAX);

        f64::from(sample) * mv_per_count
    }

    fn convert_to_celcius(sample: u16, vrefint_sample: u16) -> f64 {
        // let ts_cal1 = 0x06ca; // 30degC factory saved reading at 3.3Vdda, manually read from 0x1ffff7b8 on my discovery board
        // let ts_cal2 = 0x0507; // 110degC factory saved reading at 3.3Vdda, manually read from 0x1ffff7c2 on my discovery board
        // It appears that the factory values are stored backwards *or* the datasheets documents them backwards
        let ts_cal1 =  0x0507;// (swapped) 30degC factory saved reading at 3.3Vdda, manually read from 0x1ffff7c2 on my discovery board
        let ts_cal2 = 0x06ca; // (swapped) 110degC factory saved reading at 3.3Vdda, manually read from 0x1ffff7b8 0x1ffff7c2 on my discovery board
        let vrefint_cal = 0x05f8; // nominal 1.23V ref factory saved reading at 3.3Vdda, manually read from 0x1ffff7ba on my discovery board
        let slope = (110 - 30) as f64 / (convert_to_millivolts(ts_cal2, vrefint_cal) - convert_to_millivolts(ts_cal1, vrefint_cal)); // degC/mV
        debug!("slope: {} degC/mv", slope);
        let intercept_30deg = convert_to_millivolts(ts_cal1, vrefint_cal);
        debug!("30deg point: {}", intercept_30deg);

        (convert_to_millivolts(sample, vrefint_sample) - intercept_30deg) * slope + 30.0
    }

    for _ in 0..10 {
    let vrefint_cal = 0x05f8; // nominal 1.23V ref factory saved reading at 3.3Vdda, manually read from 0x1ffff7ba on my discovery board
    let vrefint_sample = adc.read(&mut vrefint).await;
    let vdda_mv = f64::from(adc::VDDA_CALIB_MV) * f64::from(vrefint_cal) / f64::from(vrefint_sample) ;
    debug!("vdda_mv: {}", vdda_mv);
    Timer::after(Duration::from_millis(100)).await;
}

    loop {
        // Read pins
        // When loop first starts running, there is a significant drift in Vdda (from high down to just under 3V) so we bracket and average
        let vrefint_sample1 = adc.read(&mut vrefint).await;
        let t = adc.read(&mut temperature).await;
        let vrefint_sample2 = adc.read(&mut vrefint).await;
        let vrefint_sample = vrefint_sample1/2 + vrefint_sample2/2;
        
        debug!("temp sample: {}, vrefint_sample: {}, Degrees C: {}", t, vrefint_sample, convert_to_celcius(t, vrefint_sample));
        info!("Temperature: {} degrees C", convert_to_celcius(t, vrefint_sample));

        Timer::after(Duration::from_millis(100)).await;
    }
}