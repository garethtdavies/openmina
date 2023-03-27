use p2p::connection::incoming::{
    P2pConnectionIncomingAnswerSdpCreateSuccessAction, P2pConnectionIncomingFinalizeSuccessAction,
};
use p2p::connection::outgoing::{
    P2pConnectionOutgoingAnswerRecvSuccessAction, P2pConnectionOutgoingFinalizeSuccessAction,
    P2pConnectionOutgoingOfferSdpCreateSuccessAction,
};

use crate::action::CheckTimeoutsAction;
use crate::p2p::disconnection::P2pDisconnectionFinishAction;
use crate::rpc::{
    RpcActionStatsGetAction, RpcGlobalStateGetAction, RpcP2pConnectionIncomingInitAction,
    RpcP2pConnectionOutgoingInitAction, RpcRequest,
};
use crate::{Service, Store};

use super::{
    Event, EventSourceAction, EventSourceActionWithMeta, EventSourceNewEventAction,
    P2pConnectionEvent, P2pEvent,
};

pub fn event_source_effects<S: Service>(store: &mut Store<S>, action: EventSourceActionWithMeta) {
    let (action, _) = action.split();
    match action {
        EventSourceAction::ProcessEvents(_) => {
            // process max 1024 events at a time.
            for _ in 0..1024 {
                match store.service.next_event() {
                    Some(event) => {
                        store.dispatch(EventSourceNewEventAction { event });
                    }
                    None => break,
                }
            }
            store.dispatch(CheckTimeoutsAction {});
        }
        EventSourceAction::NewEvent(content) => match content.event {
            Event::P2p(e) => match e {
                P2pEvent::Connection(e) => match e {
                    P2pConnectionEvent::OfferSdpReady(peer_id, sdp) => {
                        store.dispatch(P2pConnectionOutgoingOfferSdpCreateSuccessAction {
                            peer_id,
                            sdp,
                        });
                    }
                    P2pConnectionEvent::AnswerSdpReady(peer_id, sdp) => {
                        store.dispatch(P2pConnectionIncomingAnswerSdpCreateSuccessAction {
                            peer_id,
                            sdp,
                        });
                    }
                    P2pConnectionEvent::AnswerReceived(peer_id, answer) => {
                        store.dispatch(P2pConnectionOutgoingAnswerRecvSuccessAction {
                            peer_id,
                            answer,
                        });
                    }
                    P2pConnectionEvent::Opened(peer_id) => {
                        store.dispatch(P2pConnectionOutgoingFinalizeSuccessAction { peer_id });
                        store.dispatch(P2pConnectionIncomingFinalizeSuccessAction { peer_id });
                    }
                    P2pConnectionEvent::Closed(peer_id) => {
                        store.dispatch(P2pDisconnectionFinishAction { peer_id });
                    }
                },
            },
            Event::Rpc(rpc_id, e) => match e {
                RpcRequest::GetState => {
                    store.dispatch(RpcGlobalStateGetAction { rpc_id });
                }
                RpcRequest::ActionStatsGet(query) => {
                    store.dispatch(RpcActionStatsGetAction { rpc_id, query });
                }
                RpcRequest::P2pConnectionOutgoing(opts) => {
                    store.dispatch(RpcP2pConnectionOutgoingInitAction { rpc_id, opts });
                }
                RpcRequest::P2pConnectionIncoming(opts) => {
                    store.dispatch(RpcP2pConnectionIncomingInitAction {
                        rpc_id,
                        opts: opts.clone(),
                    });
                }
            },
        },
        EventSourceAction::WaitTimeout(_) => {
            store.dispatch(CheckTimeoutsAction {});
        }
        EventSourceAction::WaitForEvents(_) => {}
    }
}