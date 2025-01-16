use shared::{BenchmarkDetails, BenchmarkInfoFromDirectoryName};
use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct BenchmarkState {
    pub benchmarks: Vec<BenchmarkInfoFromDirectoryName>,
    pub selected_benchmark: Option<String>,
    pub benchmark_info: Option<BenchmarkDetails>,
}

pub enum BenchmarkAction {
    SetBenchmarks(Vec<BenchmarkInfoFromDirectoryName>),
    SelectBenchmark(String),
    SetBenchmarkInfo(Box<Option<BenchmarkDetails>>),
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
            BenchmarkAction::SetBenchmarks(benchmarks) => {
                // Keep selection and info if the selected benchmark still exists
                let keep_selection = self
                    .selected_benchmark
                    .as_ref()
                    .is_some_and(|current| benchmarks.iter().any(|b| b.name == *current));

                BenchmarkState {
                    benchmarks,
                    selected_benchmark: if keep_selection {
                        self.selected_benchmark.clone()
                    } else {
                        None
                    },
                    benchmark_info: if keep_selection {
                        self.benchmark_info.clone()
                    } else {
                        None
                    },
                }
            }
            BenchmarkAction::SelectBenchmark(benchmark) => BenchmarkState {
                selected_benchmark: Some(benchmark),
                ..(*self).clone()
            },
            BenchmarkAction::SetBenchmarkInfo(info) => BenchmarkState {
                benchmark_info: *info,
                ..(*self).clone()
            },
        };

        next_state.into()
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
