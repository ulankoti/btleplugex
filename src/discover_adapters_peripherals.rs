// See the "macOS permissions note" in README.md before running this on macOS
// Big Sur or later.

use std::error::Error;
use std::time::Duration;
use tokio::time;

use btleplug::api::{Central, Manager as _, Peripheral, ScanFilter};
use btleplug::platform::Manager;
use log::{debug, info, error};

pub async fn discover() -> Result<(), Box<dyn Error>> {

    let manager = Manager::new().await?;
    let adapter_list = manager.adapters().await?;
    if adapter_list.is_empty() {
        error!("No Bluetooth adapters found");
    }

    for adapter in adapter_list.iter() {
        info!("Starting scan on {}...", adapter.adapter_info().await?);
        adapter
            .start_scan(ScanFilter::default())
            .await
            .expect("Can't scan BLE adapter for connected devices...");
        time::sleep(Duration::from_secs(10)).await;
        let peripherals = adapter.peripherals().await?;
        if peripherals.is_empty() {
            error!("->>> BLE peripheral devices were not found, sorry. Exiting...");
        } else {
            // All peripheral devices in range
            info!("BLE peripherals found are: {:?}", peripherals.iter().count());
            for peripheral in peripherals.iter() {
                let properties = peripheral.properties().await?;
                let is_connected = peripheral.is_connected().await?;
                let local_name = properties.clone()
                    .unwrap()
                    .local_name
                    .unwrap_or(String::from("(peripheral name unknown)"));
                let addr = properties.unwrap().address;
                info!(
                    "Peripheral {:?} address {:?} is connected: {:?}",
                    local_name, addr, is_connected
                );
                if !is_connected {
                    info!("Connecting to peripheral {:?}...", &local_name);
                    if let Err(err) = peripheral.connect().await {
                        error!("Error connecting to peripheral, skipping: {:?}", err);
                        continue;
                    }
                }
                let is_connected = peripheral.is_connected().await?;
                info!(
                    "Now connected ({:?}) to peripheral {:?}...",
                    is_connected, &local_name
                );
                peripheral.discover_services().await?;
                info!("Discover peripheral {:?} services...", &local_name);
                for service in peripheral.services() {
                    debug!(
                        "Service UUID {}, primary: {}",
                        service.uuid, service.primary
                    );
                    for characteristic in service.characteristics {
                        debug!("  {:?}", characteristic);
                    }
                }
                if is_connected {
                    info!("Disconnecting from peripheral {:?}...", &local_name);
                    peripheral
                        .disconnect()
                        .await
                        .expect("Error disconnecting from BLE peripheral");
                }
            }
        }
    }
    Ok(())
}
