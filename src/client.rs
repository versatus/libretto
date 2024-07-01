use conductor::{publisher::PubStream, subscriber::SubStream};
use notify::{
    Event,
    EventKind,
    event::{
        AccessKind, 
        AccessMode, 
        CreateKind, 
        ModifyKind, 
        RemoveKind, 
        DataChange, 
        RenameMode, 
        MetadataKind
    }
};

use serde::{Serialize, Deserialize};
use crate::pubsub::{FilesystemSubscriber, LibrettoPublisher, LibrettoTopic, LibrettoEvent, VmmAction};

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

pub struct LibrettoClient {
    subscriber: FilesystemSubscriber,
    publisher: LibrettoPublisher
}

impl LibrettoClient {
    pub async fn new(
        subscriber_uri: &str,
        publisher_uri: &str,
    ) -> std::io::Result<Self> {
        let subscriber = FilesystemSubscriber::new(subscriber_uri).await?;
        let publisher = LibrettoPublisher::new(publisher_uri).await?;
        Ok(Self { subscriber, publisher })
    }
    pub async fn run(
        mut self,
    ) -> std::io::Result<()> {
        loop {
            tokio::select! {
                Ok(messages) = self.subscriber.receive() => {
                    log::info!("Received Libretto Message");
                    for message in messages {
                        handle_events(message, &mut self.publisher).await;
                    }
                }
                _ = tokio::signal::ctrl_c() => {
                    break;
                }
            }
        }

        Ok(())
    }
}

