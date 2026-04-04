#![allow(warnings, unused)]

use super::OptionsFiller;
use crate::config::ConsumerModeRef;
use schemars::JsonSchema;
use sea_streamer::iggy::{
    IggyAutoCommit, IggyConnectOptions, IggyConsumerOptions, IggyPartitioning,
    IggyPollingStrategy, IggyProducerOptions,
};
use sea_streamer::{ConnectOptions as ConnectOptionsTrait, ConsumerMode};
use serde::Deserialize;
use std::time::Duration;

#[derive(Default, Debug, Clone, JsonSchema, Deserialize)]
pub struct IggyOptions {
    connect: Option<ConnectOptions>,
    producer: Option<ProducerOptions>,
    consumer: Option<ConsumerOptions>,
}

impl OptionsFiller for IggyOptions {
    type ConnectOptsType = IggyConnectOptions;
    type ConsumerOptsType = IggyConsumerOptions;
    type ProducerOptsType = IggyProducerOptions;

    fn fill_connect_options(&self, opts: &mut Self::ConnectOptsType) {
        let Some(conn) = &self.connect else {
            return;
        };
        if let Some(url) = &conn.url {
            opts.set_url(url.clone());
        }
        if let Some(username) = &conn.username {
            let password = conn.password.as_deref().unwrap_or("");
            opts.set_credentials(username.clone(), password);
        }
        if let Some(timeout) = conn.timeout {
            let _ = opts.set_timeout(timeout);
        }
    }

    fn fill_consumer_options(&self, opts: &mut Self::ConsumerOptsType) {
        let Some(consumer) = &self.consumer else {
            return;
        };
        if let Some(stream_name) = &consumer.stream_name {
            opts.set_stream_name(stream_name.clone());
        }
        if let Some(topic_name) = &consumer.topic_name {
            opts.set_topic_name(topic_name.clone());
        }
        if let Some(batch_size) = consumer.batch_size {
            opts.set_batch_size(batch_size);
        }
        if let Some(polling_interval_ms) = consumer.polling_interval_ms {
            opts.set_polling_interval_ms(polling_interval_ms);
        }
        if let Some(consumer_name) = &consumer.consumer_name {
            opts.set_consumer_name(consumer_name.clone());
        }
        if let Some(val) = consumer.create_stream_if_not_exists {
            opts.set_create_stream_if_not_exists(val);
        }
        if let Some(val) = consumer.create_topic_if_not_exists {
            opts.set_create_topic_if_not_exists(val);
        }
        if let Some(count) = consumer.partitions_count {
            opts.set_partitions_count(count);
        }
        if let Some(val) = consumer.auto_join_consumer_group {
            opts.set_auto_join_consumer_group(val);
        }
        if let Some(val) = consumer.create_consumer_group_if_not_exists {
            opts.set_create_consumer_group_if_not_exists(val);
        }
        if let Some(strategy) = &consumer.polling_strategy {
            opts.set_polling_strategy(strategy.into_iggy());
        }
        if let Some(auto_commit) = &consumer.auto_commit {
            opts.set_auto_commit(auto_commit.into_iggy());
        }
        if let Some(ms) = consumer.polling_retry_interval_ms {
            opts.set_polling_retry_interval_ms(ms);
        }
        if let Some(retries) = consumer.init_retries {
            opts.set_init_retries(retries);
        }
        if let Some(ms) = consumer.init_interval_ms {
            opts.set_init_interval_ms(ms);
        }
    }

    fn fill_producer_options(&self, opts: &mut Self::ProducerOptsType) {
        let Some(producer) = &self.producer else {
            return;
        };
        if let Some(stream_name) = &producer.stream_name {
            opts.set_stream_name(stream_name.clone());
        }
        if let Some(topic_name) = &producer.topic_name {
            opts.set_topic_name(topic_name.clone());
        }
        if let Some(batch_size) = producer.batch_size {
            opts.set_batch_size(batch_size);
        }
        if let Some(send_interval_ms) = producer.send_interval_ms {
            opts.set_send_interval_ms(send_interval_ms);
        }
        if let Some(count) = producer.partitions_count {
            opts.set_partitions_count(count);
        }
        if let Some(val) = producer.create_stream_if_not_exists {
            opts.set_create_stream_if_not_exists(val);
        }
        if let Some(val) = producer.create_topic_if_not_exists {
            opts.set_create_topic_if_not_exists(val);
        }
        if let Some(factor) = producer.topic_replication_factor {
            opts.set_topic_replication_factor(factor);
        }
        if let Some(partitioning) = &producer.partitioning {
            opts.set_partitioning(partitioning.into_iggy());
        }
        if let Some(count) = producer.send_retries_count {
            opts.set_send_retries_count(count);
        }
        if let Some(ms) = producer.send_retries_interval_ms {
            opts.set_send_retries_interval_ms(ms);
        }
    }

