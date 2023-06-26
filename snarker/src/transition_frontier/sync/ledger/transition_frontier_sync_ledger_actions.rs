use serde::{Deserialize, Serialize};

use crate::ledger::LedgerAddress;
use crate::p2p::channels::rpc::P2pRpcId;
use crate::p2p::PeerId;

use super::{PeerLedgerQueryResponse, PeerRpcState, TransitionFrontierSyncLedgerState};

pub type TransitionFrontierSyncLedgerActionWithMeta =
    redux::ActionWithMeta<TransitionFrontierSyncLedgerAction>;
pub type TransitionFrontierSyncLedgerActionWithMetaRef<'a> =
    redux::ActionWithMeta<&'a TransitionFrontierSyncLedgerAction>;

#[derive(derive_more::From, Serialize, Deserialize, Debug, Clone)]
pub enum TransitionFrontierSyncLedgerAction {
    Init(TransitionFrontierSyncLedgerInitAction),
    SnarkedLedgerSyncPending(TransitionFrontierSyncLedgerSnarkedLedgerSyncPendingAction),
    SnarkedLedgerSyncPeersQuery(TransitionFrontierSyncLedgerSnarkedLedgerSyncPeersQueryAction),
    SnarkedLedgerSyncPeerQueryInit(
        TransitionFrontierSyncLedgerSnarkedLedgerSyncPeerQueryInitAction,
    ),
    SnarkedLedgerSyncPeerQueryPending(
        TransitionFrontierSyncLedgerSnarkedLedgerSyncPeerQueryPendingAction,
    ),
    SnarkedLedgerSyncPeerQuerySuccess(
        TransitionFrontierSyncLedgerSnarkedLedgerSyncPeerQuerySuccessAction,
    ),
    SnarkedLedgerSyncSuccess(TransitionFrontierSyncLedgerSnarkedLedgerSyncSuccessAction),
    StagedLedgerPartsFetchInit(TransitionFrontierSyncLedgerStagedLedgerPartsFetchInitAction),
    StagedLedgerPartsFetchPending(TransitionFrontierSyncLedgerStagedLedgerPartsFetchPendingAction),
    StagedLedgerPartsFetchSuccess(TransitionFrontierSyncLedgerStagedLedgerPartsFetchSuccessAction),
    StagedLedgerReconstructInit(TransitionFrontierSyncLedgerStagedLedgerReconstructInitAction),
    StagedLedgerReconstructPending(
        TransitionFrontierSyncLedgerStagedLedgerReconstructPendingAction,
    ),
    StagedLedgerReconstructSuccess(
        TransitionFrontierSyncLedgerStagedLedgerReconstructSuccessAction,
    ),
    Success(TransitionFrontierSyncLedgerSuccessAction),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransitionFrontierSyncLedgerInitAction {}

impl redux::EnablingCondition<crate::State> for TransitionFrontierSyncLedgerInitAction {
    fn is_enabled(&self, state: &crate::State) -> bool {
        state
            .transition_frontier
            .sync
            .root_ledger()
            .map_or(false, |s| {
                matches!(s, TransitionFrontierSyncLedgerState::Init { .. })
            })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransitionFrontierSyncLedgerSnarkedLedgerSyncPendingAction {}

impl redux::EnablingCondition<crate::State>
    for TransitionFrontierSyncLedgerSnarkedLedgerSyncPendingAction
{
    fn is_enabled(&self, state: &crate::State) -> bool {
        state
            .transition_frontier
            .sync
            .root_ledger()
            .map_or(false, |s| {
                matches!(s, TransitionFrontierSyncLedgerState::Init { .. })
            })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransitionFrontierSyncLedgerSnarkedLedgerSyncPeersQueryAction {}

impl redux::EnablingCondition<crate::State>
    for TransitionFrontierSyncLedgerSnarkedLedgerSyncPeersQueryAction
{
    fn is_enabled(&self, state: &crate::State) -> bool {
        let peers_available = state
            .p2p
            .ready_peers_iter()
            .any(|(_, p)| p.channels.rpc.can_send_request());
        peers_available
            && state
                .transition_frontier
                .sync
                .root_ledger()
                .map_or(false, |s| match s {
                    TransitionFrontierSyncLedgerState::SnarkedLedgerSyncPending {
                        next_addr,
                        ..
                    } => next_addr.is_some(),
                    _ => false,
                })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransitionFrontierSyncLedgerSnarkedLedgerSyncPeerQueryInitAction {
    pub address: LedgerAddress,
    pub peer_id: PeerId,
}

impl redux::EnablingCondition<crate::State>
    for TransitionFrontierSyncLedgerSnarkedLedgerSyncPeerQueryInitAction
{
    fn is_enabled(&self, state: &crate::State) -> bool {
        let check_next_addr =
            state
                .transition_frontier
                .sync
                .root_ledger()
                .map_or(false, |s| match s {
                    TransitionFrontierSyncLedgerState::SnarkedLedgerSyncPending {
                        pending,
                        next_addr,
                        ..
                    } => next_addr.as_ref().map_or(false, |next_addr| {
                        next_addr == &self.address
                            && (next_addr.to_index().0 != 0 || pending.is_empty())
                    }),
                    _ => false,
                });

        let check_peer_available = state
            .p2p
            .get_ready_peer(&self.peer_id)
            .map_or(false, |p| p.channels.rpc.can_send_request());
        check_next_addr && check_peer_available
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransitionFrontierSyncLedgerSnarkedLedgerSyncPeerQueryPendingAction {
    pub address: LedgerAddress,
    pub peer_id: PeerId,
    pub rpc_id: P2pRpcId,
}

impl redux::EnablingCondition<crate::State>
    for TransitionFrontierSyncLedgerSnarkedLedgerSyncPeerQueryPendingAction
{
    fn is_enabled(&self, state: &crate::State) -> bool {
        state
            .transition_frontier
            .sync
            .root_ledger()
            .map_or(false, |s| match s {
                TransitionFrontierSyncLedgerState::SnarkedLedgerSyncPending { pending, .. } => {
                    pending
                        .iter()
                        .filter_map(|(_, query_state)| query_state.attempts.get(&self.peer_id))
                        .any(|peer_rpc_state| matches!(peer_rpc_state, PeerRpcState::Init { .. }))
                }
                _ => false,
            })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransitionFrontierSyncLedgerSnarkedLedgerSyncPeerQuerySuccessAction {
    pub peer_id: PeerId,
    pub rpc_id: P2pRpcId,
    pub response: PeerLedgerQueryResponse,
}

impl redux::EnablingCondition<crate::State>
    for TransitionFrontierSyncLedgerSnarkedLedgerSyncPeerQuerySuccessAction
{
    fn is_enabled(&self, state: &crate::State) -> bool {
        state
            .transition_frontier
            .sync
            .root_ledger()
            .map_or(false, |s| {
                // TODO(binier): check if expected response
                // kind is correct.
                s.snarked_ledger_peer_query_get(&self.peer_id, self.rpc_id)
                    .is_some()
            })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransitionFrontierSyncLedgerSnarkedLedgerSyncSuccessAction {}

impl redux::EnablingCondition<crate::State>
    for TransitionFrontierSyncLedgerSnarkedLedgerSyncSuccessAction
{
    fn is_enabled(&self, state: &crate::State) -> bool {
        state
            .transition_frontier
            .sync
            .root_ledger()
            .map_or(false, |s| match s {
                TransitionFrontierSyncLedgerState::SnarkedLedgerSyncPending {
                    pending,
                    next_addr,
                    ..
                } => next_addr.is_none() && pending.is_empty(),
                _ => false,
            })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransitionFrontierSyncLedgerStagedLedgerPartsFetchInitAction {}

impl redux::EnablingCondition<crate::State>
    for TransitionFrontierSyncLedgerStagedLedgerPartsFetchInitAction
{
    fn is_enabled(&self, state: &crate::State) -> bool {
        state
            .transition_frontier
            .sync
            .root_ledger()
            .map_or(false, |s| {
                matches!(
                    s,
                    TransitionFrontierSyncLedgerState::SnarkedLedgerSyncSuccess { .. }
                )
            })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransitionFrontierSyncLedgerStagedLedgerPartsFetchPendingAction {}

impl redux::EnablingCondition<crate::State>
    for TransitionFrontierSyncLedgerStagedLedgerPartsFetchPendingAction
{
    fn is_enabled(&self, state: &crate::State) -> bool {
        state
            .transition_frontier
            .sync
            .root_ledger()
            .map_or(false, |s| {
                matches!(
                    s,
                    TransitionFrontierSyncLedgerState::SnarkedLedgerSyncSuccess { .. }
                )
            })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransitionFrontierSyncLedgerStagedLedgerPartsFetchSuccessAction {}

impl redux::EnablingCondition<crate::State>
    for TransitionFrontierSyncLedgerStagedLedgerPartsFetchSuccessAction
{
    fn is_enabled(&self, state: &crate::State) -> bool {
        state
            .transition_frontier
            .sync
            .root_ledger()
            .map_or(false, |s| {
                matches!(
                    s,
                    TransitionFrontierSyncLedgerState::StagedLedgerPartsFetchPending { .. }
                )
            })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransitionFrontierSyncLedgerStagedLedgerReconstructInitAction {}

impl redux::EnablingCondition<crate::State>
    for TransitionFrontierSyncLedgerStagedLedgerReconstructInitAction
{
    fn is_enabled(&self, state: &crate::State) -> bool {
        state
            .transition_frontier
            .sync
            .root_ledger()
            .map_or(false, |s| {
                matches!(
                    s,
                    TransitionFrontierSyncLedgerState::StagedLedgerPartsFetchSuccess { .. }
                )
            })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransitionFrontierSyncLedgerStagedLedgerReconstructPendingAction {}

impl redux::EnablingCondition<crate::State>
    for TransitionFrontierSyncLedgerStagedLedgerReconstructPendingAction
{
    fn is_enabled(&self, state: &crate::State) -> bool {
        state
            .transition_frontier
            .sync
            .root_ledger()
            .map_or(false, |s| {
                matches!(
                    s,
                    TransitionFrontierSyncLedgerState::StagedLedgerPartsFetchSuccess { .. }
                )
            })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransitionFrontierSyncLedgerStagedLedgerReconstructSuccessAction {}

impl redux::EnablingCondition<crate::State>
    for TransitionFrontierSyncLedgerStagedLedgerReconstructSuccessAction
{
    fn is_enabled(&self, state: &crate::State) -> bool {
        state
            .transition_frontier
            .sync
            .root_ledger()
            .map_or(false, |s| {
                matches!(
                    s,
                    TransitionFrontierSyncLedgerState::StagedLedgerReconstructPending { .. }
                )
            })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransitionFrontierSyncLedgerSuccessAction {}

impl redux::EnablingCondition<crate::State> for TransitionFrontierSyncLedgerSuccessAction {
    fn is_enabled(&self, state: &crate::State) -> bool {
        state
            .transition_frontier
            .sync
            .root_ledger()
            .map_or(false, |s| {
                matches!(
                    s,
                    TransitionFrontierSyncLedgerState::StagedLedgerReconstructSuccess { .. }
                )
            })
    }
}

use crate::transition_frontier::TransitionFrontierAction;

macro_rules! impl_into_global_action {
    ($a:ty) => {
        impl From<$a> for crate::Action {
            fn from(value: $a) -> Self {
                Self::TransitionFrontier(TransitionFrontierAction::SyncLedger(value.into()))
            }
        }
    };
}

impl_into_global_action!(TransitionFrontierSyncLedgerInitAction);
impl_into_global_action!(TransitionFrontierSyncLedgerSnarkedLedgerSyncPendingAction);
impl_into_global_action!(TransitionFrontierSyncLedgerSnarkedLedgerSyncPeersQueryAction);
impl_into_global_action!(TransitionFrontierSyncLedgerSnarkedLedgerSyncPeerQueryInitAction);
impl_into_global_action!(TransitionFrontierSyncLedgerSnarkedLedgerSyncPeerQueryPendingAction);
impl_into_global_action!(TransitionFrontierSyncLedgerSnarkedLedgerSyncPeerQuerySuccessAction);
impl_into_global_action!(TransitionFrontierSyncLedgerSnarkedLedgerSyncSuccessAction);
impl_into_global_action!(TransitionFrontierSyncLedgerStagedLedgerPartsFetchInitAction);
impl_into_global_action!(TransitionFrontierSyncLedgerStagedLedgerPartsFetchPendingAction);
impl_into_global_action!(TransitionFrontierSyncLedgerStagedLedgerPartsFetchSuccessAction);
impl_into_global_action!(TransitionFrontierSyncLedgerStagedLedgerReconstructInitAction);
impl_into_global_action!(TransitionFrontierSyncLedgerStagedLedgerReconstructPendingAction);
impl_into_global_action!(TransitionFrontierSyncLedgerStagedLedgerReconstructSuccessAction);
impl_into_global_action!(TransitionFrontierSyncLedgerSuccessAction);