use crate::args::{Cli, SetArgs, Commands};
use futures::stream::BoxStream;
use brightness::{Brightness, BrightnessDevice};
use colored::*;
use futures::TryStreamExt;

const MAX_BRIGHTNESS: u32 = 100;
const MIN_BRIGHTNESS: u32 = 5;

pub async fn change_brightness(cli: Cli) -> Result<(), brightness::Error> {
    match cli.command {
        Commands::Get => {},
        Commands::Set(args) => set_brightness(Box::pin(brightness::brightness_devices()), args.percent).await?,
        Commands::Inc(args) => increase_brightness(Box::pin(brightness::brightness_devices()), args.percent).await?,
        Commands::Dec(args) => decrease_brightness(Box::pin(brightness::brightness_devices()), args.percent).await?,
        Commands::Max => set_brightness(Box::pin(brightness::brightness_devices()), MAX_BRIGHTNESS).await?,
        Commands::Min => set_brightness(Box::pin(brightness::brightness_devices()), MIN_BRIGHTNESS).await?,
    }
    print_brightness(&brightness::brightness_devices(), cli).await
}

pub async fn set_brightness(devices: BoxStream<Item = Result<BrightnessDevice, brightness::Error>>, percentage: u32) -> Result<(), brightness::Error> {
    brightness::brightness_devices()
        .try_for_each(|mut device| async move {
            if percentage < MIN_BRIGHTNESS {
                percentage = MIN_BRIGHTNESS
            } else if percentage > MAX_BRIGHTNESS{
                percentage = MAX_BRIGHTNESS
            }

            device.set(percentage).await
        }).await
}

pub async fn increase_brightness(devices: BoxStream<Item = Result<BrightnessDevice, brightness::Error>>, percentage: u32) -> Result<(), brightness::Error>{
    devices.try_for_each(|mut device| async move {
        let new_level = device.get().await? + percentage;
        if new_level > MAX_BRIGHTNESS{
            new_level = MAX_BRIGHTNESS
        }

        device.set(new_level).await
    }).await
}

pub async fn decrease_brightness(devices: BoxStream<Item = Result<BrightnessDevice, brightness::Error>>, percentage: u32) -> Result<(), brightness::Error>{
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

pub async fn print_brightness(devices: BoxStream<Item = Result<BrightnessDevice, brightness::Error>>, cli: Cli) -> Result<(), brightness::Error> {
    let _ = brightness::brightness_devices()
        .try_for_each(|device| async move {
            let (name, brightness) = (device.device_name().await?, device.get().await?);
            if !cli.quiet && !cli.percent {
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
            if cli.quiet {
                return Ok(());
            }
            if cli.percent {
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
