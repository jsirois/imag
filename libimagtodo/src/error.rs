generate_error_module!(
    generate_error_types!(TodoError, TodoErrorKind,
        ConversionError     => "Conversion Error",
        StoreError          => "Store Error",
        ImpoertError        => "Error importing"
    );
);

pub use self::error::TodoError;
pub use self::error::TodoErrorKind;

