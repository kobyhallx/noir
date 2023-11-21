use noir::lib::types::{DependencyGraph, CompileResult, CompileError};

wit_bindgen::generate!({
    // the name of the world in the `*.wit` input file
    world: "noir-lib",

    // For all exported worlds, interfaces, and resources, this specifies what
    // type they're corresponding to in this module. In this case the `MyHost`
    // struct defined below is going to define the exports of the `world`,
    // namely the `run` function.
    exports: {
        world: NoirLib,
    },
});

struct NoirLib;

impl Guest for NoirLib {

    fn compile(
        entry_point: String,
        contracts: Option<bool>,
        dependency_graph: Option<DependencyGraph>,
    ) -> Result<CompileResult, CompileError> {
    }
    
}
