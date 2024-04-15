use crate::model::day::Day;
use crate::model::year::{Year, YearDataFile};
use chrono::{Datelike, Local, NaiveDate};
use itertools::Itertools;
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

type YearNum = i32;

pub struct Data {
    database: PathBuf,
}

impl Data {
    pub fn new(database: PathBuf) -> Self {
        Self { database }
    }

    pub fn day(&self, date: &NaiveDate) -> Day {
        let year = self.year(date.year());
        match year.get(date) {
            Some(day) => day.clone(),
            None => Day::default(),
        }
    }

    pub fn set_day(&mut self, date: NaiveDate, day: Day) {
        // TODO: This opens the file twice, for reading and writing. Should be able to do it in one file open which could improve performance.
        // TODO: Should come up with a performance testing approach.
        let year_num = date.year();
        let mut year = self.year(date.year());
        year.insert(date, day);
        self.set_year(&year_num, year);
    }

    fn set_year(&mut self, year_num: &YearNum, year: Year) {
        let year_file = self.year_file(year_num);
        let year_data = YearDataFile::new(year);

        // TODO: Handle errors
        let f = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(year_file)
            .unwrap();
        let writer = BufWriter::new(f);
        serde_json::to_writer(writer, &year_data).unwrap();
    }

    fn year(&self, year: YearNum) -> Year {
        let year_file = self.year_file(&year);
        if !year_file.exists() {
            return Year::default();
        }

        // TODO: Handle errors
        let f = File::open(year_file).unwrap();
        let reader = BufReader::new(f);
        let year_data: YearDataFile = serde_json::from_reader(reader).unwrap();
        year_data.days
    }

    pub fn last_date(&self) -> Option<NaiveDate> {
        // TODO: It's possible a last year file could exist but be empty. Should check back through other years in that case.
        let year = self.year(self.last_year()?);
        // TODO: Put a today() function in a utility file somewhere
        let today = Local::now().naive_local().date();

        for date in year.keys().sorted().rev() {
            if date <= &today {
                return Some(*date);
            }
        }

        None
    }

    // TODO: Look at globset or walkdir crates for better file globbing
    fn last_year(&self) -> Option<YearNum> {
        // TODO: Handle errors
        let years = fs::read_dir(&self.database)
            .unwrap()
            .filter_map(|entry| {
                let entry = entry.unwrap();
                let file_name = entry.file_name();
                let file_name = file_name.to_str().unwrap();
                file_name
                    .strip_suffix(".json")
                    .map(|year| year.parse::<YearNum>().unwrap())
            })
            .collect::<Vec<YearNum>>();

        // TODO: Handle the date being later than current date/year
        years.into_iter().max()
    }

    fn year_file(&self, year: &YearNum) -> PathBuf {
        self.database.join(format!("{year}.json"))
    }
}
