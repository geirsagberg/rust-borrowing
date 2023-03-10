struct PatientJournal {
    entries: Vec<String>,
}

impl PatientJournal {
    fn new(patient: &str) -> Self {
        Self {
            entries: vec![format!("Patient journal of {}", patient)],
        }
    }
}

struct JournalReading<'a> {
    journal: &'a PatientJournal,
}

impl JournalReading<'_> {
    fn print(&self) {
        for entry in &self.journal.entries {
            println!("{}", entry);
        }
    }
}

struct JournalWriting<'a> {
    journal: &'a mut PatientJournal,
}

impl JournalWriting<'_> {
    fn add_entry(&mut self, entry: &str) {
        self.journal.entries.push(entry.to_string());
    }
}

fn main() {}
