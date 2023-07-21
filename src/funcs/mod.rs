use crate::{MAX_BRIGHTNESS, MIN_BRIGHTNESS};
use brightness::{Brightness, BrightnessDevice};
use colored::Colorize;
use futures::{stream::BoxStream, StreamExt, TryStreamExt, future::join};
use std::{collections::HashSet, sync::Arc};

/// Represents various commands to be executed.
pub enum Command {
    BrightnessCommand {
        command: BrightnessCommand,
        selector: DeviceSelector,
    },
    List,
}

/// Represents a command to be executed on a brightness device.
pub enum BrightnessCommand {
    Get,
    Set { percent: u32 },
    Inc { percent: u32 },
    Dec { percent: u32 },
    Max,
    Min,
}

/// Represents a device selector used to choose a set of brightness devices.
pub enum DeviceSelector {
    All,
    ByName(Arc<HashSet<String>>),
}

impl Command {
    /// Handles the execution of a `Command`.
    pub async fn handle(&self, quiet: bool, only_percent: bool) -> Result<(), brightness::Error> {
        match self {
            Command::BrightnessCommand { command, selector } => {
                command.handle(get_devices(selector)).await?;

                if quiet {
                    Ok(())
                } else {
                    print_brightness(get_devices(selector), only_percent).await;
                    Ok(())
                }
            }
            Command::List => {
                print_device_all_names().await;
                Ok(())
            }
        }
    }
}

impl BrightnessCommand {
    /// Handles the execution of the `BrightnessCommand` on a set of devices.
    pub async fn handle(&self, devices: BoxStream<'_, Result<BrightnessDevice, brightness::Error>>) -> Result<(), brightness::Error> {
        match self {
            BrightnessCommand::Get => Ok(()),
            BrightnessCommand::Set { percent } => set_brightness(devices, percent).await,
            BrightnessCommand::Inc { percent } => increase_brightness(devices, percent).await,
            BrightnessCommand::Dec { percent } => decrease_brightness(devices, percent).await,
            BrightnessCommand::Max => set_brightness(devices, &MAX_BRIGHTNESS).await,
            BrightnessCommand::Min => set_brightness(devices, &MIN_BRIGHTNESS).await,
        }
    }
}

/// Retrieves a stream of brightness devices based on the provided device selector.
fn get_devices(
    selector: &DeviceSelector,
) -> BoxStream<Result<BrightnessDevice, brightness::Error>> {
    async fn filter_by_name<E>(
        device_names: Arc<HashSet<String>>,
        device: Result<BrightnessDevice, E>,
    ) -> Option<Result<BrightnessDevice, E>> {
        if let Ok(device) = device {
            if device
                .device_name()
                .await
                .is_ok_and(|name| device_names.contains(&name))
            {
                return Some(Ok(device));
            }
        }
        None
    }

    match selector {
        DeviceSelector::All => brightness::brightness_devices().boxed(),
        DeviceSelector::ByName(device_names) => brightness::brightness_devices()
            .filter_map(move |dev| {
                let device_names = device_names.clone();
                async move { filter_by_name(device_names, dev).await }
            })
            .boxed(),
    }
}


/// Sets the brightness of a single brightness device to the given percentage
async fn adjust_single_brightness(
    mut device: BrightnessDevice,
    percentage: u32,
) -> Result<(), brightness::Error> {
    let new_level = if percentage < MIN_BRIGHTNESS {
        MIN_BRIGHTNESS
    } else if percentage > MAX_BRIGHTNESS {
        MAX_BRIGHTNESS
    } else {
        percentage
    };

    device.set(new_level).await
}

/// Sets the brightness of multiple devices to the given percentage.
async fn set_brightness(
    devices: BoxStream<'_, Result<BrightnessDevice, brightness::Error>>,
    percentage: &u32,
) -> Result<(), brightness::Error> {
    devices
        .try_for_each_concurrent(None, |device| async move { adjust_single_brightness(device, *percentage).await })
        .await
}

/// Increases the brightness of multiple devices by the given percentage.
async fn increase_brightness(
    devices: BoxStream<'_, Result<BrightnessDevice, brightness::Error>>,
    percentage: &u32,
) -> Result<(), brightness::Error> {
    devices
        .try_for_each_concurrent (None, |device| async move {
            let new_level = device.get().await?.saturating_add(*percentage);
            adjust_single_brightness(device, new_level).await
        })
        .await
}

/// Decreases the brightness of multiple devices by the given percentage.
async fn decrease_brightness(
    devices: BoxStream<'_, Result<BrightnessDevice, brightness::Error>>,
    percentage: &u32,
) -> Result<(), brightness::Error> {
    devices
        .try_for_each_concurrent(None, |device| async move {
            let new_level = device.get().await?.saturating_sub(*percentage);
            adjust_single_brightness(device, new_level).await
        })
        .await?;
    Ok(())
}

/// Prints the brightness levels of selected devices.
async fn print_brightness(
    devices: BoxStream<'_, Result<BrightnessDevice, brightness::Error>>,
    percent: bool,
) {
    println!("Device brightnessess");
    devices.filter_map(|dev| async move {
        match dev {
            Ok(device) => {
                if let (Ok(name), Ok(brightness)) = join(device.device_name(), device.get()).await {
                    Some((name, brightness))
                } else {
                    None
                }
            },
            Err(_) =>  None,
        }
    }).enumerate()
    .for_each(|(index, (name, brightness))| {
        if percent {
            println!("{}", format!("{brightness}%").yellow().bold());
        } else {
            let name_str = format!("{}: {} brightness:", index+1, name.blue().bold());
            let brightness_str = format!("{brightness}%").bold();
            if brightness >= MAX_BRIGHTNESS {
                println!(
                    "{} {} [{} brightness level reached]",
                    name_str,
                    brightness_str.green(),
                    "Maximum".green().bold(),
                );
            } else if brightness <= MIN_BRIGHTNESS {
                println!(
                    "{} {} [{} brightness level reached]",
                    name_str,
                    brightness_str.green().red(),
                    "Minimum".red().bold(),
                );
            } else {
                println!("{} {}", name_str, brightness_str.yellow(),);
            }
        }
        futures::future::ready(())
    }).await;
}

/// Prints the names of available brightness devices.
async fn print_device_all_names() {
    println!("Available devices");
    get_devices(&DeviceSelector::All)
        .map(|dev| async move {
            match dev {
                Ok(dev) => dev.device_name().await.map_or_else(
                    |err| format!("<Error while retrieving monitor name: {}>", err.to_string()).red().bold(),
                    |name| name.blue().bold(),
                ),
                Err(err) =>  format!("<Error while retrieving monitor information: {}>", err.to_string()).red().bold()
            }
        })
        .enumerate()
        .for_each(|(index, repr)| async move {
            println!("{}: {}", index + 1, repr.await.blue().bold());
        })
        .await;
}
