#![allow(unused_variables, dead_code)]

mod health_platform {
    pub struct PatientJournal {
        entries: Vec<String>,
    }

    impl PatientJournal {
        pub fn new(patient: &str) -> Self {
            Self {
                entries: vec![format!("Patient journal of {}", patient)],
            }
        }
    }

    pub struct JournalReading<'a> {
        journal: &'a PatientJournal,
    }

    impl<'a> JournalReading<'a> {
        pub fn new(journal: &'a PatientJournal) -> Self {
            Self { journal }
        }
        pub fn print(&self) {
            for entry in &self.journal.entries {
                println!("{}", entry);
            }
        }
    }

    pub struct JournalWriting<'a> {
        journal: &'a mut PatientJournal,
    }

    impl<'a> JournalWriting<'a> {
        pub fn new(journal: &'a mut PatientJournal) -> Self {
            Self { journal }
        }
        pub fn add_entry(&mut self, entry: &str) {
            self.journal.entries.push(entry.to_string());
        }
    }
}

use health_platform::*;

fn main() {
    let mut journal = PatientJournal::new("John Doe");

    let patient_reading = JournalReading::new(&journal);
    patient_reading.print();

    let mut doctor_writing = JournalWriting::new(&mut journal);
    doctor_writing.add_entry("Patient is feeling much better.");

    let patient_reading_2 = JournalReading::new(&journal);
    patient_reading_2.print();

    let mut doctor_writing_2 = JournalWriting::new(&mut journal);
    doctor_writing_2.add_entry("Some issues");
}
