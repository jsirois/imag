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

generate_error_imports!();
use std::convert::From;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Copy)]
pub struct CustomErrorData {}

generate_custom_error_types!(StoreError, StoreErrorKind, CustomErrorData,
    ConfigurationError      => "Store Configuration Error",
    FileError               => "File Error",
    IoError                 => "IO Error",
    IdLocked                => "ID locked",
    IdNotFound              => "ID not found",
    OutOfMemory             => "Out of Memory",
    FileNotFound            => "File corresponding to ID not found",
    FileNotCreated          => "File corresponding to ID could not be created",
    StorePathExists         => "Store path exists",
    StorePathCreate         => "Store path create",
    LockError               => "Error locking datastructure",
    LockPoisoned            => "The internal Store Lock has been poisoned",
    EntryAlreadyBorrowed    => "Entry is already borrowed",
    EntryAlreadyExists      => "Entry already exists",
    MalformedEntry          => "Entry has invalid formatting, missing header",
    HeaderPathSyntaxError   => "Syntax error in accessor string",
    HeaderPathTypeFailure   => "Header has wrong type for path",
    HeaderKeyNotFound       => "Header Key not found",
    HeaderTypeFailure       => "Header type is wrong",
    HookRegisterError       => "Hook register error",
    AspectNameNotFoundError => "Aspect name not found",
    HookExecutionError      => "Hook execution error",
    PreHookExecuteError     => "Pre-Hook execution error",
    PostHookExecuteError    => "Post-Hook execution error",
    StorePathLacksVersion   => "The supplied store path has no version part",
    GlobError               => "glob() error",
    EncodingError           => "Encoding error",
    StorePathError          => "Store Path error",
    EntryRenameError        => "Entry rename error",

    CreateCallError            => "Error when calling create()",
    RetrieveCallError          => "Error when calling retrieve()",
    GetCallError               => "Error when calling get()",
    GetAllVersionsCallError    => "Error when calling get_all_versions()",
    RetrieveForModuleCallError => "Error when calling retrieve_for_module()",
    UpdateCallError            => "Error when calling update()",
    RetrieveCopyCallError      => "Error when calling retrieve_copy()",
    DeleteCallError            => "Error when calling delete()",
    MoveCallError              => "Error when calling move()",
    MoveByIdCallError          => "Error when calling move_by_id()"
);

generate_custom_error_types!(ParserError, ParserErrorKind, CustomErrorData,
    TOMLParserErrors    => "Several TOML-Parser-Errors",
    MissingMainSection  => "Missing main section",
    MissingVersionInfo  => "Missing version information in main section",
    NonTableInBaseTable => "A non-table was found in the base table",
    HeaderInconsistency => "The header is inconsistent"
);

impl From<ParserError> for StoreError {
    fn from(ps: ParserError) -> StoreError {
        StoreError {
            err_type: StoreErrorKind::MalformedEntry,
            cause: Some(Box::new(ps)),
            custom_data: None,
        }
    }
}

impl From<::std::io::Error> for StoreError {
    fn from(ps: ::std::io::Error) -> StoreError {
        StoreError {
            err_type: StoreErrorKind::IoError,
            cause: Some(Box::new(ps)),
            custom_data: None,
        }
    }
}

