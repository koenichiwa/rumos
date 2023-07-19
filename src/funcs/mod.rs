use crate::args::{Cli, SetArgs, Commands};
use brightness::Brightness;
use colored::*;
use futures::TryStreamExt;

const MAX_BRIGHTNESS: u32 = 100;
const MIN_BRIGHTNESS: u32 = 5;

pub async fn change_brightness(cli: Cli) -> Result<(), brightness::Error> {
    match cli.command {
        Commands::Get => {},
        Commands::Set(args) => set_brightness(brightness::brightness_devices(), args.percent).await?,
        Commands::Inc(_) => increase_brightness(brightness::brightness_devices(), args.percent).await?,
        Commands::Dec(_) => decrease_brightness(brightness::brightness_devices(), args.percent).await?,
        Commands::Max => set_brightness(brightness::brightness_devices(), MAX_BRIGHTNESS).await?,
        Commands::Min => set_brightness(brightness::brightness_devices(), MIN_BRIGHTNESS).await?,
    }
    print_brightness(brightness::brightness_devices(), cli)
}

pub async fn set_brightness(devices: dyn Stream<Item = Result<BrightnessDevice, Error>>, percentage: u32) -> Result<(), brightness::Error> {
    brightness::brightness_devices()
        .try_for_each(|mut device| async move {
            if percentage < 5 {
                device.set(5).await?;
            } else {
                device.set(percentage).await?;
            }
            Ok(())
        }).await?;
    Ok(())
}

pub async fn increase_brightness(devices: dyn Stream<Item = Result<BrightnessDevice, Error>>, percentage: u32) -> Result<(), brightness::Error>{
    devices.try_for_each(|device| async move {
        let level = device.get().await?;
        if level + percentage < MAX_BRIGHTNESS {
            dev.set(level + percentage).await?;
        } else {
            dev.set(MAX_BRIGHTNESS).await?;
        }
        Ok(())
    }).await?;
    Ok(())
}

pub async fn decrease_brightness(devices: dyn Stream<Item = Result<BrightnessDevice, Error>>, percentage: u32) -> Result<(), brightness::Error>{
    devices.try_for_each(|device| async move {
        let level = device.get().await?;
        if level - percentage < MIN_BRIGHTNESS {
            dev.set(level - percentage).await?;
        } else {
            dev.set(MIN_BRIGHTNESS).await?;
        }
        Ok(())
    }).await?;
    Ok(())
}

pub async fn print_brightness(devices: dyn Stream<Item = Result<BrightnessDevice, Error>>, cli: Cli) -> Result<(), brightness::Error> {
    let _ = brightness::brightness_devices()
        .try_for_each(|dev| async move {
            let (device, result) = (dev.device_name().await?, dev.get().await?);
            if !cli.quiet && !cli.percent {
                if result >= 100 {
                    println!(
                        "{} brightness level reached ({})",
                        "Maximum".green().bold(),
                        "100%".green().bold()
                    );
                    return Ok(());
                }
                if result <= 5 {
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
                println!("{}", format!("{result}%").yellow().bold());
                return Ok(());
            }
            println!(
                "Brightness of device {} is {}",
                device.blue().bold(),
                format!("{result}%").yellow().bold()
            );
            Ok(())
        })
        .await;
    Ok(())
}
