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

use std::io::{Stdout, stdout};

use toml::encode_str;

use viewer::{ViewInformation, Viewer};

pub struct StdoutViewer {
    out: Stdout,
}

impl StdoutViewer {

    pub fn new() -> StdoutViewer {
        StdoutViewer { out: stdout() }
    }

}

impl Viewer for StdoutViewer {

    fn view(&self, vi: ViewInformation) {
        if vi.view_copy {
            unimplemented!();
        }

        if vi.view_header {
            println!("{}", encode_str(vi.entry.get_header().header()));
        }

        if vi.view_content {
            println!("{}", vi.entry.get_content());
        }

        if vi.view_copy && !vi.keep_copy {
            unimplemented!()
        }
    }

}
