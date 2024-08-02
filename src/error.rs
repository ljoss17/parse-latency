use flex_error::define_error;
use flex_error::DisplayOnly;
use serde_json::Value;

define_error! {
    #[derive(Debug, Clone, PartialEq, Eq)]
    Error {
        Io
        [ DisplayOnly<std::io::Error> ]
        | _ | { "I/O error" },

        MissingProfilingFile
        | _ | { "missing profiling file" },

        ParseToObject
        { serde_value: Value }
        | e | { format_args!("failed to parse Value `{}` to Object", e.serde_value) },

        ParseToU64
        { serde_value: Value }
        | e | { format_args!("failed to parse Value `{}` to u64", e.serde_value) },

        SerdeParse
        [ DisplayOnly<serde_json::Error> ]
        | _ | { "serde parse error" },
    }
}
