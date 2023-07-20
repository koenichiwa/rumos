use std::fmt::format;

use crate::{MIN_BRIGHTNESS, MAX_BRIGHTNESS};
use futures::{Stream, stream::BoxStream, StreamExt};
use brightness::{Brightness, BrightnessDevice};
use colored::*;
use futures::TryStreamExt;

pub enum Command {
    BrightnessCommand{ command: BrightnessCommand, devices: Vec<String> }
}

pub enum BrightnessCommand {
    Get,
    Set{ percent: u32 },
    Inc{ percent: u32 },
    Dec{ percent: u32 },
    Max,
    Min,
}

impl Command {
    pub async fn handle(&self, quiet: bool, only_percent: bool) -> Result<(), brightness::Error> {
        match self {
            Command::BrightnessCommand { command, devices: device_names} => {
                let devices = get_devices(device_names);
                match command {
                    BrightnessCommand::Get => {},
                    BrightnessCommand::Set{percent} => set_brightness(devices, percent).await?,
                    BrightnessCommand::Inc{percent} => increase_brightness(devices, percent).await?,
                    BrightnessCommand::Dec{percent} => decrease_brightness(devices, percent).await?,
                    BrightnessCommand::Max => set_brightness(devices, &MAX_BRIGHTNESS).await?,
                    BrightnessCommand::Min => set_brightness(devices, &MIN_BRIGHTNESS).await?,
                }
                if(!quiet) {
                    print_brightness(get_devices(device_names), percent).await
                }
            }
        }
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
    
    async fn set_brightness(devices: BoxStream<Result<BrightnessDevice, brightness::Error>>, percentage: &u32) -> Result<(), brightness::Error> {
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
    
    async fn increase_brightness(devices: BoxStream<'_, Result<BrightnessDevice, brightness::Error>>, percentage: &u32) -> Result<(), brightness::Error>{
        devices.try_for_each(|mut device| async move {
            let mut new_level = device.get().await?.saturating_add(percentage);
            if new_level > MAX_BRIGHTNESS{
                new_level = MAX_BRIGHTNESS
            }
    
            device.set(new_level).await
        }).await
    }
    
    async fn decrease_brightness(devices: BoxStream<Result<BrightnessDevice, brightness::Error>>, percentage: &u32) -> Result<(), brightness::Error>{
        devices.try_for_each(|mut device| async move {
            let mut new_level = device.get().await?.saturating_sub(percentage);
            if new_level < MIN_BRIGHTNESS{
                new_level = MIN_BRIGHTNESS
            }
    
            device.set(new_level).await
        }).await?;
        Ok(())
    }
    
    async fn print_brightness(devices: BoxStream<Result<BrightnessDevice, brightness::Error>>, percent: bool) -> Result<(), brightness::Error> {
        devices.try_for_each(|device| async move {
                let (name, brightness) = (device.device_name().await?, device.get().await?);
                if percent {
                    println!("{}", format!("{brightness}%").yellow().bold());
                } else {
                    let name_str = format!("Brightness of device {}:" name.blue().bold());
                    let brightness_str = format!("{}%", brightness).bold();
                    if brightness >= MAX_BRIGHTNESS {
                        println!(
                            "{} {} {} brightness level reached",
                            name_str
                            brightness_str.green()
                            "Maximum".green().bold(),
                        )
                    } else if brightness <= MIN_BRIGHTNESS {
                        println!(
                            "{} {} {} brightness level reached",
                            name_str
                            brightness_str.green().red()
                            "Minimum".red().bold(),
                        )
                    } else {
                        println!(
                            "{} {}",
                            name_str,
                            brightness_str.yellow()
                        )
                    }
                }
                Ok(())
            })
            .await;
        Ok(())
    }
}
