use crate::error::Error;
use crate::{MAX_BRIGHTNESS, MAX_CONCURRENCY, MIN_BRIGHTNESS};
use brightness::{Brightness, BrightnessDevice};
use colored::Colorize;
use futures::{future::join, future::ready, stream::BoxStream, StreamExt, TryStreamExt};
use std::{collections::HashSet, sync::Arc};

type BrightnessResult<T> = Result<T, brightness::Error>;

pub enum BrightnessOutput {
    Default,
    Percent,
    Quiet,
}

/// Represents various commands to be executed.
pub enum Command {
    BrightnessCommand {
        command: BrightnessCommand,
        selector: DeviceSelector,
        output: BrightnessOutput,
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
    ByIndex(HashSet<usize>),
}

impl Command {
    /// Handles the execution of a `Command`.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the command is executed successfully. Otherwise, returns an `rumos::Error`.
    pub async fn handle(&self) -> Result<(), Error> {
        match self {
            Command::BrightnessCommand {
                command,
                selector,
                output,
            } => {
                command
                    .handle(Self::stream_selected_devices(selector))
                    .await?;
                Self::print_device_brightnessess(Self::stream_selected_devices(selector), output)
                    .await;
            }
            Command::List => {
                println!("Available devices:");
                Self::print_device_names(Self::stream_selected_devices(&DeviceSelector::All)).await;
            }
        }
        Ok(())
    }

