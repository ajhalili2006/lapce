use std::{
    collections::{BTreeMap, HashMap},
    fmt::Display,
    path::{Path, PathBuf},
    time::Instant,
};

use floem::{
    app::AppContext,
    reactive::{create_rw_signal, RwSignal, SignalGetUntracked},
};
use lapce_rpc::{
    dap_types::{DapId, RunDebugConfig, SourceBreakpoint, StackFrame, ThreadId},
    terminal::TermId,
};
use serde::{Deserialize, Serialize};

const DEFAULT_RUN_TOML: &str = include_str!("../../defaults/run.toml");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RunDebugMode {
    Run,
    Debug,
}

impl Display for RunDebugMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            RunDebugMode::Run => "Run",
            RunDebugMode::Debug => "Debug",
        };
        f.write_str(s)
    }
}

#[derive(Clone)]
pub struct RunDebugProcess {
    pub mode: RunDebugMode,
    pub config: RunDebugConfig,
    pub stopped: bool,
    pub created: Instant,
}

#[derive(Deserialize, Serialize)]
pub struct RunDebugConfigs {
    pub configs: Vec<RunDebugConfig>,
}

pub fn run_configs(workspace: Option<&Path>) -> Option<RunDebugConfigs> {
    let workspace = workspace?;
    let run_toml = workspace.join(".lapce").join("run.toml");
    if !run_toml.exists() {
        if !workspace.join(".lapce").exists() {
            let _ = std::fs::create_dir_all(workspace.join(".lapce"));
        }
        let _ = std::fs::write(&run_toml, DEFAULT_RUN_TOML);
        return None;
    }
    let content = std::fs::read_to_string(run_toml).ok()?;
    let configs: RunDebugConfigs = toml_edit::easy::from_str(&content).ok()?;
    Some(configs)
}

#[derive(Clone)]
pub struct RunDebugData {
    pub active_term: RwSignal<Option<TermId>>,
    pub daps: RwSignal<im::HashMap<DapId, DapData>>,
    pub breakpoints: RwSignal<BTreeMap<PathBuf, Vec<LapceBreakpoint>>>,
}

impl RunDebugData {
    pub fn new(cx: AppContext) -> Self {
        let active_term = create_rw_signal(cx.scope, None);
        let daps = create_rw_signal(cx.scope, im::HashMap::new());
        let breakpoints = create_rw_signal(cx.scope, BTreeMap::new());
        Self {
            active_term,
            daps,
            breakpoints,
        }
    }

    pub fn source_breakpoints(&self) -> HashMap<PathBuf, Vec<SourceBreakpoint>> {
        self.breakpoints
            .get_untracked()
            .iter()
            .map(|(path, breakpoints)| {
                (
                    path.to_path_buf(),
                    breakpoints
                        .iter()
                        .map(|b| SourceBreakpoint {
                            line: b.line + 1,
                            column: None,
                            condition: None,
                            hit_condition: None,
                            log_message: None,
                        })
                        .collect(),
                )
            })
            .collect()
    }
}

#[derive(Clone, PartialEq)]
pub struct StackTraceData {
    pub expanded: bool,
    pub frames: Vec<StackFrame>,
    pub frames_shown: usize,
}

#[derive(Clone)]
pub struct LapceBreakpoint {
    pub id: Option<usize>,
    pub verified: bool,
    pub message: Option<String>,
    pub line: usize,
    pub offset: usize,
    pub dap_line: Option<usize>,
}

#[derive(Clone)]
pub struct DapData {
    pub term_id: TermId,
    pub dap_id: DapId,
    pub stopped: bool,
    pub thread_id: Option<ThreadId>,
    pub stack_frames: BTreeMap<ThreadId, StackTraceData>,
}

impl DapData {
    pub fn new(dap_id: DapId, term_id: TermId) -> Self {
        Self {
            term_id,
            dap_id,
            stopped: false,
            thread_id: None,
            stack_frames: BTreeMap::new(),
        }
    }
}