    fn default_consumer_mode(&self) -> Option<&ConsumerMode> {
        match &self.consumer {
            Some(consumer) => Some(&consumer.mode),
            None => None,
        }
    }

    fn default_consumer_group_id(&self) -> Option<String> {
        match &self.consumer {
            Some(consumer) => consumer.group_id.clone(),
            None => None,
        }
    }
}

#[derive(Debug, Clone, JsonSchema, Deserialize)]
struct ConnectOptions {
    /// Iggy connection string URL, e.g. `iggy://user:pass@localhost:8090`
    ///
    /// Supports transport selection via scheme:
    /// `iggy+tcp://`, `iggy+quic://`, `iggy+http://`
    ///
    /// Supports query parameters: `tls`, `tls_domain`, `tls_ca_file`,
    /// `reconnection_retries`, `reconnection_interval`, `reestablish_after`,
    /// `heartbeat_interval`, `nodelay`
    url: Option<String>,
    username: Option<String>,
    password: Option<String>,
    timeout: Option<Duration>,
}

#[derive(Debug, Clone, JsonSchema, Deserialize)]
struct ConsumerOptions {
    #[serde(with = "ConsumerModeRef")]
    mode: ConsumerMode,
    group_id: Option<String>,
    stream_name: Option<String>,
    topic_name: Option<String>,
    batch_size: Option<u32>,
    polling_interval_ms: Option<u64>,
    consumer_name: Option<String>,
    create_stream_if_not_exists: Option<bool>,
    create_topic_if_not_exists: Option<bool>,
    partitions_count: Option<u32>,
    auto_join_consumer_group: Option<bool>,
    create_consumer_group_if_not_exists: Option<bool>,
    polling_strategy: Option<PollingStrategy>,
    auto_commit: Option<AutoCommit>,
    polling_retry_interval_ms: Option<u64>,
    init_retries: Option<u32>,
    init_interval_ms: Option<u64>,
}

#[derive(Debug, Clone, JsonSchema, Deserialize)]
struct ProducerOptions {
    stream_name: Option<String>,
    topic_name: Option<String>,
    batch_size: Option<u32>,
    send_interval_ms: Option<u64>,
    partitions_count: Option<u32>,
    create_stream_if_not_exists: Option<bool>,
    create_topic_if_not_exists: Option<bool>,
    topic_replication_factor: Option<u32>,
    partitioning: Option<Partitioning>,
    send_retries_count: Option<u32>,
    send_retries_interval_ms: Option<u64>,
}

#[derive(Debug, Clone, JsonSchema, Deserialize)]
enum PollingStrategy {
    Offset(u64),
    Timestamp(u64),
    First,
    Last,
    Next,
}

impl PollingStrategy {
    fn into_iggy(&self) -> IggyPollingStrategy {
        match self {
            PollingStrategy::Offset(v) => IggyPollingStrategy::Offset(*v),
            PollingStrategy::Timestamp(v) => IggyPollingStrategy::Timestamp(*v),
            PollingStrategy::First => IggyPollingStrategy::First,
            PollingStrategy::Last => IggyPollingStrategy::Last,
            PollingStrategy::Next => IggyPollingStrategy::Next,
        }
    }
}

#[derive(Debug, Clone, JsonSchema, Deserialize)]
enum AutoCommit {
    Disabled,
    AfterPolling,
    Interval(u64),
    IntervalOrAfterPolling(u64),
}

impl AutoCommit {
    fn into_iggy(&self) -> IggyAutoCommit {
        match self {
            AutoCommit::Disabled => IggyAutoCommit::Disabled,
            AutoCommit::AfterPolling => IggyAutoCommit::AfterPolling,
            AutoCommit::Interval(ms) => IggyAutoCommit::Interval(*ms),
            AutoCommit::IntervalOrAfterPolling(ms) => IggyAutoCommit::IntervalOrAfterPolling(*ms),
        }
    }
}

#[derive(Debug, Clone, JsonSchema, Deserialize)]
enum Partitioning {
    Balanced,
    PartitionId(u32),
    MessageKey(Vec<u8>),
}

impl Partitioning {
    fn into_iggy(&self) -> IggyPartitioning {
        match self {
            Partitioning::Balanced => IggyPartitioning::Balanced,
            Partitioning::PartitionId(id) => IggyPartitioning::PartitionId(*id),
            Partitioning::MessageKey(key) => IggyPartitioning::MessageKey(key.clone()),
        }
    }
}
