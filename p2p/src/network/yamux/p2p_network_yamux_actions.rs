use openmina_core::ActionEvent;
use serde::{Deserialize, Serialize};

use super::p2p_network_yamux_state::{StreamId, YamuxFlags, YamuxFrame, YamuxPing};
use crate::{token, ConnectionAddr, Data, P2pState};

#[derive(Serialize, Deserialize, Debug, Clone, ActionEvent)]
#[action_event(fields(display(addr), stream_id, debug(data), fin, debug(stream_kind)))]
pub enum P2pNetworkYamuxAction {
    IncomingData {
        addr: ConnectionAddr,
        data: Data,
    },
    OutgoingData {
        addr: ConnectionAddr,
        stream_id: StreamId,
        data: Data,
        flags: YamuxFlags,
    },
    #[action_event(level = trace)]
    IncomingFrame {
        addr: ConnectionAddr,
        frame: YamuxFrame,
    },
    #[action_event(level = trace)]
    OutgoingFrame {
        addr: ConnectionAddr,
        frame: YamuxFrame,
    },
    PingStream {
        addr: ConnectionAddr,
        ping: YamuxPing,
    },
    OpenStream {
        addr: ConnectionAddr,
        stream_id: StreamId,
        stream_kind: token::StreamKind,
    },
}

impl P2pNetworkYamuxAction {
    pub fn addr(&self) -> &ConnectionAddr {
        match self {
            Self::IncomingData { addr, .. } => addr,
            Self::OutgoingData { addr, .. } => addr,
            Self::IncomingFrame { addr, .. } => addr,
            Self::OutgoingFrame { addr, .. } => addr,
            Self::PingStream { addr, .. } => addr,
            Self::OpenStream { addr, .. } => addr,
        }
    }
}

impl From<P2pNetworkYamuxAction> for crate::P2pAction {
    fn from(a: P2pNetworkYamuxAction) -> Self {
        Self::Network(a.into())
    }
}

impl redux::EnablingCondition<P2pState> for P2pNetworkYamuxAction {
    fn is_enabled(&self, _state: &P2pState, _time: redux::Timestamp) -> bool {
        #[allow(unused_variables)]
        match self {
            P2pNetworkYamuxAction::IncomingData { addr, data } => true,
            P2pNetworkYamuxAction::OutgoingData {
                addr,
                stream_id,
                data,
                flags,
            } => true,
            P2pNetworkYamuxAction::IncomingFrame { addr, frame } => true,
            P2pNetworkYamuxAction::OutgoingFrame { addr, frame } => true,
            P2pNetworkYamuxAction::PingStream { addr, ping } => true,
            P2pNetworkYamuxAction::OpenStream {
                addr,
                stream_id,
                stream_kind,
            } => true,
        }
    }
}