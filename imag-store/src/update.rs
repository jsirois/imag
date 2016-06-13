/*
 *  imag - The personal information management suite for the commandline
 *  Copyright (C) 2016    Matthias Beyer <mail@beyermatthias.de>
 *
 *  This library is free software; you can redistribute it and/or
 *  modify it under the terms of the GNU Lesser General Public
 *  License as published by the Free Software Foundation; version
 *  2.1 of the License.
 *
 *  This library is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 *  Lesser General Public License for more details.
 *
 *  You should have received a copy of the GNU Lesser General Public
 *  License along with this library; if not, write to the Free Software
 *  Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
 */

use std::ops::DerefMut;
use std::process::exit;

use libimagrt::runtime::Runtime;
use libimagstore::storeid::build_entry_path;
use libimagerror::trace::trace_error;

use util::build_toml_header;

pub fn update(rt: &Runtime) {
    rt.cli()
        .subcommand_matches("update")
        .map(|scmd| {
            scmd.value_of("id")
                .map(|id| {
                    let path = build_entry_path(rt.store(), id);
                    if path.is_err() {
                        trace_error(&path.unwrap_err());
                        exit(1);
                    }
                    let path = path.unwrap();

                    rt.store()
                        .retrieve(path)
                        .map(|mut locked_e| {
                            let mut e = locked_e.deref_mut();

                            scmd.value_of("content")
                                .map(|new_content| {
                                    *e.get_content_mut() = String::from(new_content);
                                    debug!("New content set");
                                });

                            *e.get_header_mut() = build_toml_header(scmd, e.get_header().clone());
                            debug!("New header set");
                        })
                })
        });

}

