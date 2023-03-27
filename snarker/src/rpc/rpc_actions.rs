use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::p2p::connection::incoming::P2pConnectionIncomingInitOpts;
use crate::p2p::connection::outgoing::P2pConnectionOutgoingInitOpts;
use crate::p2p::webrtc;
use crate::service::ActionStatsForRanges;
use crate::ActionKind;

use super::RpcId;

pub type RpcActionWithMeta = redux::ActionWithMeta<RpcAction>;
pub type RpcActionWithMetaRef<'a> = redux::ActionWithMeta<&'a RpcAction>;

#[derive(derive_more::From, Serialize, Deserialize, Debug, Clone)]
pub enum RpcAction {
    GlobalStateGet(RpcGlobalStateGetAction),

    // Stats
    ActionStatsGet(RpcActionStatsGetAction),

    P2pConnectionOutgoingInit(RpcP2pConnectionOutgoingInitAction),
    P2pConnectionOutgoingPending(RpcP2pConnectionOutgoingPendingAction),
    P2pConnectionOutgoingError(RpcP2pConnectionOutgoingErrorAction),
    P2pConnectionOutgoingSuccess(RpcP2pConnectionOutgoingSuccessAction),

    P2pConnectionIncomingInit(RpcP2pConnectionIncomingInitAction),
    P2pConnectionIncomingPending(RpcP2pConnectionIncomingPendingAction),
    P2pConnectionIncomingAnswerSet(RpcP2pConnectionIncomingAnswerSetAction),
    P2pConnectionIncomingError(RpcP2pConnectionIncomingErrorAction),
    P2pConnectionIncomingSuccess(RpcP2pConnectionIncomingSuccessAction),

    Finish(RpcFinishAction),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RpcGlobalStateGetAction {
    pub rpc_id: RpcId,
}

impl redux::EnablingCondition<crate::State> for RpcGlobalStateGetAction {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ActionStatsQuery {
    SinceStart,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ActionStatsResponse {
    SinceStart(BTreeMap<ActionKind, ActionStatsForRanges>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RpcActionStatsGetAction {
    pub rpc_id: RpcId,
    pub query: ActionStatsQuery,
}

impl redux::EnablingCondition<crate::State> for RpcActionStatsGetAction {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RpcP2pConnectionOutgoingInitAction {
    pub rpc_id: RpcId,
    pub opts: P2pConnectionOutgoingInitOpts,
}

impl redux::EnablingCondition<crate::State> for RpcP2pConnectionOutgoingInitAction {
    fn is_enabled(&self, state: &crate::State) -> bool {
        !state.rpc.requests.contains_key(&self.rpc_id)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RpcP2pConnectionOutgoingPendingAction {
    pub rpc_id: RpcId,
}

impl redux::EnablingCondition<crate::State> for RpcP2pConnectionOutgoingPendingAction {
    fn is_enabled(&self, state: &crate::State) -> bool {
        state
            .rpc
            .requests
            .get(&self.rpc_id)
            .map_or(false, |v| v.status.is_init())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RpcP2pConnectionOutgoingErrorAction {
    pub rpc_id: RpcId,
    pub error: String,
}

impl redux::EnablingCondition<crate::State> for RpcP2pConnectionOutgoingErrorAction {
    fn is_enabled(&self, state: &crate::State) -> bool {
        state
            .rpc
            .requests
            .get(&self.rpc_id)
            .map_or(false, |v| v.status.is_pending())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RpcP2pConnectionOutgoingSuccessAction {
    pub rpc_id: RpcId,
}

impl redux::EnablingCondition<crate::State> for RpcP2pConnectionOutgoingSuccessAction {
    fn is_enabled(&self, state: &crate::State) -> bool {
        state
            .rpc
            .requests
            .get(&self.rpc_id)
            .map_or(false, |v| v.status.is_pending())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RpcP2pConnectionIncomingInitAction {
    pub rpc_id: RpcId,
    pub opts: P2pConnectionIncomingInitOpts,
}

impl redux::EnablingCondition<crate::State> for RpcP2pConnectionIncomingInitAction {
    fn is_enabled(&self, state: &crate::State) -> bool {
        !state.rpc.requests.contains_key(&self.rpc_id)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RpcP2pConnectionIncomingPendingAction {
    pub rpc_id: RpcId,
}

impl redux::EnablingCondition<crate::State> for RpcP2pConnectionIncomingPendingAction {
    fn is_enabled(&self, state: &crate::State) -> bool {
        state
            .rpc
            .requests
            .get(&self.rpc_id)
            .map_or(false, |v| v.status.is_init())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RpcP2pConnectionIncomingAnswerSetAction {
    pub rpc_id: RpcId,
    pub answer: webrtc::Answer,
}

impl redux::EnablingCondition<crate::State> for RpcP2pConnectionIncomingAnswerSetAction {
    fn is_enabled(&self, state: &crate::State) -> bool {
        state
            .rpc
            .requests
            .get(&self.rpc_id)
            .map_or(false, |v| v.status.is_pending())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RpcP2pConnectionIncomingErrorAction {
    pub rpc_id: RpcId,
    pub error: String,
}

impl redux::EnablingCondition<crate::State> for RpcP2pConnectionIncomingErrorAction {
    fn is_enabled(&self, state: &crate::State) -> bool {
        state
            .rpc
            .requests
            .get(&self.rpc_id)
            .map_or(false, |v| v.status.is_pending())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RpcP2pConnectionIncomingSuccessAction {
    pub rpc_id: RpcId,
}

impl redux::EnablingCondition<crate::State> for RpcP2pConnectionIncomingSuccessAction {
    fn is_enabled(&self, state: &crate::State) -> bool {
        state
            .rpc
            .requests
            .get(&self.rpc_id)
            .map_or(false, |v| v.status.is_pending())
    }
}

/// Finish/Cleanup rpc request.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RpcFinishAction {
    pub rpc_id: RpcId,
}

impl redux::EnablingCondition<crate::State> for RpcFinishAction {
    fn is_enabled(&self, state: &crate::State) -> bool {
        state
            .rpc
            .requests
            .get(&self.rpc_id)
            .map_or(false, |v| v.status.is_finished())
    }
}

macro_rules! impl_into_global_action {
    ($a:ty) => {
        impl From<$a> for crate::Action {
            fn from(value: $a) -> Self {
                Self::Rpc(value.into())
            }
        }
    };
}

impl_into_global_action!(RpcGlobalStateGetAction);

impl_into_global_action!(RpcActionStatsGetAction);

impl_into_global_action!(RpcP2pConnectionOutgoingInitAction);
impl_into_global_action!(RpcP2pConnectionOutgoingPendingAction);
impl_into_global_action!(RpcP2pConnectionOutgoingErrorAction);
impl_into_global_action!(RpcP2pConnectionOutgoingSuccessAction);

impl_into_global_action!(RpcP2pConnectionIncomingInitAction);
impl_into_global_action!(RpcP2pConnectionIncomingPendingAction);
impl_into_global_action!(RpcP2pConnectionIncomingAnswerSetAction);
impl_into_global_action!(RpcP2pConnectionIncomingErrorAction);
impl_into_global_action!(RpcP2pConnectionIncomingSuccessAction);

impl_into_global_action!(RpcFinishAction);