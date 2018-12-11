extern crate flate2;
extern crate tar;

use std::path::Path;
use std::io::Error;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

pub struct Notebook {
    pub name: String,
    notes: HashMap<String, String>,
    // is_encrypted: bool,
}

impl Notebook {
    // Get a list of the titles of all of the notes within this notebook
    pub fn get_note_titles(&self) -> Vec<&String> {
        let mut titles = vec![];
        for (title, _) in &self.notes {
            titles.push(title);
        }

        return titles;
    }

    // get the body content of a specific note in markdown format
    pub fn get_note_body(&self, note: String) -> String {
        let body = match self.notes.get(&note) {
            Some(body) => body.clone(),
            None => String::new(),
        };

        return body;
    }
}

// Gets a new notebook of the note archive
pub fn get_notebook(p: &str) -> Result<Notebook, Error> {
    let path = Path::new(p);
    let name = path.file_name().unwrap();
    let tar_gz = File::open(p)?;
    let tar = flate2::read::GzDecoder::new(tar_gz);

    let mut archive = tar::Archive::new(tar);

    let mut notes: HashMap<String, String> = HashMap::new();

    for f in archive.entries().unwrap() {
        // Make sure there wasn't an I/O error
        let mut f = f.unwrap();

        // Inspect metadata about the file
        let title = f.header()
            .path()
            .unwrap()
            .into_owned()
            .to_string_lossy()
            .to_string();

        // files implement the Read trait
        let mut body = String::new();
        f.read_to_string(&mut body).unwrap();

        notes.insert(title, body);
    }

    let notebook = Notebook {
        name: name.to_os_string().into_string().unwrap(),
        notes: notes,
        // is_encrypted: true,
    };

    Ok(notebook)
}

// gets a single note and returns its contents
// pub fn getNote(note: &str) {
// }

// encypts a notebook
// fn encryptNotebook(notebook: Notebook, password: String) {
// }

// decrypts a notebook and returns its contents to the user
// fn decyptNotbook(notebook: Notebook, password: String) {

// }
