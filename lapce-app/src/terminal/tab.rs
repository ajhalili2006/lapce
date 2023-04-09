use std::sync::Arc;

use floem::{
    app::AppContext,
    reactive::{
        create_rw_signal, RwSignal, SignalGet, SignalGetUntracked, SignalWith,
        SignalWithUntracked,
    },
};

use crate::{
    debug::RunDebugProcess, id::TerminalTabId, window_tab::CommonData,
    workspace::LapceWorkspace,
};

use super::data::TerminalData;

#[derive(Clone)]
pub struct TerminalTabData {
    pub terminal_tab_id: TerminalTabId,
    pub active: RwSignal<usize>,
    pub terminals: RwSignal<im::Vector<(RwSignal<usize>, TerminalData)>>,
}

impl TerminalTabData {
    pub fn new(
        cx: AppContext,
        workspace: Arc<LapceWorkspace>,
        run_debug: Option<RunDebugProcess>,
        common: CommonData,
    ) -> Self {
        let terminal_data = TerminalData::new(cx, workspace, run_debug, common);
        let terminals = im::vector![(create_rw_signal(cx.scope, 0), terminal_data)];
        let terminals = create_rw_signal(cx.scope, terminals);
        let active = create_rw_signal(cx.scope, 0);
        let terminal_tab_id = TerminalTabId::next();
        Self {
            terminal_tab_id,
            active,
            terminals,
        }
    }

    pub fn active_terminal(&self, tracked: bool) -> Option<TerminalData> {
        let active = if tracked {
            self.active.get()
        } else {
            self.active.get_untracked()
        };

        if tracked {
            self.terminals
                .with(|t| t.get(active).or_else(|| t.last()).cloned())
                .map(|(_, t)| t)
        } else {
            self.terminals
                .with_untracked(|t| t.get(active).or_else(|| t.last()).cloned())
                .map(|(_, t)| t)
        }
    }
}