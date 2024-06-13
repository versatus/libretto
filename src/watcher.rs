use lazy_static::lazy_static;
use notify::{Watcher, Event, RecursiveMode};
use tokio::sync::RwLock;
use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

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
    queue: Arc<RwLock<VecDeque<Event>>>,
    stop_recv: std::sync::mpsc::Receiver<()>
) -> std::io::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel::<Event>();

    let inner_watch_path = watch_path.to_string();
    let mut watcher = notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
        let tx = tx.clone();
        match res {
            Ok(event) => {
                let paths: Vec<PathBuf> = event.clone().paths.iter().map(|p| p.to_path_buf()).collect();
                for path in &paths {
                    if let Some(rel_path) = path.strip_prefix(&format!("{}/{}", inner_watch_path.clone(), "containers")).or_else(|_| {
                        path.strip_prefix(&format!("{}/{}", inner_watch_path.clone(), "virtual-machines"))
                    }).ok()
                    .and_then(|p| p.iter().skip(1).collect::<PathBuf>().to_str().map(|s| s.to_string())) {
                        if SYSTEM_PATHS.iter().any(|sp| rel_path.starts_with(sp)) {
                            continue;
                        } else {
                            let _ = tx.send(event.clone());
                        }
                    }
                }
            }
            Err(e) => println!("watch error: {:?}", e)
        }
    }).unwrap();


    watcher.watch(Path::new(&watch_path), RecursiveMode::Recursive).unwrap();

    let inner_queue = queue.clone();
    tokio::spawn(async move {
        loop {
            match rx.recv() {
                Ok(event) => {
                    let mut guard = inner_queue.write().await;
                    guard.push_back(event.clone());
                    drop(guard);
                }
                Err(e) => {
                    println!("Error: {e}");
                    break
                }
            }
        }
        println!("Channel closed");
    });



    loop {
        if let Ok(()) = stop_recv.recv() {
            break
        }
        std::thread::sleep(Duration::from_secs(60));
    }

    Ok(())
}
