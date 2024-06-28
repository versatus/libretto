use conductor::{publisher::PubStream, subscriber::SubStream}; 
use notify::Event;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tonic::async_trait;
use conductor::util::{try_get_topic_len, try_get_message_len, parse_next_message};
use conductor::{HEADER_SIZE, TOPIC_SIZE_OFFSET};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use derive_more::Display;
use serde::{Serialize, Deserialize};

#[derive(Display)]
pub struct FilesystemTopic;

#[derive(Display)]
pub struct LibrettoTopic;

pub struct FilesystemSubscriber {
    stream: TcpStream
}

impl FilesystemSubscriber {
    pub async fn new(uri: &str) -> std::io::Result<Self> {
        let mut stream = TcpStream::connect(uri).await?;
        let topics_str = FilesystemTopic.to_string();
        stream.write_all(topics_str.as_bytes()).await?;
        Ok(Self { stream })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LibrettoEvent {
    event: Event,
    action: VmmAction,
    instance_name: Option<String>,
}

impl LibrettoEvent {
    pub fn new(
        event: Event,
        action: VmmAction,
        instance_name: Option<String>
    ) -> Self {
        Self { event, action, instance_name }
    }

    pub fn event(&self) -> &Event {
        &self.event
    }

    pub fn action(&self) -> &VmmAction {
        &self.action
    }

    pub fn instance_name(&self) -> &Option<String> {
        &self.instance_name
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum VmmAction {
    Copy,
    Migrate,
    Snapshot,
    Rollup,
    Other(String)
}

#[async_trait]
impl SubStream for FilesystemSubscriber {
    type Message = Vec<Event>;

    async fn receive(&mut self) -> std::io::Result<Self::Message> {
        let mut buffer = Vec::new();
        loop {
            let mut read_buffer = [0; 1024]; 
            let n = self.stream.read(&mut read_buffer).await.expect("unable to read stream to buffer");
            if n == 0 {
                break;
            }

            buffer.extend_from_slice(&read_buffer[..n]);
            let results = Self::parse_messages(&mut buffer).await?;
            if !results.is_empty() {
                return Ok(results)
            }
        }
        Err(
            std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "No complete messages received"
            )
        )

    }
    async fn parse_messages(msg: &mut Vec<u8>) -> std::io::Result<Self::Message> {
        let mut results = Vec::new();
        while msg.len() >= HEADER_SIZE {
            let total_len = try_get_message_len(msg)?;
            if msg.len() >= total_len {
                let topic_len = try_get_topic_len(msg)?;
                let (_, message) = parse_next_message(total_len, topic_len, msg).await;
                let message_offset = TOPIC_SIZE_OFFSET + topic_len;
                let msg = &message[message_offset..message_offset + total_len];
                results.push(msg.to_vec());
            }
        }

        let msg_results = results.par_iter().filter_map(|m| {
            serde_json::from_slice(
                &m
            ).ok()
        }).collect();

        Ok(msg_results)
    }
}

pub struct FilesystemPublisher {
    stream: TcpStream
}

impl FilesystemPublisher {
    pub async fn new(uri: &str) -> std::io::Result<Self> {
        let stream = TcpStream::connect(uri).await?;
        Ok(Self { stream })
    }
}

#[async_trait]
impl PubStream for FilesystemPublisher {
    type Topic = FilesystemTopic;
    type Message<'async_trait> = Event where Self: 'async_trait;

    async fn publish(&mut self, topic: Self::Topic, msg: Self::Message<'async_trait>) -> std::io::Result<()> {
        let topic_len = topic.to_string().len();
        let topic_len_bytes = topic_len.to_be_bytes();
        let message_str = serde_json::to_string(&msg).map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                e
            )
        })?;
        let message_len = message_str.len();
        let message_len_bytes = message_len.to_be_bytes();
        let total_len = conductor::HEADER_SIZE + conductor::TOPIC_SIZE_OFFSET + topic_len + message_len;
        let mut full_message = Vec::with_capacity(total_len);
        full_message.extend_from_slice(&message_len_bytes);
        full_message.extend_from_slice(&topic_len_bytes);
        full_message.extend_from_slice(&topic.to_string().as_bytes());
        full_message.extend_from_slice(message_str.as_bytes());
        self.stream.write_all(&full_message).await?;
        Ok(())
    }
}


pub struct LibrettoPublisher {
    stream: TcpStream
}

impl LibrettoPublisher {
    pub async fn new(uri: &str) -> std::io::Result<Self> {
        let stream = TcpStream::connect(uri).await?;
        Ok(Self { stream })
    }
}

#[async_trait]
impl PubStream for LibrettoPublisher {
    type Topic = LibrettoTopic;
    type Message<'async_trait> = LibrettoEvent where Self: 'async_trait;

    async fn publish(&mut self, topic: Self::Topic, msg: Self::Message<'async_trait>) -> std::io::Result<()> {
        let topic_len = topic.to_string().len();
        let topic_len_bytes = topic_len.to_be_bytes();
        let message_str = serde_json::to_string(&msg).map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                e
            )
        })?;
        let message_len = message_str.len();
        let message_len_bytes = message_len.to_be_bytes();
        let total_len = conductor::HEADER_SIZE + conductor::TOPIC_SIZE_OFFSET + topic_len + message_len;
        let mut full_message = Vec::with_capacity(total_len);
        full_message.extend_from_slice(&message_len_bytes);
        full_message.extend_from_slice(&topic_len_bytes);
        full_message.extend_from_slice(&topic.to_string().as_bytes());
        full_message.extend_from_slice(message_str.as_bytes());
        self.stream.write_all(&full_message).await?;
        Ok(())
    }
}