pub async fn handle_events(event: Event, publisher: &mut LibrettoPublisher) {
    let kind = event.kind.clone();
    match kind {
        EventKind::Any => {}
        EventKind::Access(access) => {
            match access {
                AccessKind::Any => {
                    log::info!("AccessKind::Any: {:?}", event);
                }
                AccessKind::Read => {
                    log::info!("AccessKind::Read: {:?}", event);
                }
                AccessKind::Open(mode) => {
                    match mode {
                        AccessMode::Any => {
                            log::info!("AccessKind::Open(AccessMode::Any): {:?}", event);
                        }
                        AccessMode::Read => {
                            log::info!("AccessKind::Open(AccessMode::Read): {:?}", event);
                        }
                        AccessMode::Execute => {
                            log::info!("AccessKind::Open(AccessMode::Execute): {:?}", event);
                        }
                        AccessMode::Write => {
                            log::info!("AccessKind::Open(AccessMode::Write): {:?}", event);
                        }
                        AccessMode::Other => {
                            log::info!("AccessKind::Open(AccessMode::Other): {:?}", event);
                        }
                    }
                }
                AccessKind::Close(mode) => {
                    match mode {
                        AccessMode::Any => {
                            log::info!("AccessKind::Close(AccessMode::Any): {:?}", event);
                        }
                        AccessMode::Read => {
                            log::info!("AccessKind::Close(AccessMode::Read): {:?}", event);
                        }
                        AccessMode::Execute => {
                            log::info!("AccessKind::Close(AccessMode::Execute): {:?}", event);
                            if let Some(path) = event.clone().paths.get(0) {
                                if let Err(e) = notify_vmm(path.to_str(), publisher, event, VmmAction::Copy).await {
                                    log::info!("ERROR: attempting to notify nodes of AccessMode::Execute: {e}");
                                }
                            }
                        }
                        AccessMode::Write => {
                            log::info!("AccessKind::Close(AccessMode::Write): {:?}", event);
                            if let Some(path) = event.clone().paths.get(0) {
                                if let Err(e) = notify_vmm(path.to_str(), publisher, event, VmmAction::Copy).await {
                                    log::info!("ERROR: attempting to notify nodes of AccessMode::Write: {e}");
                                }
                            }
                        }
                        AccessMode::Other => {
                            log::info!("AccessKind::Close(AccessMode::Other): {:?}", event);
                        }
                    }
                }
                AccessKind::Other => {
                    log::info!("AccessKind::Other: {:?}", event);
                }
            }
        }
        EventKind::Create(create) => {
            match create {
                CreateKind::Any => {
                    log::info!("AccessKind::Create(CreateKind::Any): {:?}", event);
                    if let Some(path) = event.clone().paths.get(0) {
                        if let Err(e) = notify_vmm(path.to_str(), publisher, event, VmmAction::Copy).await {
                            log::info!("ERROR: attempting to notify nodes of CreateKind::Any: {e}");
                        }
                    }
                }
                CreateKind::File => {
                    log::info!("AccessKind::Create(CreateKind::File): {:?}", event);
                    if let Some(path) = event.clone().paths.get(0) {
                        if let Err(e) = notify_vmm(path.to_str(), publisher, event, VmmAction::Copy).await {
                            log::info!("ERROR: attempting to notify nodes of CreateKind::File: {e}");
                        }
                    }
                }
                CreateKind::Folder => {
                    log::info!("AccessKind::Create(CreateKind::Folder): {:?}", event);
                    if let Some(path) = event.clone().paths.get(0) {
                        if let Err(e) = notify_vmm(path.to_str(), publisher, event, VmmAction::Copy).await {
                            log::info!("ERROR: attempting to notify nodes of CreateKind::Folder: {e}");
                        }
                    }
                }
                CreateKind::Other => {
                    log::info!("AccessKind::Create(CreateKind::Other): {:?}", event);
                    if let Some(path) = event.clone().paths.get(0) {
                        if let Err(e) = notify_vmm(path.to_str(), publisher, event, VmmAction::Copy).await {
                            log::info!("ERROR: attempting to notify nodes of CreateKind::Other: {e}");
                        }
                    }
                }
            }
        }
        EventKind::Modify(modify) => {
            match modify {
                ModifyKind::Any => {
                    log::info!("ModifyKind::Any: {:?}", event);
                    if let Some(path) = event.clone().paths.get(0) {
                        if let Err(e) = notify_vmm(path.to_str(), publisher, event, VmmAction::Copy).await {
                            log::info!("ERROR: attempting to notify nodes of ModifyKind::Any: {e}");
                        }
                    }
                }
                ModifyKind::Data(data_change) => {
                    match data_change {
                        DataChange::Any => {
                            log::info!("ModifyKind::Data(DataChange::Any): {:?}", event);
                            if let Some(path) = event.clone().paths.get(0) {
                                if let Err(e) = notify_vmm(path.to_str(), publisher, event, VmmAction::Copy).await {
                                    log::info!("ERROR: attempting to notify nodes of ModifyKind::Data(DataChange::Any): {e}");
                                }
                            }
                        }
                        DataChange::Size => {
                            log::info!("ModifyKind::Data(DataChange::Size): {:?}", event);
                            if let Some(path) = event.clone().paths.get(0) {
                                if let Err(e) = notify_vmm(path.to_str(), publisher, event, VmmAction::Copy).await {
                                    log::info!("ERROR: attempting to notify nodes of ModifyKind::Data(DataChange::Size): {e}");
                                }
                            }
                        }
                        DataChange::Content => {
                            log::info!("ModifyKind::Data(DataChange::Content): {:?}", event);
                            if let Some(path) = event.clone().paths.get(0) {
                                if let Err(e) = notify_vmm(path.to_str(), publisher, event, VmmAction::Copy).await {
                                    log::info!("ERROR: attempting to notify nodes of ModifyKind::Data(DataChange::Content): {e}");
                                }
                            }
                        }
                        DataChange::Other => {
                            log::info!("ModifyKind::Data(DataChange::Other): {:?}", event);
                            if let Some(path) = event.clone().paths.get(0) {
                                if let Err(e) = notify_vmm(path.to_str(), publisher, event, VmmAction::Copy).await {
                                    log::info!("ERROR: attempting to notify nodes of ModifyKind::Data(DataChange::Other): {e}");
                                }
                            }
                        }
                    }
                }
                ModifyKind::Name(rename) => {
                    match rename {
                        RenameMode::Any => {
                            log::info!("ModifyKind::Name(RenameMode::Any): {:?}", event);
                            if let Some(path) = event.clone().paths.get(0) {
                                if let Err(e) = notify_vmm(path.to_str(), publisher, event, VmmAction::Copy).await {
                                    log::info!("ERROR: attempting to notify nodes of ModifyKind::Name(RenameMode::Any): {e}");
                                }
                            }
                        }
                        RenameMode::To => {
                            log::info!("ModifyKind::Name(RenameMode::To): {:?}", event);
                            if let Some(path) = event.clone().paths.get(0) {
                                if let Err(e) = notify_vmm(path.to_str(), publisher, event, VmmAction::Copy).await {
                                    log::info!("ERROR: attempting to notify nodes of ModifyKind::Name(RenameMode::To): {e}");
                                }
                            }
                        }
                        RenameMode::From => {
                            log::info!("ModifyKind::Name(RenameMode::From): {:?}", event);
                            if let Some(path) = event.clone().paths.get(0) {
                                if let Err(e) = notify_vmm(path.to_str(), publisher, event, VmmAction::Copy).await {
                                    log::info!("ERROR: attempting to notify nodes of ModifyKind::Name(RenameMode::From): {e}");
                                }
                            }
                        }
                        RenameMode::Both => {
                            log::info!("ModifyKind::Name(RenameMode::Both): {:?}", event);
                            if let Some(path) = event.clone().paths.get(0) {
                                if let Err(e) = notify_vmm(path.to_str(), publisher, event, VmmAction::Copy).await {
                                    log::info!("ERROR: attempting to notify nodes of ModifyKind::Name(RenameMode::Both): {e}");
                                }
                            }
                        }
                        RenameMode::Other => {
                            log::info!("ModifyKind::Name(RenameMode::Other): {:?}", event);
                            if let Some(path) = event.clone().paths.get(0) {
                                if let Err(e) = notify_vmm(path.to_str(), publisher, event, VmmAction::Copy).await {
                                    log::info!("ERROR: attempting to notify nodes of ModifyKind::Name(RenameMode::Other): {e}");
                                }
                            }
                        }
                    }
                }
                ModifyKind::Metadata(metadata) => {
                    match metadata {
                        MetadataKind::Any => {
                            log::info!("ModifyKind::Metadata(MetadataKind::Any): {:?}", event);
                            if let Some(path) = event.clone().paths.get(0) {
                                if let Err(e) = notify_vmm(path.to_str(), publisher, event, VmmAction::Rollup).await {
                                    log::info!("ERROR: attempting to notify nodes of ModifyKind::Metadata(MetdataKind::Any): {e}");
                                }
                            }
                        }
                        MetadataKind::Other => {
                            log::info!("ModifyKind::Metadata(MetadataKind::Other): {:?}", event);
                            if let Some(path) = event.clone().paths.get(0) {
                                if let Err(e) = notify_vmm(path.to_str(), publisher, event, VmmAction::Rollup).await {
                                    log::info!("ERROR: attempting to notify nodes of ModifyKind::Metadata(MetdataKind::Other): {e}");
                                }
                            }
                        }
                        MetadataKind::Extended => {
                            log::info!("ModifyKind::Metadata(MetadataKind::Extended): {:?}", event);
                            if let Some(path) = event.clone().paths.get(0) {
                                if let Err(e) = notify_vmm(path.to_str(), publisher, event, VmmAction::Rollup).await {
                                    log::info!("ERROR: attempting to notify nodes of ModifyKind::Metadata(MetdataKind::Other): {e}");
                                }
                            }
                        }
                        MetadataKind::WriteTime => {
                            log::info!("ModifyKind::Metadata(MetadataKind::WriteTime): {:?}", event);
                            if let Some(path) = event.clone().paths.get(0) {
                                if let Err(e) = notify_vmm(path.to_str(), publisher, event, VmmAction::Rollup).await {
                                    log::info!("ERROR: attempting to notify nodes of ModifyKind::Metadata(MetdataKind::WriteTime): {e}");
                                }
                            }
                        }
                        MetadataKind::Ownership => {
                            log::info!("ModifyKind::Metadata(MetadataKind::Ownership): {:?}", event);
                            if let Some(path) = event.clone().paths.get(0) {
                                if let Err(e) = notify_vmm(path.to_str(), publisher, event, VmmAction::Copy).await {
                                    log::info!("ERROR: attempting to notify nodes of ModifyKind::Metadata(MetdataKind::Ownership): {e}");
                                }
                            }
                        }
                        MetadataKind::AccessTime => {
                            log::info!("ModifyKind::Metadata(MetadataKind::AccessTime): {:?}", event);
                            if let Some(path) = event.clone().paths.get(0) {
                                if let Err(e) = notify_vmm(path.to_str(), publisher, event, VmmAction::Rollup).await {
                                    log::info!("ERROR: attempting to notify nodes of ModifyKind::Metadata(MetdataKind::AccessTime): {e}");
                                }
                            }
                        }
                        MetadataKind::Permissions => {
                            log::info!("ModifyKind::Metadata(MetadataKind::Permissions): {:?}", event);
                            if let Some(path) = event.clone().paths.get(0) {
                                if let Err(e) = notify_vmm(path.to_str(), publisher, event, VmmAction::Copy).await {
                                    log::info!("ERROR: attempting to notify nodes of ModifyKind::Metadata(MetdataKind::Permissions): {e}");
                                }
                            }
                        }
                    }
                }
                ModifyKind::Other => {
                    log::info!("ModifyKind::Metadata(MetadataKind::Other): {:?}", event);
                    if let Some(path) = event.clone().paths.get(0) {
                        if let Err(e) = notify_vmm(path.to_str(), publisher, event, VmmAction::Copy).await {
                            log::info!("ERROR: attempting to notify nodes of ModifyKind::Metadata(MetdataKind::Other): {e}");
                        }
                    }
                }
            }
        }
        EventKind::Remove(remove) => {
            match remove {
                RemoveKind::Any => {
                    log::info!("RemoveKind::Any: {:?}", event);
                    if let Some(path) = event.clone().paths.get(0) {
                        if let Err(e) = notify_vmm(path.to_str(), publisher, event, VmmAction::Copy).await {
                            log::info!("ERROR: attempting to notify nodes of RemoveKind::Any: {e}");
                        }
                    }
                }
                RemoveKind::File => {
                    log::info!("RemoveKind::File: {:?}", event);
                    if let Some(path) = event.clone().paths.get(0) {
                        if let Err(e) = notify_vmm(path.to_str(), publisher, event, VmmAction::Copy).await {
                            log::info!("ERROR: attempting to notify nodes of RemoveKind::File: {e}");
                        }
                    }
                }
                RemoveKind::Folder => {
                    log::info!("RemoveKind::Folder: {:?}", event);
                    if let Some(path) = event.clone().paths.get(0) {
                        if let Err(e) = notify_vmm(path.to_str(), publisher, event, VmmAction::Copy).await {
                            log::info!("ERROR: attempting to notify nodes of RemoveKind::Folder: {e}");
                        }
                    }
                }
                RemoveKind::Other => {
                    log::info!("RemoveKind::Other: {:?}", event);
                    if let Some(path) = event.clone().paths.get(0) {
                        if let Err(e) = notify_vmm(path.to_str(), publisher, event, VmmAction::Copy).await {
                            log::info!("ERROR: attempting to notify nodes of RemoveKind::Other: {e}");
                        }
                    }
                }
            }
        }
        EventKind::Other => {
            log::info!("EventKind::Other: {:?}", event);
            if let Some(path) = event.clone().paths.get(0) {
                if let Err(e) = notify_vmm(path.to_str(), publisher, event, VmmAction::Snapshot).await {
                    log::info!("ERROR: attempting to notify nodes of EventKind::Other: {e}");
                }
            }
        }
    }
}

async fn notify_vmm(instance_name: Option<&str>, publisher: &mut LibrettoPublisher, event: Event, action: VmmAction) -> std::io::Result<()> {
    log::info!("received an event {:?}, inform vmm, time to copy {:?}", event, instance_name);

    let event = LibrettoEvent::new(
        event,
        action,
        instance_name.map(|s| s.to_string())
    );

    publisher.publish(LibrettoTopic, event).await?;

    Ok(())
}
