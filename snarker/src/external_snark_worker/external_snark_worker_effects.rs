use ledger::scan_state::conv::job_to_spec;
use mina_p2p_messages::v2::StateBodyHash;

use crate::external_snark_worker::ExternalSnarkWorkerPruneWorkAction;

use super::{ExternalSnarkWorkerAction, ExternalSnarkWorkerActionWithMeta};

pub fn external_snark_worker_effects<S: crate::Service>(
    store: &mut crate::Store<S>,
    action: ExternalSnarkWorkerActionWithMeta,
) {
    let (action, _) = action.split();
    match action {
        ExternalSnarkWorkerAction::Start(_) => {
            let Some(path) = store.state().config.path.as_ref().cloned() else {
                return;
            };
            if let Err(err) = store.service().start(path) {
                todo!("report error {err:?}");
            }
        }
        ExternalSnarkWorkerAction::Started(_) => {}
        ExternalSnarkWorkerAction::Kill(_) => {
            if let Err(err) = store.service().kill() {
                todo!("report error {err:?}");
            }
        }
        ExternalSnarkWorkerAction::Killed(_) => {}
        ExternalSnarkWorkerAction::Error(action) => {
            todo!("report {err:?}", err = action.error);
        }
        ExternalSnarkWorkerAction::SubmitWork(action) => {
            let job_id = &action.job_id;
            let config = &store.state().config;
            let public_key = config.public_key.clone();
            let fee = config.fee.clone();
            let Some(job) = store.state().snark_pool.get(job_id) else {
                return;
            };
            let protocol_state_body = |block_hash: StateBodyHash| {
                store
                    .state()
                    .transition_frontier
                    .best_chain
                    .iter()
                    .find_map(|block_with_hash| {
                        if block_with_hash.block.header.protocol_state.body.hash() == *block_hash {
                            Some(block_with_hash.block.header.protocol_state.body.clone())
                        } else {
                            None
                        }
                    })
                    .unwrap()
            };
            let input = job_to_spec(
                public_key.into(),
                fee,
                job.job.clone(),
                &protocol_state_body,
            );
            if let Err(_err) = store.service().submit(input) {
                // TODO report error
            }
        }
        ExternalSnarkWorkerAction::WorkResult(_) => {
            store.dispatch(ExternalSnarkWorkerPruneWorkAction {});
        }
        ExternalSnarkWorkerAction::WorkError(_) => {
            store.dispatch(ExternalSnarkWorkerPruneWorkAction {});
        }
        ExternalSnarkWorkerAction::PruneWork(_) => {
            // noop
        }
    }
}