use crate::args::ChangeBrightnessCommand;
use futures::stream::BoxStream;
use brightness::{Brightness, BrightnessDevice};
use colored::*;
use futures::TryStreamExt;

const MAX_BRIGHTNESS: u32 = 100;
const MIN_BRIGHTNESS: u32 = 5;

pub async fn change_brightness(command: &ChangeBrightnessCommand, quiet: bool, percent: bool) -> Result<(), brightness::Error> {
    match command {
        ChangeBrightnessCommand::Get => {},
        ChangeBrightnessCommand::Set(percent) => set_brightness(Box::pin(brightness::brightness_devices()), &percent.value).await?,
        ChangeBrightnessCommand::Inc(percent) => increase_brightness(Box::pin(brightness::brightness_devices()), &percent.value).await?,
        ChangeBrightnessCommand::Dec(percent) => decrease_brightness(Box::pin(brightness::brightness_devices()), &percent.value).await?,
        ChangeBrightnessCommand::Max => set_brightness(Box::pin(brightness::brightness_devices()), &MAX_BRIGHTNESS).await?,
        ChangeBrightnessCommand::Min => set_brightness(Box::pin(brightness::brightness_devices()), &MIN_BRIGHTNESS).await?,
    }
    print_brightness(Box::pin(brightness::brightness_devices()), quiet, percent).await
}

pub async fn set_brightness(devices: BoxStream<'_, Result<BrightnessDevice, brightness::Error>>, percentage: &u32) -> Result<(), brightness::Error> {
    brightness::brightness_devices()
        .try_for_each(|mut device| async move {
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
        let mut new_level = device.get().await? + percentage;
        if new_level > MAX_BRIGHTNESS{
            new_level = MAX_BRIGHTNESS
        }

        device.set(new_level).await
    }).await
}

pub async fn decrease_brightness(devices: BoxStream<'_, Result<BrightnessDevice, brightness::Error>>, percentage: &u32) -> Result<(), brightness::Error>{
    devices.try_for_each(|mut device| async move {
        let level = device.get().await?;
        if level - percentage < MIN_BRIGHTNESS {
            device.set(level - percentage).await?;
        } else {
            device.set(MIN_BRIGHTNESS).await?;
        }
        Ok(())
    }).await?;
    Ok(())
}

pub async fn print_brightness(devices: BoxStream<'_, Result<BrightnessDevice, brightness::Error>>, quiet: bool, percent: bool) -> Result<(), brightness::Error> {
    devices.try_for_each(|device| async move {
            let (name, brightness) = (device.device_name().await?, device.get().await?);
            if !quiet && !percent {
                if brightness >= 100 {
                    println!(
                        "{} brightness level reached ({})",
                        "Maximum".green().bold(),
                        "100%".green().bold()
                    );
                    return Ok(());
                }
                if brightness <= 5 {
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
