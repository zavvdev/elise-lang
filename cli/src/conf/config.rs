pub struct FileExt;
impl FileExt {
    pub const SOURCE_CODE: &[&str] = &[".eli"];
    pub const EXECUTABLE: &[&str] = &[".elb"];
    pub const DATA_SCHEMA: &[&str] = &[".elt"];
    pub const CSV: &str = ".csv";
    pub const DATA: &[&str] = &[Self::CSV];
}

pub struct ArgName;
impl ArgName {
    pub const MODE: &str = "mode";
    pub const SOURCE_CODE: &str = "source-code";
    pub const DATA: &str = "data";
    pub const DATA_SCHEMA: &str = "data-schema";
    pub const EXECUTABLE: &str = "executable";
    pub const OUTPUT: &str = "output";
    pub const PRINT_BYTECODE: &str = "print-bytecode";
}

pub struct ArgValue;
impl ArgValue {
    pub const BOOL_TRUE: &str = "true";
    pub const BOOL_FALSE: &str = "false";
    pub const MODE_RUN: &str = "run";
    pub const MODE_BUILD: &str = "build";
    pub const MODE_EXEC: &str = "exec";
    pub const MODE_VALIDATE: &str = "validate";

    pub const MODES: [&str; 4] = [
        Self::MODE_RUN,
        Self::MODE_BUILD,
        Self::MODE_EXEC,
        Self::MODE_VALIDATE,
    ];
}

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

pub const RUN_ARGS: &[Arg] = &[
    Arg {
        name: ArgName::SOURCE_CODE,
        ty: ArgType::SourceFile(FileExt::SOURCE_CODE),
        req: true,
        def: None,
    },
    Arg {
        name: ArgName::DATA,
        ty: ArgType::SourceFile(FileExt::DATA),
        req: true,
        def: None,
    },
    Arg {
        name: ArgName::DATA_SCHEMA,
        ty: ArgType::SourceFile(FileExt::DATA_SCHEMA),
        req: true,
        def: None,
    },
    Arg {
        name: ArgName::OUTPUT,
        ty: ArgType::Any,
        req: false,
        def: None,
    },
    Arg {
        name: ArgName::PRINT_BYTECODE,
        ty: ArgType::Boolean,
        req: false,
        def: Some(ArgValue::BOOL_FALSE),
    },
];

pub const BUILD_ARGS: &[Arg] = &[
    Arg {
        name: ArgName::SOURCE_CODE,
        ty: ArgType::SourceFile(FileExt::SOURCE_CODE),
        req: true,
        def: None,
    },
    Arg {
        name: ArgName::DATA_SCHEMA,
        ty: ArgType::SourceFile(FileExt::DATA_SCHEMA),
        req: true,
        def: None,
    },
    Arg {
        name: ArgName::OUTPUT,
        ty: ArgType::SourceFile(FileExt::EXECUTABLE),
        req: true,
        def: None,
    },
];

pub const EXEC_ARGS: &[Arg] = &[
    Arg {
        name: ArgName::EXECUTABLE,
        ty: ArgType::SourceFile(FileExt::EXECUTABLE),
        req: true,
        def: None,
    },
    Arg {
        name: ArgName::DATA,
        ty: ArgType::SourceFile(FileExt::DATA),
        req: true,
        def: None,
    },
];

pub const VALIDATE_ARGS: &[Arg] = &[
    Arg {
        name: ArgName::DATA,
        ty: ArgType::SourceFile(FileExt::DATA),
        req: true,
        def: None,
    },
    Arg {
        name: ArgName::DATA_SCHEMA,
        ty: ArgType::SourceFile(FileExt::DATA_SCHEMA),
        req: true,
        def: None,
    },
];
