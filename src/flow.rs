struct IggyPerformanceFlow {}

impl IggyPerformanceFlow {
    pub fn new() -> Self {
        IggyPerformanceFlow {}
    }

    pub fn run(&self) {
        // checkout to master
        // save iggy-bench and scripts
        // checkout to commit or tag from cmdline
        // loop {
        //   build iggy-server
        //   run benchmarks
        //   reset to previous commit
        // }
    }

    pub fn checkout_commit_or_tag(&self) {
        println!("Checking out commit or tag");
    }

    pub fn run_benchmarks(&self) {
        println!("Running benchmarks");
    }

    pub fn ingest_results(&self) {
        println!("Ingesting results");
    }
}
