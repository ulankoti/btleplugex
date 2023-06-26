use btleplug::api::{Central, Manager as _, Peripheral, ScanFilter};
use btleplug::platform::Manager;
use log::{debug, error, info};
use std::error::Error;
use std::time::Duration;
use tokio::time;

pub async fn services_characteristics() -> Result<(), Box<dyn Error>> {
    let manager = Manager::new().await?;
    let adapter_list = manager.adapters().await?;
    if adapter_list.is_empty() {
        error!("No Bluetooth adapters found");
        return Err("No Bluetooth Adapters found".into());
    }

    if let Some(adapter) = adapter_list.iter().nth(0) {
        info!("Scan for 30 secs on {}...", adapter.adapter_info().await?);
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
                "{:?}) Addr: {:?} -> Name: {:?}",
                idx, addr, local_name
            );
            idx += 1;
            if !is_connected {
                info!("Connecting to peripheral {:?}...", &local_name);
                if let Err(err) = peripheral.connect().await {
                    error!("Failed to connect peripheral, skipping: {:?}", err);
                    continue;
                }
            }
            let is_connected = peripheral.is_connected().await?;
            info!(
                "Now connected ({:?}) to peripheral {:?}...",
                is_connected, &local_name
            );
            peripheral.discover_services().await?;
            info!("Discover services {:?} ", &local_name);
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
                    .expect("Failed to disconnect peripheral");
            }
        }
    }
    Ok(())
}
