use csvread::app::run_filter_csv;
use csvread::filtercsv::FilterAdultAndChildren;
use csvread::FilterCsvTypes;
use csvread::Setup;
use std::process;
fn main() {
    let mut setup = Setup::run().unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });
    run_filter_csv(
        &mut setup,
        FilterCsvTypes::AdultAndChildren(FilterAdultAndChildren {}),
    );
}
