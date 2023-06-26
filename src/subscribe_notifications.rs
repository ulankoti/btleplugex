use btleplug::api::{Central, CharPropFlags, Manager as _, Peripheral, ScanFilter};
use btleplug::platform::Manager;
use futures::stream::StreamExt;
use log::{debug, error, info};
use rand::Rng;
use std::error::Error;
use std::time::Duration;
use tokio::time;
use uuid::Uuid;

/// Only devices whose name contains this string will be tried.
const PERIPHERAL_NAME_MATCH_FILTER: &str = "Zephyr Heartrate Sensor";
/// UUID of the characteristic for which we should subscribe to notifications.
const NOTIFY_CHARACTERISTIC_UUID: Uuid = Uuid::from_u128(0x00002a37_0000_1000_8000_00805f9b34fb);

pub async fn subscribe() -> Result<(), Box<dyn Error>> {
    let manager = Manager::new().await?;
    let adapter_list = manager.adapters().await?;
    if adapter_list.is_empty() {
        error!("No Bluetooth adapters found");
        return Err("No Bluetooth Adapters found".into());
    }

    if let Some(adapter) = adapter_list.iter().nth(0) {
        info!("Starting scan on {}...", adapter.adapter_info().await?);
        adapter
            .start_scan(ScanFilter::default())
            .await
            .expect("failed to start scan");
        time::sleep(Duration::from_secs(30)).await;

        adapter.stop_scan().await?;

        let peripherals = adapter.peripherals().await?;
        let count: usize = peripherals.iter().count();
        let mut idx: i32 = 1;

        info!("peripherals scanned : {:?}", count);

        for peripheral in peripherals.iter() {
            let properties = peripheral.properties().await?;
            let is_connected = peripheral.is_connected().await?;
            let local_name = properties
                .clone()
                .unwrap()
                .local_name
                .unwrap_or(String::from("---"));
            let addr = properties.unwrap().address;
            debug!(
                "{:?}) Addr: {:?} -> Name: {:?} ",
                idx, addr, local_name
            );
            idx += 1;
            if local_name.contains(PERIPHERAL_NAME_MATCH_FILTER) {
                info!("Found matching peripheral {:?}...", &local_name);
                if !is_connected {
                    // Connect if we aren't already connected.
                    info!("Connecting to peripheral {:?}...", &local_name);
                    if let Err(err) = peripheral.connect().await {
                        error!("Failed to connect peripheral, skipping: {}", err);
                        continue;
                    }
                }
                let is_connected = peripheral.is_connected().await?;
                info!(
                    "Now connected ({:?}) to peripheral {:?}.",
                    is_connected, &local_name
                );
                if is_connected {
                    info!("Discover services {:?} ", local_name);
                    peripheral.discover_services().await?;
                    for service in peripheral.services() {
                        debug!(
                            "Service UUID {}, primary: {}",
                            service.uuid, service.primary
                        );
                        for characteristic in service.characteristics {
                            debug!("  {:?}", characteristic);

                            // Subscribe to notifications from the characteristic with the selected
                            // UUID.
                            if characteristic.uuid == NOTIFY_CHARACTERISTIC_UUID
                                && characteristic.properties.contains(CharPropFlags::NOTIFY)
                            {
                                info!("Subscribing to characteristic {:?}", characteristic.uuid);
                                peripheral.subscribe(&characteristic).await?;

                                let mut notification_stream = peripheral.notifications().await?;
                                let mut count: i32 = rand::thread_rng().gen_range(1..10);
                                debug!("receive {:?} number of notifications", count);
                                while count > 0 {
                                    match tokio::time::timeout(
                                        tokio::time::Duration::from_millis(1000),
                                        notification_stream.next(),
                                    )
                                    .await
                                    {
                                        Ok(Some(data)) => {
                                            info!(
                                                "Received data from {:?} [{:?}]: {:?}",
                                                local_name, data.uuid, data.value
                                            );
                                            count -= 1;
                                        }
                                        Ok(None) => {
                                            debug!("Received None, break loop");
                                            break;
                                        }
                                        Err(_) => {
                                            debug!("timed out");
                                        }
                                    }
                                }
                                drop(notification_stream);

                                peripheral.unsubscribe(&characteristic).await?;
                                info!("unsubscribed sucessfully");
                                break;
                            }
                        }
                    }
                    info!("Disconnecting from peripheral {:?}...", &local_name);
                    peripheral
                        .disconnect()
                        .await
                        .expect("Failed to disconnect peripheral");
                }
            } else {
                debug!("Skipping unknown peripheral {:?}", peripheral);
            }
        }
    }
    Ok(())
}
