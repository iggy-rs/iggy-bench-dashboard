use gloo::console::log;
use iggy_benchmark_report::benchmark_kind::BenchmarkKind;
use shared::BenchmarkReportLight;
use std::{collections::BTreeMap, rc::Rc};
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct BenchmarkState {
    pub entries: BTreeMap<BenchmarkKind, Vec<BenchmarkReportLight>>,
    pub selected_benchmark: Option<BenchmarkReportLight>,
    pub selected_kind: BenchmarkKind,
}

impl BenchmarkState {
    /// Finds a benchmark that matches the parameters of the given benchmark
    pub fn find_matching_benchmark(
        &self,
        benchmark: &BenchmarkReportLight,
    ) -> Option<BenchmarkReportLight> {
        for benchmarks in self.entries.values() {
            if let Some(matching) = benchmarks.iter().find(|b| {
                b.params.message_size == benchmark.params.message_size
                    && b.params.message_batches == benchmark.params.message_batches
                    && b.params.messages_per_batch == benchmark.params.messages_per_batch
                    && b.params.transport == benchmark.params.transport
                    && b.params.remark == benchmark.params.remark
            }) {
                return Some(matching.clone());
            }
        }
        None
    }

    /// Gets the first available benchmark from any kind
    pub fn get_first_available(&self) -> Option<BenchmarkReportLight> {
        self.entries
            .values()
            .next()
            .and_then(|benchmarks| benchmarks.first().cloned())
    }
}

pub enum BenchmarkAction {
    SelectBenchmark(Box<Option<BenchmarkReportLight>>),
    SelectBenchmarkKind(BenchmarkKind),
    SetBenchmarksForGitref(Vec<BenchmarkReportLight>),
}

#[derive(Clone, PartialEq)]
pub struct BenchmarkContext {
    pub state: BenchmarkState,
    pub dispatch: Callback<BenchmarkAction>,
}

impl BenchmarkContext {
    pub fn new(state: BenchmarkState, dispatch: Callback<BenchmarkAction>) -> Self {
        Self { state, dispatch }
    }
}

impl Reducible for BenchmarkState {
    type Action = BenchmarkAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let next_state = match action {
            BenchmarkAction::SelectBenchmark(benchmark) => {
                log!(format!(
                    "Benchmark selected: {:?}",
                    benchmark.as_ref().clone().map(|b| b.uuid)
                ));
                BenchmarkState {
                    selected_benchmark: *benchmark,
                    ..(*self).clone()
                }
            }
            BenchmarkAction::SelectBenchmarkKind(kind) => {
                log!(format!("Kind changed: {:?}", kind));

                // First create a state with cleared selection to avoid showing old data
                let mut next_state = BenchmarkState {
                    selected_kind: kind,
                    selected_benchmark: None,
                    entries: self.entries.clone(),
                };

                // Then try to find a matching benchmark with the same parameters
                if let Some(current) = &self.selected_benchmark {
                    if let Some(benchmarks) = self.entries.get(&kind) {
                        let matching = benchmarks.iter().find(|b| {
                            b.params.message_size == current.params.message_size
                                && b.params.message_batches == current.params.message_batches
                                && b.params.messages_per_batch == current.params.messages_per_batch
                                && b.params.transport == current.params.transport
                                && b.params.remark == current.params.remark
                        });

                        if matching.is_none() {
                            log!("No matching benchmark found with the same parameters, selecting first available entry");
                            next_state.selected_benchmark = benchmarks.first().cloned();
                        } else {
                            log!("Matching benchmark found");
                            next_state.selected_benchmark = matching.cloned();
                        }
                    }
                }

                next_state
            }
            BenchmarkAction::SetBenchmarksForGitref(benchmarks) => {
                log!(format!("Loaded {} benchmarks for gitref", benchmarks.len()));

                // Group benchmarks by kind
                let mut entries = BTreeMap::new();
                for benchmark in benchmarks {
                    entries
                        .entry(benchmark.params.benchmark_kind)
                        .or_insert_with(Vec::new)
                        .push(benchmark);
                }

                // Try to maintain current benchmark selection if possible
                let selected_benchmark = if let Some(current) = &self.selected_benchmark {
                    log!("Attempting to find matching benchmark after gitref switch");
                    // First try to find exact match
                    let state = BenchmarkState {
                        entries: entries.clone(),
                        selected_benchmark: None,
                        selected_kind: self.selected_kind,
                    };

                    state.find_matching_benchmark(current).or_else(|| {
                        // If no exact match, try to find benchmark of same kind
                        log!("No exact match found, looking for benchmark of same kind");
                        entries
                            .get(&self.selected_kind)
                            .and_then(|benchmarks| benchmarks.first().cloned())
                            .or_else(|| {
                                // If no benchmark of same kind, use first available
                                log!("No benchmark of same kind found, using first available");
                                state.get_first_available()
                            })
                    })
                } else {
                    // No current selection, try to get first benchmark of current kind
                    entries
                        .get(&self.selected_kind)
                        .and_then(|benchmarks| benchmarks.first().cloned())
                        .or_else(|| {
                            // If no benchmark of current kind, use first available
                            BenchmarkState {
                                entries: entries.clone(),
                                selected_benchmark: None,
                                selected_kind: self.selected_kind,
                            }
                            .get_first_available()
                        })
                };

                if selected_benchmark.is_some() {
                    log!("Found suitable benchmark to display");
                } else {
                    log!("No suitable benchmark found");
                }

                BenchmarkState {
                    entries,
                    selected_benchmark,
                    selected_kind: self.selected_kind,
                }
            }
        };

        Rc::new(next_state)
    }
}

#[derive(Properties, PartialEq)]
pub struct BenchmarkProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(BenchmarkProvider)]
pub fn benchmark_provider(props: &BenchmarkProviderProps) -> Html {
    let state = use_reducer(BenchmarkState::default);

    let context = BenchmarkContext::new(
        (*state).clone(),
        Callback::from(move |action| state.dispatch(action)),
    );

    html! {
        <ContextProvider<BenchmarkContext> context={context}>
            { for props.children.iter() }
        </ContextProvider<BenchmarkContext>>
    }
}

#[hook]
pub fn use_benchmark() -> BenchmarkContext {
    use_context::<BenchmarkContext>().expect("Benchmark context not found")
}
