use crate::BotId;

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Default)]
pub struct NetSDK {
    pub bots: Arc<RwLock<HashMap<BotId, NetSDKConfig>>>,
}

#[derive(Clone, Debug)]
pub struct NetSDKConfig {
    pub host: std::net::IpAddr,
    pub port: u16,
    pub authorize: Option<String>,
}

// async fn handle_signal() {
//     match signal.op {
//         0 => {
//             if let Some(body) = signal.body {
//                 match serde_json::from_value::<Event>(body) {
//                     Ok(event) => {
//                         info!(target: SATORI, "receive event: {:?}", event);
//                         let s = s.clone();
//                         *seq = event.id;
//                         tokio::spawn(async move { s.handle_event(event).await });
//                     }
//                     Err(e) => {
//                         warn!(target: SATORI, "deserlize event error:{e}");
//                     }
//                 }
//             }
//         }
//         2 => {}
//         4 => {
//             if let Some(body) = signal.body {
//                 match serde_json::from_value::<Logins>(body) {
//                     Ok(logins) => {
//                         let mut bots = bots.write().await;
//                         for login in logins.logins {
//                             bots.insert(
//                                 BotId {
//                                     platform: login.platform.unwrap(),
//                                     id: login.self_id.unwrap(),
//                                 },
//                                 net.clone(),
//                             );
//                         }
//                     }
//                     Err(e) => {
//                         warn!(target: SATORI, "deserlize logins error:{e}")
//                     }
//                 }
//             }
//         }
//         _ => unreachable!(),
//     }
// }
