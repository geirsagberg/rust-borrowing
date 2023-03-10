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

    let patient_read_journal = JournalReading::new(&journal);
    patient_read_journal.print();

    let doctor_read_journal = JournalReading::new(&journal);
    doctor_read_journal.print();

    {
        let mut doctor_write_journal = JournalWriting::new(&mut journal);
        doctor_write_journal.add_entry("Patient is feeling better");
    }

    let patient_read_journal = JournalReading::new(&journal);
    patient_read_journal.print();
}
