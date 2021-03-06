generate_error_module!(
    generate_error_types!(BookmarkError, BookmarkErrorKind,
        StoreReadError     => "Store read error",
        LinkError          => "Link error",
        LinkParsingError   => "Link parsing error",
        LinkingError       => "Error while linking",
        CollectionNotFound => "Link-Collection not found"
    );
);

pub use self::error::BookmarkError;
pub use self::error::BookmarkErrorKind;
pub use self::error::MapErrInto;

