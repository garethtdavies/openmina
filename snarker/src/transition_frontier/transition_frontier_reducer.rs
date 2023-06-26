use super::{
    sync::ledger::TransitionFrontierSyncLedgerState, TransitionFrontierAction,
    TransitionFrontierActionWithMetaRef, TransitionFrontierState, TransitionFrontierSyncState,
};

impl TransitionFrontierState {
    pub fn reducer(&mut self, action: TransitionFrontierActionWithMetaRef<'_>) {
        let (action, meta) = action.split();
        match action {
            TransitionFrontierAction::SyncInit(a) => {
                self.sync = TransitionFrontierSyncState::Init {
                    time: meta.time(),
                    best_tip: a.best_tip.clone(),
                    root_block: a.root_block.clone(),
                    missing_blocks: a.blocks_inbetween.clone(),
                };
            }
            TransitionFrontierAction::RootLedgerSyncPending(_) => {
                if let TransitionFrontierSyncState::Init {
                    best_tip,
                    root_block,
                    missing_blocks,
                    ..
                } = &mut self.sync
                {
                    self.sync = TransitionFrontierSyncState::RootLedgerSyncPending {
                        time: meta.time(),
                        best_tip: best_tip.clone(),
                        root_ledger: TransitionFrontierSyncLedgerState::Init {
                            time: meta.time(),
                            block: root_block.clone(),
                        },
                        missing_blocks: std::mem::take(missing_blocks),
                    };
                }
            }
            TransitionFrontierAction::SyncLedger(a) => match &mut self.sync {
                TransitionFrontierSyncState::RootLedgerSyncPending { root_ledger, .. } => {
                    root_ledger.reducer(meta.with_action(a));
                }
                _ => {}
            },
        }
    }
}