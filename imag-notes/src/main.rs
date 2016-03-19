extern crate clap;
#[macro_use] extern crate log;
extern crate semver;
#[macro_use] extern crate version;

extern crate libimagnotes;
extern crate libimagrt;
extern crate libimagtag;
extern crate libimagutil;

use std::process::exit;

use libimagrt::runtime::Runtime;
use libimagnotes::note::Note;
use libimagutil::trace::trace_error;

mod ui;
use ui::build_ui;

fn main() {
    let name = "imag-notes";
    let version = &version!()[..];
    let about = "Note taking helper";
    let ui = build_ui(Runtime::get_default_cli_builder(name, version, about));
    let rt = {
        let rt = Runtime::new(ui);
        if rt.is_ok() {
            rt.unwrap()
        } else {
            println!("Could not set up Runtime");
            println!("{:?}", rt.err().unwrap());
            exit(1);
        }
    };

    debug!("Hello. Logging was just enabled");
    debug!("I already set up the Runtime object and build the commandline interface parser.");
    debug!("Lets get rollin' ...");

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

fn create(rt: &Runtime) {
    let name = rt.cli()
        .subcommand_matches("create")
        .unwrap() // we already know it is there
        .value_of("name")
        .unwrap(); // enforced by clap

    Note::new(rt.store(), String::from(name), String::new())
        .map_err(|e| trace_error(&e))
        .map(|note| {
            // call editor now...
        });
}

fn delete(rt: &Runtime) {
    let name = rt.cli()
        .subcommand_matches("delete")
        .unwrap() // we already know it is there
        .value_of("name")
        .unwrap(); // enforced by clap

    Note::delete(rt.store(), String::from(name))
        .map_err(|e| trace_error(&e))
        .map(|_| println!("Ok"));
}

fn edit(rt: &Runtime) {
    unimplemented!()
}

fn list(rt: &Runtime) {
    use std::cmp::Ordering;

    let iter = Note::all_notes(rt.store());
    if iter.is_err() {
        trace_error(&iter.err().unwrap());
        exit(1);
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
        note.get_name().map(|name| println!("{}", name));
    }
}

