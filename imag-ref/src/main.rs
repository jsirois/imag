#[macro_use] extern crate log;
#[macro_use] extern crate version;
extern crate semver;
extern crate clap;

extern crate libimagstore;
extern crate libimagrt;
extern crate libimagref;
extern crate libimagerror;
extern crate libimagentrylist;

mod ui;
use ui::build_ui;

use std::path::PathBuf;

use libimagref::reference::Ref;
use libimagref::flags::RefFlags;
use libimagerror::trace::trace_error;
use libimagrt::setup::generate_runtime_setup;
use libimagrt::runtime::Runtime;

fn main() {
    let rt = generate_runtime_setup("imag-ref",
                                    &version!()[..],
                                    "Reference files outside of the store",
                                    build_ui);
    rt.cli()
        .subcommand_name()
        .map(|name| {
            debug!("Call: {}", name);
            match name {
                "add"    => add(&rt),
                "remove" => remove(&rt),
                "list"   => list(&rt),
                _        => {
                    debug!("Unknown command"); // More error handling
                },
            };
        });
}

fn add(rt: &Runtime) {
    let cmd  = rt.cli().subcommand_matches("add").unwrap();
    let path = cmd.value_of("path").map(PathBuf::from).unwrap(); // saved by clap

    let flags = RefFlags::default()
        .with_content_hashing(cmd.is_present("track-content"))
        .with_permission_tracking(cmd.is_present("track-permissions"));

    Ref::create(rt.store(), path, flags)
        .map(|r| debug!("Reference created: {:?}", r))
        .map(|_| info!("Ok"))
        .map_err(|e| {
            trace_error(&e);
            warn!("Failed to create reference");
        });
}

fn remove(rt: &Runtime) {
    use libimagref::error::RefErrorKind;
    use libimagerror::into::IntoError;

    let cmd  = rt.cli().subcommand_matches("remove").unwrap();
    let hash = cmd.value_of("hash").map(String::from).unwrap(); // saved by clap
    let yes  = cmd.is_present("yes");

    Ref::get_by_hash(rt.store(), hash)
        .and_then(|r| {
            match r {
                Some(r) => {
                    debug!("We have a Ref object: {:?}", r);
                    r.delete(rt.store())
                },

                None => Err(RefErrorKind::RefNotInStore.into_error()),
            }
        })
        .map_err(|e| trace_error(&e))
        .map(|_| info!("Ok"));
}

fn list(rt: &Runtime) {
    use std::process::exit;

    use libimagentrylist::lister::Lister;
    use libimagentrylist::listers::core::CoreLister;

    let cmd                      = rt.cli().subcommand_matches("list").unwrap();
    let do_check_dead            = cmd.is_present("check-dead");
    let do_check_changed         = cmd.is_present("check-changed");
    let do_check_changed_content = cmd.is_present("check-changed-content");
    let do_check_changed_permiss = cmd.is_present("check-changed-permissions");

    let iter = match rt.store().retrieve_for_module("ref") {
        Ok(iter) => iter,
        Err(e) => {
            trace_error(&e);
            exit(1);
        }
    };

    fn check_dead(r: &Ref) -> bool {
        match r.fs_link_exists() {
            Ok(b)  => b,
            Err(e) => {
                warn!("Could not check whether the ref {} exists on the FS:", r);
                trace_error(&e);

                // We continue here and tell the callee that this reference is dead, what is kind of
                // true actually, as we might not have access to it right now
                true
            },
        }
    }

    fn check_changed(r: &Ref) -> bool {
        check_changed_content(r) && check_changed_permiss(r)
    }

    fn check_changed_content(r: &Ref) -> bool {
        let eq = r.get_current_hash()
            .and_then(|hash| r.get_stored_hash().map(|stored| (hash, stored)))
            .map(|(hash, stored)| hash == stored);

        match eq {
            Ok(eq) => eq,
            Err(e) => {
                warn!("Could not check whether the ref {} changed on the FS:", r);
                trace_error(&e);

                // We continue here and tell the callee that this reference is unchanged
                false
            },
        }
    }

    fn check_changed_permiss(r: &Ref) -> bool {
        warn!("Permission changes tracking not supported yet.");
        false
    }

    let lister_fn = |fle| {
        Ref::from_filelockentry(fle) // should succeed here
            .map(|r| {
                let is_dead = if do_check_dead {
                    if check_dead(&r) { "dead" } else { "alive" }
                } else {
                    "not checked"
                };

                let is_changed = if do_check_changed {
                    if check_changed(&r) { "changed" } else { "unchanged" }
                } else {
                    "not checked"
                };

                let is_changed_content = if do_check_changed_content {
                    if check_changed_content(&r) { "changed" } else { "unchanged" }
                } else {
                    "not checked"
                };

                let is_changed_permiss = if do_check_changed_permiss {
                    if check_changed_permiss(&r) { "changed" } else { "unchanged" }
                } else {
                    "not checked"
                };

                format!("{} | {} | {} | {} | {} | {}",
                        is_dead,
                        is_changed,
                        is_changed_content,
                        is_changed_permiss,
                        r.get_path_hash().unwrap_or_else(|| String::from("Cannot get hash")),
                        r.get_location())
            }).unwrap_or_else(|e| format!("Error: {:?}", e))
    };

    let iter = iter.filter_map(|id| {
            match Ref::get(rt.store(), id) {
                Ok(r) => Some(r),
                Err(e) => {
                    trace_error(&e);
                    None
                },
            }
        });

    unimplemented!()

}

