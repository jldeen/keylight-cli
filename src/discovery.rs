use mdns_sd::{ServiceDaemon, ServiceEvent};
use std::collections::HashSet;
use std::net::IpAddr;
use std::time::{Duration, Instant};

const SERVICE_TYPE: &str = "_elg._tcp.local.";

pub struct DiscoveredLight {
    pub name: String,
    pub ip: String,
    pub port: u16,
}

pub fn discover_lights(
    timeout: Duration,
) -> Result<Vec<DiscoveredLight>, Box<dyn std::error::Error>> {
    let mdns = ServiceDaemon::new()?;
    let receiver = mdns.browse(SERVICE_TYPE)?;

    let mut lights = Vec::new();
    let mut seen_names = HashSet::new();
    let start = Instant::now();
    let mut last_found = None::<Instant>;
    let grace_period = Duration::from_secs(5);
    let poll_interval = Duration::from_secs(1);

    while start.elapsed() < timeout {
        // After finding at least one light, stop if no new light found within grace period
        if let Some(last) = last_found {
            if last.elapsed() >= grace_period {
                log::info!("No new lights found in {}s, stopping discovery", grace_period.as_secs());
                break;
            }
        }

        let remaining = timeout.saturating_sub(start.elapsed());
        if remaining.is_zero() {
            break;
        }
        let wait = remaining.min(poll_interval);

        match receiver.recv_timeout(wait) {
            Ok(ServiceEvent::ServiceResolved(info)) => {
                let name = info.get_fullname().to_string();
                if seen_names.contains(&name) {
                    continue;
                }

                // Prefer IPv4 addresses over IPv6 for local network devices
                let ipv4 = info.get_addresses().iter().find(|a| matches!(a, IpAddr::V4(_)));
                let ip = match ipv4 {
                    Some(addr) => addr.to_string(),
                    None => match info.get_addresses().iter().next() {
                        Some(addr) => addr.to_string(),
                        None => continue,
                    },
                };

                log::info!(
                    "Discovered light: {} at {}:{}",
                    name,
                    ip,
                    info.get_port()
                );

                let display_name = name
                    .strip_suffix(SERVICE_TYPE)
                    .unwrap_or(&name)
                    .trim_end_matches('.')
                    .to_string();

                seen_names.insert(name);
                lights.push(DiscoveredLight {
                    name: display_name,
                    ip,
                    port: info.get_port(),
                });
                last_found = Some(Instant::now());
            }
            Ok(_) => {}
            Err(_) => {}
        }
    }

    mdns.shutdown()?;
    Ok(lights)
}
