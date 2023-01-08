pub mod csvmanager;
pub mod filtercsv;
pub mod person;
use csv::Reader;
use std::env;
use std::error::Error;
use std::fs::File;
pub struct Setup {
    pub file_open_csv: Reader<File>,
}

impl Setup {
    pub fn run() -> Result<Setup, Box<dyn Error>> {
        let args: Vec<String> = env::args().collect();
        let file_parameter: &String = &args[1];
        let file_open_csv = csvmanager::read_file_csv(file_parameter)?;

        Ok(Setup { file_open_csv })
    }
}
pub enum FilterCsvTypes {
    AdultAndChildren(filtercsv::FilterAdultAndChildren),
}
pub mod app {
    use super::filtercsv::FilterGeneralCsv;
    use super::FilterCsvTypes;
    pub fn run_filter_csv(setup: &mut super::Setup, filter_type: super::FilterCsvTypes) {
        match filter_type {
            FilterCsvTypes::AdultAndChildren(instance) => instance.filter_csv(setup),
        }
    }
}
