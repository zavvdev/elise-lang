// ==========================
//
// FILE EXTS START
//
// ==========================

pub const FILE_EXT_SOURCE_CODE: &[&str] = &[".eli"];
pub const FILE_EXT_EXECUTABLE: &[&str] = &[".elc"];
pub const FILE_EXT_DATA_SCHEMA: &[&str] = &[".elt"];
pub const FILE_EXT_DATA: &[&str] = &[".csv"];

// ==========================
//
// FILE EXTS END
//
// ==========================

// ==========================
//
// ARGUMENT FLAGS START
//
// ==========================

pub const ARG_FLAG_MODE: &str = "mode";
pub const ARG_FLAG_SOURCE_CODE: &str = "source-code";
pub const ARG_FLAG_DATA: &str = "data";
pub const ARG_FLAG_DATA_SCHEMA: &str = "data-schema";
pub const ARG_FLAG_EXECUTABLE: &str = "executable";
pub const ARG_FLAG_OUTPUT: &str = "output";
pub const ARG_FLAG_UNSAFE_ASSUME_VALID: &str = "unsafe-assume-valid";
pub const ARG_FLAG_PRINT_BYTECODE: &str = "print-bytecode";

// ==========================
//
// ARGUMENT FLAGS END
//
// ==========================

// ==========================
//
// ARGUMENT VALUES START
//
// ==========================

pub const ARG_V_BOOL_TRUE: &str = "true";
pub const ARG_V_BOOL_FALSE: &str = "false";
pub const ARG_V_MODE_RUN: &str = "run";
pub const ARG_V_MODE_BUILD: &str = "build";
pub const ARG_V_MODE_EXEC: &str = "exec";
pub const ARG_V_MODE_VALIDATE: &str = "validate";

pub const ARG_V_MODES: [&str; 4] = [
    ARG_V_MODE_RUN,
    ARG_V_MODE_BUILD,
    ARG_V_MODE_EXEC,
    ARG_V_MODE_VALIDATE,
];

// ==========================
//
// ARGUMENT VALUES END
//
// ==========================

// ==========================
//
// ARGUMENT TYPES START
//
// ==========================

pub enum ArgType {
    SourceFile(&'static [&'static str]),
    Boolean,
    Any,
}

pub struct Arg {
    pub name: &'static str,
    pub ty: ArgType,
    pub req: bool,                 // required or not
    pub def: Option<&'static str>, // default argument value
}

// ==========================
//
// ARGUMENT TYPES END
//
// ==========================

// ==========================
//
// ARGUMENT LISTS START
//
// ==========================

pub const RUN_ARGS: &[Arg] = &[
    Arg {
        name: ARG_FLAG_SOURCE_CODE,
        ty: ArgType::SourceFile(FILE_EXT_SOURCE_CODE),
        req: true,
        def: None,
    },
    Arg {
        name: ARG_FLAG_DATA,
        ty: ArgType::SourceFile(FILE_EXT_DATA),
        req: true,
        def: None,
    },
    Arg {
        name: ARG_FLAG_DATA_SCHEMA,
        ty: ArgType::SourceFile(FILE_EXT_DATA_SCHEMA),
        req: true,
        def: None,
    },
    Arg {
        name: ARG_FLAG_OUTPUT,
        ty: ArgType::Any,
        req: false,
        def: None,
    },
    Arg {
        name: ARG_FLAG_PRINT_BYTECODE,
        ty: ArgType::Boolean,
        req: false,
        def: Some(ARG_V_BOOL_FALSE),
    },
];

pub const BUILD_ARGS: &[Arg] = &[
    Arg {
        name: ARG_FLAG_SOURCE_CODE,
        ty: ArgType::SourceFile(FILE_EXT_SOURCE_CODE),
        req: true,
        def: None,
    },
    Arg {
        name: ARG_FLAG_DATA_SCHEMA,
        ty: ArgType::SourceFile(FILE_EXT_DATA_SCHEMA),
        req: true,
        def: None,
    },
    Arg {
        name: ARG_FLAG_OUTPUT,
        ty: ArgType::SourceFile(FILE_EXT_EXECUTABLE),
        req: true,
        def: None,
    },
];

pub const EXEC_ARGS: &[Arg] = &[
    Arg {
        name: ARG_FLAG_EXECUTABLE,
        ty: ArgType::SourceFile(FILE_EXT_EXECUTABLE),
        req: true,
        def: None,
    },
    Arg {
        name: ARG_FLAG_DATA,
        ty: ArgType::SourceFile(FILE_EXT_DATA),
        req: true,
        def: None,
    },
    Arg {
        name: ARG_FLAG_UNSAFE_ASSUME_VALID,
        ty: ArgType::Boolean,
        req: false,
        def: Some(ARG_V_BOOL_FALSE),
    },
];

pub const VALIDATE_ARGS: &[Arg] = &[
    Arg {
        name: ARG_FLAG_DATA,
        ty: ArgType::SourceFile(FILE_EXT_DATA),
        req: true,
        def: None,
    },
    Arg {
        name: ARG_FLAG_DATA_SCHEMA,
        ty: ArgType::SourceFile(FILE_EXT_DATA_SCHEMA),
        req: true,
        def: None,
    },
];

// ==========================
//
// ARGUMENT LISTS END
//
// ==========================
