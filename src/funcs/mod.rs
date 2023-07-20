use crate::args::{ChangeBrightnessCommand, DeviceSelector};
use futures::{Stream, stream::BoxStream, StreamExt};
use brightness::{Brightness, BrightnessDevice};
use colored::*;
use futures::TryStreamExt;

const MAX_BRIGHTNESS: u32 = 100;
const MIN_BRIGHTNESS: u32 = 5;

pub async fn change_brightness(selector: &DeviceSelector, command: &ChangeBrightnessCommand, quiet: bool, percent: bool) -> Result<(), brightness::Error> {
    let devices = get_devices(selector);
    match command {
        ChangeBrightnessCommand::Get => {},
        ChangeBrightnessCommand::Set(percent) => set_brightness(devices, &percent.value).await?,
        ChangeBrightnessCommand::Inc(percent) => increase_brightness(devices, &percent.value).await?,
        ChangeBrightnessCommand::Dec(percent) => decrease_brightness(devices, &percent.value).await?,
        ChangeBrightnessCommand::Max => set_brightness(devices, &MAX_BRIGHTNESS).await?,
        ChangeBrightnessCommand::Min => set_brightness(devices, &MIN_BRIGHTNESS).await?,
    }
    print_brightness(get_devices(selector), quiet, percent).await
}

fn get_devices(selector: &DeviceSelector) -> BoxStream<Result<BrightnessDevice, brightness::Error>> {
    match selector {
        DeviceSelector::All => brightness::brightness_devices().boxed(),
        DeviceSelector::ByName(names) => brightness::brightness_devices()
            .try_filter(|device| async {
                device.device_name()
                .await
                .is_ok_and(|devname| names.iter().any(|name|devname == *name))
            }
        ).boxed()
    }
}

pub async fn set_brightness(devices: BoxStream<Result<BrightnessDevice, brightness::Error>>, percentage: &u32) -> Result<(), brightness::Error> {
    devices.try_for_each(|mut device| async move {
            let mut new_level: u32 = *percentage;
            if new_level < MIN_BRIGHTNESS {
                new_level = MIN_BRIGHTNESS
            } else if new_level > MAX_BRIGHTNESS{
                new_level = MAX_BRIGHTNESS
            }

            device.set(new_level).await
        }).await
}

pub async fn increase_brightness(devices: BoxStream<'_, Result<BrightnessDevice, brightness::Error>>, percentage: &u32) -> Result<(), brightness::Error>{
    devices.try_for_each(|mut device| async move {
        let mut new_level = device.get().await?.saturating_add(percentage);
        if new_level > MAX_BRIGHTNESS{
            new_level = MAX_BRIGHTNESS
        }

        device.set(new_level).await
    }).await
}

pub async fn decrease_brightness(devices: BoxStream<Result<BrightnessDevice, brightness::Error>>, percentage: &u32) -> Result<(), brightness::Error>{
    devices.try_for_each(|mut device| async move {
        let mut new_level = device.get().await?.saturating_sub(percentage);
        if new_level < MIN_BRIGHTNESS{
            new_level = MIN_BRIGHTNESS
        }

        device.set(new_level).await
    }).await?;
    Ok(())
}

pub async fn print_brightness(devices: BoxStream<Result<BrightnessDevice, brightness::Error>>, quiet: bool, percent: bool) -> Result<(), brightness::Error> {
    devices.try_for_each(|device| async move {
            let (name, brightness) = (device.device_name().await?, device.get().await?);
            if !quiet && !percent {
                if brightness >= MAX_BRIGHTNESS {
                    println!(
                        "{} brightness level reached ({})",
                        "Maximum".green().bold(),
                        "100%".green().bold()
                    );
                    return Ok(());
                }
                if brightness <= MIN_BRIGHTNESS {
                    println!(
                        "{} brightness level reached ({})",
                        "Minimum".red().bold(),
                        "5%".red().bold()
                    );
                    return Ok(());
                }
            }
            if quiet {
                return Ok(());
            }
            if percent {
                println!("{}", format!("{brightness}%").yellow().bold());
                return Ok(());
            }
            println!(
                "Brightness of device {} is {}",
                name.blue().bold(),
                format!("{brightness}%").yellow().bold()
            );
            Ok(())
        })
        .await;
    Ok(())
}
