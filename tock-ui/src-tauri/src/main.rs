#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use probe_rs::probe::list::Lister;
use serde::{Deserialize, Serialize};
use tockloader_lib::known_boards::known_board_names;
use tokio_serial::available_ports;
use tokio_serial::SerialPortType;

#[derive(Debug, Serialize, Deserialize)]
pub struct DebugProbeSummary {
    pub identifier: String,
    pub vendor_id: u16,
    pub product_id: u16,
    pub serial_number: Option<String>,
}

#[tauri::command]
fn get_known_board_names() -> Vec<String> {
    known_board_names()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SerialPortSummary {
    pub port_name: String,
    pub usb_vid: Option<u16>,
    pub usb_pid: Option<u16>,
    pub manufacturer: Option<String>,
    pub product: Option<String>,
    pub serial_number: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectedDevices {
    pub debug_probes: Vec<DebugProbeSummary>,
    pub serial_ports: Vec<SerialPortSummary>,
}

#[tauri::command]
async fn list_all_devices() -> Result<ConnectedDevices, String> {
    let probes = Lister::new().list_all();
    let debug_probe_summaries: Vec<DebugProbeSummary> = probes
        .into_iter()
        .map(|p| DebugProbeSummary {
            identifier: p.identifier,
            vendor_id: p.vendor_id,
            product_id: p.product_id,
            serial_number: p.serial_number,
        })
        .collect();

    let serial_ports = match available_ports() {
        Ok(ports) => ports,
        Err(e) => {
            eprintln!("Error listing serial ports: {e:?}");
            return Err(format!("Failed to list serial ports: {e}"));
        }
    };

    let serial_port_summaries: Vec<SerialPortSummary> = serial_ports
        .into_iter()
        .map(|p| {
            let mut usb_vid = None;
            let mut usb_pid = None;
            let mut manufacturer = None;
            let mut product = None;
            let mut serial_number = None;

            if let SerialPortType::UsbPort(usb_info) = p.port_type {
                usb_vid = Some(usb_info.vid);
                usb_pid = Some(usb_info.pid);
                manufacturer = usb_info.manufacturer;
                product = usb_info.product;
                serial_number = usb_info.serial_number;
            }

            SerialPortSummary {
                port_name: p.port_name,
                usb_vid,
                usb_pid,
                manufacturer,
                product,
                serial_number,
            }
        })
        .collect();

    Ok(ConnectedDevices {
        debug_probes: debug_probe_summaries,
        serial_ports: serial_port_summaries,
    })
}

fn main() {
    #[cfg(target_os = "linux")]
    {
        std::env::set_var("GDK_BACKEND", "x11");
        std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_known_board_names])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
