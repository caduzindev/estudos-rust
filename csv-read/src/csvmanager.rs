use csv::Reader;
use csv::StringRecord;
use csv::Writer;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Error;

pub fn read_file_csv(path: &str) -> Result<Reader<File>, Error> {
    let file_open_csv = File::open(path)?;
    let data = Reader::from_reader(file_open_csv);
    Ok(data)
}

pub fn write_file_csv(path: &str, record: &StringRecord) -> Result<(), Error> {
    let file_choose = OpenOptions::new().append(true).open(path)?;
    let mut wrt = Writer::from_writer(file_choose);
    wrt.write_record(record).expect("File csv write");
    wrt.flush()?;

    Ok(())
}
