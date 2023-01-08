use super::csvmanager;
use super::person::Person;
use super::Setup;
use csv::StringRecord;
pub trait FilterGeneralCsv {
    fn filter_csv(&self, setup: &mut Setup);
}

pub struct FilterAdultAndChildren {}

impl FilterGeneralCsv for FilterAdultAndChildren {
    fn filter_csv(&self, setup: &mut Setup) {
        for result in setup.file_open_csv.records() {
            let record: StringRecord = result.expect("Error read data csv");
            let person = Person::build_csv(&record);

            let path = if person.is_adult() {
                "./adult.csv"
            } else {
                "./children.csv"
            };

            csvmanager::write_file_csv(&path, &record).unwrap_or_else(|err| eprintln!("{err}"));
        }
    }
}