    /// Retrieves a stream of brightness devices based on the provided device selector.
    ///
    /// # Arguments
    ///
    /// * `selector`: The device selector specifying which devices to retrieve.
    ///
    /// # Returns
    ///
    /// Returns a stream of brightness devices wrapped in a `BoxStream`.
    fn stream_selected_devices(
        selector: &DeviceSelector,
    ) -> BoxStream<BrightnessResult<BrightnessDevice>> {
        async fn filter_by_name(
            device_names: Arc<HashSet<String>>,
            device: BrightnessResult<BrightnessDevice>,
        ) -> Option<BrightnessResult<BrightnessDevice>> {
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
        let stream = brightness::brightness_devices();
        match selector {
            DeviceSelector::All => stream.boxed(),
            DeviceSelector::ByName(device_names) => stream
                .filter_map(move |dev| {
                    let device_names = device_names.clone();
                    async move { filter_by_name(device_names, dev).await }
                })
                .boxed(),
            DeviceSelector::ByIndex(device_indices) => stream
                .enumerate()
                .filter_map(|(index, device)| {
                    if device.is_ok() && device_indices.contains(&index) {
                        ready(Some(device))
                    } else {
                        ready(None)
                    }
                })
                .boxed(),
        }
    }

    /// Prints the names of available brightness devices.
    async fn print_device_names(devices: BoxStream<'_, BrightnessResult<BrightnessDevice>>) {
        devices
            .map(|dev| async move {
                match dev {
                    Ok(dev) => dev.device_name().await.map_err(|err| Error::PrintError {
                        explanation: "Error while retrieving monitor name".to_string(),
                        source: err,
                    }),
                    Err(err) => Err(Error::PrintError {
                        explanation: "Error while retrieving monitor information".to_string(),
                        source: err,
                    }),
                }
            })
            .enumerate()
            .for_each(|(index, res)| async move {
                match res.await {
                    Ok(name) => println!("{}: {}", index, name.blue().bold()),
                    Err(Error::PrintError { explanation, .. }) => {
                        println!("{}: <{}>", index, explanation.red().bold());
                    }
                    Err(err) => println!(
                        "{}: Unknown error <{}>",
                        index,
                        err.to_string().red().bold()
                    ),
                }
            })
            .await;
    }

    /// Prints the brightness levels of selected devices, their index and their names.
    async fn print_device_brightnessess_default(
        devices: BoxStream<'_, BrightnessResult<BrightnessDevice>>,
    ) {
        devices
            .map(|dev| async move {
                match dev {
                    Ok(device) => match join(device.device_name(), device.get()).await {
                        (Ok(name), Ok(brightness)) => Ok((name, brightness)),
                        (Ok(name), Err(err)) => Err(Error::PrintError {
                            explanation: format!(
                                "Unable to retrieve brightness for device {}",
                                name.blue().bold()
                            ),
                            source: err,
                        }),
                        (Err(err), _) => Err(Error::PrintError {
                            explanation: "Unable to retrieve name for device".to_string(),
                            source: err,
                        }),
                    },
                    Err(err) => Err(Error::PrintError {
                        explanation: "Unable to retrieve information for device".to_string(),
                        source: err,
                    }),
                }
            })
            .enumerate()
            .for_each(move |(index, result)| async move {
                match result.await {
                    Ok((name, brightness)) => {
                        let name_str = format!("{}: {} brightness:", index, name.blue().bold());
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
                    Err(Error::PrintError { explanation, .. }) => {
                        println!("{}: Error {}", index, explanation.red().underline());
                    }
                    Err(err) => println!(
                        "{}: Unknown error <{}>",
                        index,
                        err.to_string().red().underline()
                    ),
                }
            })
            .await;
    }

    /// Prints only the brightness levels of selected devices.
    async fn print_device_brightnessess_percent(
        devices: BoxStream<'_, BrightnessResult<BrightnessDevice>>,
    ) {
        devices
            .map(|dev| async move {
                match dev {
                    Ok(device) => device.get().await.map_err(|err| Error::PrintError {
                        explanation: "Unable to retrieve brightness for device".to_string(),
                        source: err,
                    }),
                    Err(err) => Err(Error::PrintError {
                        explanation: "Unable to retrieve information for device".to_string(),
                        source: err,
                    }),
                }
            })
            .for_each(|result| async {
                match result.await {
                    Ok(percent) => println!("{}", format!("{percent}%").yellow().bold()),
                    Err(Error::PrintError { explanation, .. }) => {
                        println!("{}", explanation.red().underline());
                    }
                    Err(err) => println!("Unknown error: {}", err.to_string().red().underline()),
                }
            })
            .await;
    }

    /// Prints the brightness levels of selected devices.
    async fn print_device_brightnessess(
        devices: BoxStream<'_, BrightnessResult<BrightnessDevice>>,
        output: &BrightnessOutput,
    ) {
        match output {
            BrightnessOutput::Default => {
                Self::print_device_brightnessess_default(devices).await;
            }
            BrightnessOutput::Percent => {
                Self::print_device_brightnessess_percent(devices).await;
            }
            BrightnessOutput::Quiet => {}
        }
    }
}

impl BrightnessCommand {
    /// Handles the execution of the `BrightnessCommand` on a set of devices.
    ///
    /// # Arguments
    ///
    /// * `devices`: A stream of brightness devices on which the command will be executed.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the command is executed successfully. Otherwise, returns a `brightness::Error`.
    pub async fn handle(
        &self,
        devices: BoxStream<'_, BrightnessResult<BrightnessDevice>>,
    ) -> BrightnessResult<()> {
        match self {
            BrightnessCommand::Get => Ok(()),
            BrightnessCommand::Set { percent } => Self::set_brightness(devices, *percent).await,
            BrightnessCommand::Inc { percent } => {
                Self::increase_brightness(devices, *percent).await
            }
            BrightnessCommand::Dec { percent } => {
                Self::decrease_brightness(devices, *percent).await
            }
            BrightnessCommand::Max => Self::set_brightness(devices, MAX_BRIGHTNESS).await,
            BrightnessCommand::Min => Self::set_brightness(devices, MIN_BRIGHTNESS).await,
        }
    }

    /// Adjusts the brightness of a stream of devices based on the provided adjust function.
    ///
    /// # Arguments
    ///
    /// * `devices`: The stream of devices
    /// * `percentage`: The percentage used for the change
    /// * `adjust_fn`: A function that takes the current brightness value and the `percentage` and returns the new brightness value
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the command is executed successfully. Otherwise, returns a `brightness::Error`.
    async fn adjust_brightness<F>(
        devices: BoxStream<'_, BrightnessResult<BrightnessDevice>>,
        percentage: u32,
        adjust_fn: Arc<F>,
    ) -> BrightnessResult<()>
    where
        F: Fn(u32, u32) -> u32 + Send + Sync,
    {
        devices
            .try_for_each_concurrent(MAX_CONCURRENCY, |mut device| {
                let adjust_fn = adjust_fn.clone();
                async move {
                    let current_level = device.get().await?;
                    let new_level = adjust_fn(current_level, percentage);
                    let clamped_level = if new_level < MIN_BRIGHTNESS {
                        MIN_BRIGHTNESS
                    } else if new_level > MAX_BRIGHTNESS {
                        MAX_BRIGHTNESS
                    } else {
                        new_level
                    };
                    device.set(clamped_level).await
                }
            })
            .await
    }

    /// Sets the brightness of multiple devices to the given percentage.
    async fn set_brightness(
        devices: BoxStream<'_, BrightnessResult<BrightnessDevice>>,
        percentage: u32,
    ) -> BrightnessResult<()> {
        Self::adjust_brightness(devices, percentage, Arc::new(|_, p| p)).await
    }

    /// Increases the brightness of multiple devices by the given percentage.
    async fn increase_brightness(
        devices: BoxStream<'_, BrightnessResult<BrightnessDevice>>,
        percentage: u32,
    ) -> BrightnessResult<()> {
        Self::adjust_brightness(devices, percentage, Arc::new(u32::saturating_add)).await
    }

    /// Decreases the brightness of multiple devices by the given percentage.
    async fn decrease_brightness(
        devices: BoxStream<'_, BrightnessResult<BrightnessDevice>>,
        percentage: u32,
    ) -> BrightnessResult<()> {
        Self::adjust_brightness(devices, percentage, Arc::new(u32::saturating_sub)).await
    }
}
