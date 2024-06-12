use std::{sync::Arc, collections::VecDeque, time::Duration};

use notify::{Event, EventKind, event::{AccessKind, AccessMode, CreateKind, ModifyKind, RemoveKind, DataChange, RenameMode, MetadataKind}};
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum VmmAction {
    Copy
}

#[derive(Serialize, Deserialize)]
pub struct LxdOperation {
    pub id: String,
    pub class: String,
    pub description: String,
    pub status: String,
    pub status_code: u32,
    pub resources: Option<LxdResources>,
}

#[derive(Serialize, Deserialize)]
pub struct LxdResources {
    pub containers: Option<Vec<String>>,
    pub instances: Option<Vec<String>>
}

pub async fn handle_events(queue: Arc<RwLock<VecDeque<Event>>>) {
    loop {
        let mut guard = queue.write().await;
        if let Some(event) = guard.pop_front() {
            drop(guard);

            let kind = event.kind.clone();

            match kind {
                EventKind::Any => {}
                EventKind::Access(access) => {
                    match access {
                        AccessKind::Any => {
                            println!("AccessKind::Any: {:?}", event);
                        }
                        AccessKind::Read => {
                            println!("AccessKind::Read: {:?}", event);
                        }
                        AccessKind::Open(mode) => {
                            match mode {
                                AccessMode::Any => {
                                    println!("AccessKind::Open(AccessMode::Any): {:?}", event);
                                }
                                AccessMode::Read => {
                                    println!("AccessKind::Open(AccessMode::Read): {:?}", event);
                                }
                                AccessMode::Execute => {
                                    println!("AccessKind::Open(AccessMode::Execute): {:?}", event);
                                }
                                AccessMode::Write => {
                                    println!("AccessKind::Open(AccessMode::Write): {:?}", event);
                                }
                                AccessMode::Other => {
                                    println!("AccessKind::Open(AccessMode::Other): {:?}", event);
                                }
                            }
                        }
                        AccessKind::Close(mode) => {
                            match mode {
                                AccessMode::Any => {
                                    println!("AccessKind::Close(AccessMode::Any): {:?}", event);
                                }
                                AccessMode::Read => {
                                    println!("AccessKind::Close(AccessMode::Read): {:?}", event);
                                }
                                AccessMode::Execute => {
                                    println!("AccessKind::Close(AccessMode::Execute): {:?}", event);
                                    if let Some(path) = event.clone().paths.get(0) {
                                        if let Err(e) = notify_vmm(path.to_str(), event, None).await {
                                            println!("ERROR: attempting to notify nodes of AccessMode::Execute: {e}");
                                        }
                                    }
                                }
                                AccessMode::Write => {
                                    println!("AccessKind::Close(AccessMode::Write): {:?}", event);
                                    if let Some(path) = event.clone().paths.get(0) {
                                        if let Err(e) = notify_vmm(path.to_str(), event, None).await {
                                            println!("ERROR: attempting to notify nodes of AccessMode::Write: {e}");
                                        }
                                    }
                                }
                                AccessMode::Other => {
                                    println!("AccessKind::Close(AccessMode::Other): {:?}", event);
                                }
                            }
                        }
                        AccessKind::Other => {
                            println!("AccessKind::Other: {:?}", event);
                        }
                    }
                }
                EventKind::Create(create) => {
                    match create {
                        CreateKind::Any => {
                            println!("AccessKind::Create(CreateKind::Any): {:?}", event);
                            if let Some(path) = event.clone().paths.get(0) {
                                if let Err(e) = notify_vmm(path.to_str(), event, None).await {
                                    println!("ERROR: attempting to notify nodes of CreateKind::Any: {e}");
                                }
                            }
                        }
                        CreateKind::File => {
                            println!("AccessKind::Create(CreateKind::File): {:?}", event);
                            if let Some(path) = event.clone().paths.get(0) {
                                if let Err(e) = notify_vmm(path.to_str(), event, None).await {
                                    println!("ERROR: attempting to notify nodes of CreateKind::File: {e}");
                                }
                            }
                        }
                        CreateKind::Folder => {
                            println!("AccessKind::Create(CreateKind::Folder): {:?}", event);
                            if let Some(path) = event.clone().paths.get(0) {
                                if let Err(e) = notify_vmm(path.to_str(), event, None).await {
                                    println!("ERROR: attempting to notify nodes of CreateKind::Folder: {e}");
                                }
                            }
                        }
                        CreateKind::Other => {
                            println!("AccessKind::Create(CreateKind::Other): {:?}", event);
                            if let Some(path) = event.clone().paths.get(0) {
                                if let Err(e) = notify_vmm(path.to_str(), event, None).await {
                                    println!("ERROR: attempting to notify nodes of CreateKind::Other: {e}");
                                }
                            }
                        }
                    }
                }
                EventKind::Modify(modify) => {
                    match modify {
                        ModifyKind::Any => {
                            println!("ModifyKind::Any: {:?}", event);
                            if let Some(path) = event.clone().paths.get(0) {
                                if let Err(e) = notify_vmm(path.to_str(), event, None).await {
                                    println!("ERROR: attempting to notify nodes of ModifyKind::Any: {e}");
                                }
                            }
                        }
                        ModifyKind::Data(data_change) => {
                            match data_change {
                                DataChange::Any => {
                                    println!("ModifyKind::Data(DataChange::Any): {:?}", event);
                                    if let Some(path) = event.clone().paths.get(0) {
                                        if let Err(e) = notify_vmm(path.to_str(), event, None).await {
                                            println!("ERROR: attempting to notify nodes of ModifyKind::Data(DataChange::Any): {e}");
                                        }
                                    }
                                }
                                DataChange::Size => {
                                    println!("ModifyKind::Data(DataChange::Size): {:?}", event);
                                    if let Some(path) = event.clone().paths.get(0) {
                                        if let Err(e) = notify_vmm(path.to_str(), event, None).await {
                                            println!("ERROR: attempting to notify nodes of ModifyKind::Data(DataChange::Size): {e}");
                                        }
                                    }
                                }
                                DataChange::Content => {
                                    println!("ModifyKind::Data(DataChange::Content): {:?}", event);
                                    if let Some(path) = event.clone().paths.get(0) {
                                        if let Err(e) = notify_vmm(path.to_str(), event, None).await {
                                            println!("ERROR: attempting to notify nodes of ModifyKind::Data(DataChange::Content): {e}");
                                        }
                                    }
                                }
                                DataChange::Other => {
                                    println!("ModifyKind::Data(DataChange::Other): {:?}", event);
                                    if let Some(path) = event.clone().paths.get(0) {
                                        if let Err(e) = notify_vmm(path.to_str(), event, None).await {
                                            println!("ERROR: attempting to notify nodes of ModifyKind::Data(DataChange::Other): {e}");
                                        }
                                    }
                                }
                            }
                        }
                        ModifyKind::Name(rename) => {
                            match rename {
                                RenameMode::Any => {
                                    println!("ModifyKind::Name(RenameMode::Any): {:?}", event);
                                    if let Some(path) = event.clone().paths.get(0) {
                                        if let Err(e) = notify_vmm(path.to_str(), event, None).await {
                                            println!("ERROR: attempting to notify nodes of ModifyKind::Name(RenameMode::Any): {e}");
                                        }
                                    }
                                }
                                RenameMode::To => {
                                    println!("ModifyKind::Name(RenameMode::To): {:?}", event);
                                    if let Some(path) = event.clone().paths.get(0) {
                                        if let Err(e) = notify_vmm(path.to_str(), event, None).await {
                                            println!("ERROR: attempting to notify nodes of ModifyKind::Name(RenameMode::To): {e}");
                                        }
                                    }
                                }
                                RenameMode::From => {
                                    println!("ModifyKind::Name(RenameMode::From): {:?}", event);
                                    if let Some(path) = event.clone().paths.get(0) {
                                        if let Err(e) = notify_vmm(path.to_str(), event, None).await {
                                            println!("ERROR: attempting to notify nodes of ModifyKind::Name(RenameMode::From): {e}");
                                        }
                                    }
                                }
                                RenameMode::Both => {
                                    println!("ModifyKind::Name(RenameMode::Both): {:?}", event);
                                    if let Some(path) = event.clone().paths.get(0) {
                                        if let Err(e) = notify_vmm(path.to_str(), event, None).await {
                                            println!("ERROR: attempting to notify nodes of ModifyKind::Name(RenameMode::Both): {e}");
                                        }
                                    }
                                }
                                RenameMode::Other => {
                                    println!("ModifyKind::Name(RenameMode::Other): {:?}", event);
                                    if let Some(path) = event.clone().paths.get(0) {
                                        if let Err(e) = notify_vmm(path.to_str(), event, None).await {
                                            println!("ERROR: attempting to notify nodes of ModifyKind::Name(RenameMode::Other): {e}");
                                        }
                                    }
                                }
                            }
                        }
                        ModifyKind::Metadata(metadata) => {
                            match metadata {
                                MetadataKind::Any => {
                                    println!("ModifyKind::Metadata(MetadataKind::Any): {:?}", event);
                                    if let Some(path) = event.clone().paths.get(0) {
                                        if let Err(e) = notify_vmm(path.to_str(), event, None).await {
                                            println!("ERROR: attempting to notify nodes of ModifyKind::Metadata(MetdataKind::Any): {e}");
                                        }
                                    }
                                }
                                MetadataKind::Other => {
                                    println!("ModifyKind::Metadata(MetadataKind::Other): {:?}", event);
                                    if let Some(path) = event.clone().paths.get(0) {
                                        if let Err(e) = notify_vmm(path.to_str(), event, None).await {
                                            println!("ERROR: attempting to notify nodes of ModifyKind::Metadata(MetdataKind::Other): {e}");
                                        }
                                    }
                                }
                                MetadataKind::Extended => {
                                    println!("ModifyKind::Metadata(MetadataKind::Extended): {:?}", event);
                                    if let Some(path) = event.clone().paths.get(0) {
                                        if let Err(e) = notify_vmm(path.to_str(), event, None).await {
                                            println!("ERROR: attempting to notify nodes of ModifyKind::Metadata(MetdataKind::Other): {e}");
                                        }
                                    }
                                }
                                MetadataKind::WriteTime => {
                                    println!("ModifyKind::Metadata(MetadataKind::WriteTime): {:?}", event);
                                    if let Some(path) = event.clone().paths.get(0) {
                                        if let Err(e) = notify_vmm(path.to_str(), event, None).await {
                                            println!("ERROR: attempting to notify nodes of ModifyKind::Metadata(MetdataKind::WriteTime): {e}");
                                        }
                                    }
                                }
                                MetadataKind::Ownership => {
                                    println!("ModifyKind::Metadata(MetadataKind::Ownership): {:?}", event);
                                    if let Some(path) = event.clone().paths.get(0) {
                                        if let Err(e) = notify_vmm(path.to_str(), event, None).await {
                                            println!("ERROR: attempting to notify nodes of ModifyKind::Metadata(MetdataKind::Ownership): {e}");
                                        }
                                    }
                                }
                                MetadataKind::AccessTime => {
                                    println!("ModifyKind::Metadata(MetadataKind::AccessTime): {:?}", event);
                                    if let Some(path) = event.clone().paths.get(0) {
                                        if let Err(e) = notify_vmm(path.to_str(), event, None).await {
                                            println!("ERROR: attempting to notify nodes of ModifyKind::Metadata(MetdataKind::AccessTime): {e}");
                                        }
                                    }
                                }
                                MetadataKind::Permissions => {
                                    println!("ModifyKind::Metadata(MetadataKind::Permissions): {:?}", event);
                                    if let Some(path) = event.clone().paths.get(0) {
                                        if let Err(e) = notify_vmm(path.to_str(), event, None).await {
                                            println!("ERROR: attempting to notify nodes of ModifyKind::Metadata(MetdataKind::Permissions): {e}");
                                        }
                                    }
                                }
                            }
                        }
                        ModifyKind::Other => {
                            println!("ModifyKind::Metadata(MetadataKind::Other): {:?}", event);
                            if let Some(path) = event.clone().paths.get(0) {
                                if let Err(e) = notify_vmm(path.to_str(), event, None).await {
                                    println!("ERROR: attempting to notify nodes of ModifyKind::Metadata(MetdataKind::Other): {e}");
                                }
                            }
                        }
                    }
                }
                EventKind::Remove(remove) => {
                    match remove {
                        RemoveKind::Any => {
                            println!("RemoveKind::Any: {:?}", event);
                            if let Some(path) = event.clone().paths.get(0) {
                                if let Err(e) = notify_vmm(path.to_str(), event, None).await {
                                    println!("ERROR: attempting to notify nodes of RemoveKind::Any: {e}");
                                }
                            }
                        }
                        RemoveKind::File => {
                            println!("RemoveKind::File: {:?}", event);
                            if let Some(path) = event.clone().paths.get(0) {
                                if let Err(e) = notify_vmm(path.to_str(), event, None).await {
                                    println!("ERROR: attempting to notify nodes of RemoveKind::File: {e}");
                                }
                            }
                        }
                        RemoveKind::Folder => {
                            println!("RemoveKind::Folder: {:?}", event);
                            if let Some(path) = event.clone().paths.get(0) {
                                if let Err(e) = notify_vmm(path.to_str(), event, None).await {
                                    println!("ERROR: attempting to notify nodes of RemoveKind::Folder: {e}");
                                }
                            }
                        }
                        RemoveKind::Other => {
                            println!("RemoveKind::Other: {:?}", event);
                            if let Some(path) = event.clone().paths.get(0) {
                                if let Err(e) = notify_vmm(path.to_str(), event, None).await {
                                    println!("ERROR: attempting to notify nodes of RemoveKind::Other: {e}");
                                }
                            }
                        }
                    }
                }
                EventKind::Other => {
                    println!("EventKind::Other: {:?}", event);
                    if let Some(path) = event.clone().paths.get(0) {
                        if let Err(e) = notify_vmm(path.to_str(), event, None).await {
                            println!("ERROR: attempting to notify nodes of EventKind::Other: {e}");
                        }
                    }
                }
            }
        } else {
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }
}

async fn notify_vmm(instance_name: Option<&str>, event: Event, _action: Option<VmmAction>) -> std::io::Result<()> {
    println!("received an event {:?}, inform vmm, time to copy {:?}", event, instance_name);
    Ok(())
}
