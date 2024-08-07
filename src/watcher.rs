use conductor::publisher::PubStream;
use lazy_static::lazy_static;
use notify::{Watcher, Event, RecursiveMode};
use std::{collections::VecDeque, path::{Path, PathBuf}};
use std::sync::{Arc, RwLock};

use crate::pubsub::{FilesystemPublisher, FilesystemTopic};

lazy_static! {
    static ref SYSTEM_PATHS: Vec<&'static str> = vec![
        "/var/lib/snapd", "/snap/", "/var/log/", "/var/run/utmp", "/var/run/wtmp", "/var/run/btmp",
        "/tmp/", "/var/tmp/", "/var/cache/", "/var/lib/apt/", "/var/lib/dpkg/", "/var/lib/systemd/",
        "/var/lib/dbus/", "/var/lib/NetworkManager/", "/var/lib/ucf/", "/var/lib/apt/lists/",
        "/var/lock/", "/var/lib/lock/", "/var/lib/rpm/", "/var/lib/pacman/", "/var/run/",
        "/run/", "/usr/bin/", "/usr/sbin/", "/usr/lib/", "/lib/", "/lib64/", "/sbin/", "/bin/",
        "/tmp/.X11-unix/", "/var/lib/lightdm/", "/var/lib/gdm3/", "/var/lib/sddm/", "/var/crash/",
        "/var/lib/AccountsService/", "/var/lib/alsa/", "/var/lib/bluetooth/", "/var/lib/colord/",
        "/var/lib/connman/", "/var/lib/console-setup/", "/var/lib/dhcp/", "/var/lib/dovecot/",
        "/var/lib/flatpak/", "/var/lib/fwupd/", "/var/lib/gdm3/", "/var/lib/hwclock/",
        "/var/lib/iio-sensor-proxy/", "/var/lib/initramfs-tools/", "/var/lib/initscripts/",
        "/var/lib/insserv/", "/var/lib/ipsec/", "/var/lib/iscsi/", "/var/lib/kubelet/",
        "/var/lib/libvirt/", "/var/lib/logrotate/", "/var/lib/machines/", "/var/lib/mdadm/",
        "/var/lib/misc/", "/var/lib/mlocate/", "/var/lib/NetworkManager/", "/var/lib/nginx/",
        "/var/lib/nodm/", "/var/lib/nss/", "/var/lib/nut/", "/var/lib/openvpn/", "/var/lib/pam/",
        "/var/lib/pciutils/", "/var/lib/plymouth/", "/var/lib/polkit-1/", "/var/lib/postgresql/",
        "/var/lib/pulse/", "/var/lib/rsyslog/", "/var/lib/samba/", "/var/lib/sddm/",
        "/var/lib/snapd/", "/var/lib/snmp/", "/var/lib/sssd/", "/var/lib/stratisd/", "/var/lib/sudo/",
        "/var/lib/systemd/", "/var/lib/tor/", "/var/lib/ucf/", "/var/lib/udisks2/",
        "/var/lib/unattended-upgrades/", "/var/lib/upower/", "/var/lib/usbutils/", "/var/lib/vmware/",
        "/var/lib/xdm/", "/var/lib/xkb/", "/etc/", "/boot/", "/proc/", "/sys/", "/dev/"
    ];
}


pub async fn monitor_directory(
    watch_path: &str,
    mut publisher: FilesystemPublisher,
) -> std::io::Result<()> {

    let event_queue = Arc::new(RwLock::new(VecDeque::new()));
    let watcher_queue = event_queue.clone();
    let mut watcher = notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
        let inner_queue = watcher_queue.clone();
        match res {
            Ok(event) => {
                log::info!("watcher discovered event: {:?}", event);
                let paths: Vec<PathBuf> = event.clone()
                    .paths.iter().map(|p| {
                        p.to_path_buf()
                    }).collect();

                log::info!("Paths changed: {:?}", paths);
                for path in &paths {
                    log::info!("Paths changed: {:?}", paths);
                    /*
                    let rel_path = if let Ok(rel_path) = path.strip_prefix(
                        &format!(
                            "{}/containers", inner_watch_path.clone()
                        )
                    ) {
                        rel_path
                    } else if let Ok(rel_path) = path.strip_prefix(
                        &format!(
                            "{}/virtual-machine",
                            inner_watch_path.clone()
                        )
                    ) {
                        rel_path
                    } else {
                        continue;
                    };
                    */

                    let rel_path = path.iter()
                        .skip(2)
                        .collect::<PathBuf>();

                    let rel_path_str = format!("/{}", rel_path.display());

                    log::info!("Change detected in rel_path: {}", &rel_path_str);

                    if SYSTEM_PATHS.iter().any(|sp| {
                        rel_path_str.starts_with(sp)
                    }) {
                        continue;
                    } 
                    
                    log::info!("Change detected in non-system path...");
                    if let Ok(mut guard) = inner_queue.write() {
                        guard.push_back(event.clone());
                        drop(guard);
                    }

                }
            }
            Err(e) => log::error!("watch error: {:?}", e)
        }
    }).unwrap();


    watcher.watch(
        Path::new(
            &watch_path
        ), 
        RecursiveMode::Recursive
    ).unwrap();

    let processor_queue = event_queue.clone();
    tokio::spawn(
        async move {
            let mut heartbeat_interval = tokio::time::interval(tokio::time::Duration::from_secs(20));
            loop {
                tokio::select! {
                    Ok(event) = process_queue(processor_queue.clone()) => {
                        publisher.publish(
                            FilesystemTopic,
                            event
                        ).await?;
                        log::info!("Succesfully published event...");
                    },
                    _heartbeat = heartbeat_interval.tick() => {
                        log::info!("Filesystem monitor still alive...");
                    }
                    _ = tokio::signal::ctrl_c() => {
                        break;
                    }
                }
            }

            Ok::<(), std::io::Error>(())
        }
    );



    loop {
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {
                break;
            }
        }
    }

    Ok(())
}

async fn process_queue(
    shared_queue: Arc<RwLock<VecDeque<Event>>>,
) -> std::io::Result<Event> {
    let res = shared_queue.write();
    let return_res = match res {
        Ok(mut guard) => {
            let event_res = guard.pop_front().ok_or(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Queue currently empty"
                )
            );
            drop(guard);
            event_res
        }
        Err(e) => {
            Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("unable to acquire lock on shared queue: {e}")
                )
            )
        }
    };

    return_res
}
