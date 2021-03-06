extern crate clap;
#[macro_use] extern crate log;
extern crate semver;
#[macro_use] extern crate version;

extern crate libimagnotes;
extern crate libimagrt;
extern crate libimagentrytag;
extern crate libimagerror;

use std::process::exit;

use libimagrt::edit::Edit;
use libimagrt::runtime::Runtime;
use libimagrt::setup::generate_runtime_setup;
use libimagnotes::note::Note;
use libimagerror::trace::{trace_error, trace_error_exit};

mod ui;
use ui::build_ui;

fn main() {
    let rt = generate_runtime_setup("imag-notes",
                                    &version!()[..],
                                    "Note taking helper",
                                    build_ui);

    rt.cli()
        .subcommand_name()
        .map(|name| {
            debug!("Call: {}", name);
            match name {
                "create" => create(&rt),
                "delete" => delete(&rt),
                "edit"   => edit(&rt),
                "list"   => list(&rt),
                _        => {
                    debug!("Unknown command"); // More error handling
                },
            };
        });
}

fn name_from_cli(rt: &Runtime, subcmd: &str) -> String {
    rt.cli().subcommand_matches(subcmd).unwrap().value_of("name").map(String::from).unwrap()
}

fn create(rt: &Runtime) {
    let name = name_from_cli(rt, "create");
    Note::new(rt.store(), name.clone(), String::new())
        .map_err(|e| trace_error(&e))
        .ok();

    if rt.cli().subcommand_matches("create").unwrap().is_present("edit") &&
            !edit_entry(rt, name) {
        exit(1);
    }
}

fn delete(rt: &Runtime) {
    Note::delete(rt.store(), String::from(name_from_cli(rt, "delete")))
        .map_err(|e| trace_error(&e))
        .map(|_| println!("Ok"))
        .ok();
}

fn edit(rt: &Runtime) {
    edit_entry(rt, name_from_cli(rt, "edit"));
}

fn edit_entry(rt: &Runtime, name: String) -> bool {
    let mut note = match Note::get(rt.store(), name) {
        Ok(Some(note)) => note,
        Ok(None) => {
            warn!("Cannot edit nonexistent Note");
            return false
        },
        Err(e) => {
            trace_error(&e);
            warn!("Cannot edit nonexistent Note");
            return false
        },
    };

    if let Err(e) = note.edit_content(rt) {
        trace_error(&e);
        warn!("Editing failed");
        return false
    }
    true
}

fn list(rt: &Runtime) {
    use std::cmp::Ordering;

    let iter = Note::all_notes(rt.store());
    if iter.is_err() {
        trace_error_exit(&iter.unwrap_err(), 1);
    }

    let mut iter = iter.unwrap()
        .filter_map(|note| {
            match note {
                Err(e) => {
                    trace_error(&e);
                    None
                },
                Ok(e) => Some(e)
            }
        })
        .collect::<Vec<Note>>();

    iter.sort_by(|note_a, note_b| {
        if let (Ok(a), Ok(b)) = (note_a.get_name(), note_b.get_name()) {
            return a.cmp(&b)
        } else {
            return Ordering::Greater;
        }
    });

    for note in iter {
        note.get_name()
            .map(|name| println!("{}", name))
            .map_err(|e| trace_error(&e))
            .ok();
    }
}

