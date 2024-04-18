use super::heartbeat_manager::HeartbeatManager;
use crate::{
    metrics::metrics_heartbeat_keep_alive_run_info,
    server::{tcp::packet::RequestPackage, MQTTProtocol},
};
use common_base::{
    log::{debug, error, info},
    tools::{now_mills, now_second},
};
use protocol::mqtt::{Disconnect, DisconnectProperties, DisconnectReasonCode, MQTTPacket};
use serde::{Deserialize, Serialize};
use std::{sync::Arc, time::Duration};
use tokio::{
    sync::{
        broadcast::{self, Sender},
        RwLock, Semaphore,
    },
    time::sleep,
};

pub struct KeepAlive {
    shard_num: u64,
    connection_keep_live_data: Arc<RwLock<HeartbeatManager>>,
    request_queue_sx4: Sender<RequestPackage>,
    request_queue_sx5: Sender<RequestPackage>,
    stop_send: broadcast::Receiver<bool>,
}

impl KeepAlive {
    pub fn new(
        shard_num: u64,
        connection_keep_live_data: Arc<RwLock<HeartbeatManager>>,
        request_queue_sx4: Sender<RequestPackage>,
        request_queue_sx5: Sender<RequestPackage>,
        stop_send: broadcast::Receiver<bool>,
    ) -> Self {
        return KeepAlive {
            shard_num,
            connection_keep_live_data,
            request_queue_sx4,
            request_queue_sx5,
            stop_send,
        };
    }

    // TCP connection heartbeat detection is performed in parallel, and subsequent processing is carried out
    pub async fn start_heartbeat_check(&mut self) {
        loop {
            match self.stop_send.try_recv() {
                Ok(flag) => {
                    if flag {
                        info("KeepAlive thread stopped successfully".to_string());
                        break;
                    }
                }
                Err(_) => {}
            }
            let lock = self.connection_keep_live_data.read().await;
            let mut heartbeat_data = lock.heartbeat_data.clone();
            drop(lock);

            sleep(Duration::from_secs(5)).await;
            //
            let semaphore = Arc::new(Semaphore::new(self.shard_num as usize));
            for i in 0..self.shard_num {
                let data = heartbeat_data.remove(&i);
                let request_queue_sx4 = self.request_queue_sx4.clone();
                let request_queue_sx5 = self.request_queue_sx5.clone();
                let sp = semaphore.clone();
                tokio::spawn(async move {
                    match sp.acquire().await {
                        Ok(_) => {}
                        Err(e) => {
                            error(format!("The heartbeat thread failed to retrieve the semaplight with error message:{}",e.to_string()));
                        }
                    }
                    if let Some(da) = data {
                        for (connect_id, time) in da.heartbeat_data {
                            // The server will decide that the connection has failed twice as long as the client-set expiration time.
                            let max_timeout = (time.keep_live * 2) as u64;
                            if (now_second() - time.heartbeat) > max_timeout {
                                let disconnect = Disconnect {
                                    reason_code: DisconnectReasonCode::AdministrativeAction,
                                };
                                let properties = Some(DisconnectProperties {
                                        session_expiry_interval: None,
                                        reason_string: Some("The connection was closed by the server because the heartbeat timeout was not reported.".to_string()),
                                        user_properties: vec![("heartbeat_close".to_string(), "true".to_string())],
                                        server_reference: None,
                                    });
                                if time.protobol == MQTTProtocol::MQTT4 {
                                    let req = RequestPackage {
                                        connection_id: connect_id,
                                        packet: MQTTPacket::Disconnect(disconnect.clone(), None),
                                    };
                                    match request_queue_sx4.send(req) {
                                        Ok(_) => {}
                                        Err(e) => {
                                            error(e.to_string());
                                        }
                                    };
                                }
                                if time.protobol == MQTTProtocol::MQTT5 {
                                    let req = RequestPackage {
                                        connection_id: connect_id,
                                        packet: MQTTPacket::Disconnect(disconnect, properties),
                                    };

                                    match request_queue_sx5.send(req) {
                                        Ok(_) => {}
                                        Err(e) => {
                                            error(e.to_string());
                                        }
                                    };
                                }
                            }
                        }
                    }
                });
            }

            // Waiting for all spawn to complete, thinking about the next batch of detection
            let start_time = now_mills();
            loop {
                if semaphore.available_permits() == (self.shard_num as usize) {
                    break;
                }
                sleep(Duration::from_millis(100)).await;
            }
            let end_time = now_mills();
            let use_time = end_time - start_time;
            let run_info = KeepAliveRunInfo {
                start_time,
                end_time,
                use_time,
            };
            metrics_heartbeat_keep_alive_run_info(use_time);
            debug(format!("{:?}", run_info));
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct KeepAliveRunInfo {
    pub start_time: u128,
    pub end_time: u128,
    pub use_time: u128,
}
