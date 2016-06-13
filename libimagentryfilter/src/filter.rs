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

use libimagstore::store::Entry;

pub use ops::and::And;
pub use ops::not::Not;
pub use ops::or::Or;

pub trait Filter {

    fn filter(&self, &Entry) -> bool;

    fn not(self) -> Not
        where Self: Sized + 'static
    {
        Not::new(Box::new(self))
    }

    fn or(self, other: Box<Filter>) -> Or
        where Self: Sized + 'static
    {
        Or::new(Box::new(self), other)
    }

    fn or_not(self, other: Box<Filter>) -> Or
        where Self: Sized + 'static
    {
        self.or(Box::new(Not::new(other)))
    }

    fn or3(self, other: Box<Filter>, other2: Box<Filter>) -> Or
        where Self: Sized + 'static
    {
        Or::new(Box::new(self), Box::new(Or::new(other, other2)))
    }

    fn and(self, other: Box<Filter>) -> And
        where Self: Sized + 'static
    {
        And::new(Box::new(self), other)
    }

    fn and3(self, other: Box<Filter>, other2: Box<Filter>) -> And
        where Self: Sized + 'static
    {
        And::new(Box::new(self), Box::new(And::new(other, other2)))
    }

    fn and_not(self, other: Box<Filter>) -> And
        where Self: Sized + 'static
    {
        self.and(Box::new(Not::new(other)))
    }

}

