//This is the main slide. This is where all the explanations will be run.

//Here are all of the slides: To call a function in another file, you must prefix the file name with mod,
mod variables;
mod data_types;
mod functions;
mod control_flow;

fn main() {
    //then call the function inside using ::funcName()
    //variables::explain();
    //data_types::explain();
    // functions::explain();
    control_flow::explain();
}
