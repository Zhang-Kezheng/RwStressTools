use std::net::IpAddr;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use local_ip_address::{list_afinet_netifas};
#[tauri::command]
fn network_interfaces() -> Result<Vec<(String, IpAddr)>, Error> {
    let network_interfaces=list_afinet_netifas()?;
    Ok(network_interfaces)
}
#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    LocalIpError(#[from] local_ip_address::Error),
}
impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_udp::init())
        .invoke_handler(tauri::generate_handler![network_interfaces])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
