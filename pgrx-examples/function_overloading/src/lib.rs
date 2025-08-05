use pgrx::prelude::*;

::pgrx::pg_module_magic!();

///
/// Create a function named `add` for 3 different types (INT, REAL, TEXT)
/// each version will be linked to a sepatate Rust function
///
/// |      Rust                    |        SQL                |
/// |------------------------------|---------------------------|
/// | add(i32,i32)                 | add(INT,INT)              |
/// | add_real(f32,f32)            | add(REAL,REAL)            |
/// | add_string(String,String)    | add(TEXT,TEXT)            |
///

#[pg_extern]
fn add(a: i32, b: i32) -> i32 {
    a+b
}

#[pg_extern(sql= "
        CREATE FUNCTION add(REAL,REAL)
        RETURNS REAL
        AS 'MODULE_PATHNAME', 'add_real_wrapper'
        LANGUAGE C STRICT;
    ")
]
fn add_real(a: f32, b: f32) -> f32 {
    a+b
}

#[pg_extern(sql= "
        CREATE FUNCTION add(TEXT,TEXT)
        RETURNS TEXT
        AS 'MODULE_PATHNAME', 'add_string_wrapper'
        LANGUAGE C STRICT;
    ")
]
fn add_string(a: String, b: String) -> String {
    format!("{a}{b}")
}


///
/// Rust does not support default function arguments, but Postgres does
///
/// Again we create 2 different Rust functions linked to the same Postgres
/// function.
///

#[pg_extern]
fn strip(val: String, character: Option<char>) -> String {
    val.replace(character.unwrap_or(' '),"")
}

#[pg_extern(sql= "
        CREATE FUNCTION strip(TEXT)
        RETURNS TEXT
        AS 'MODULE_PATHNAME', 'strip_none_wrapper'
        LANGUAGE C STRICT;
    ")
]
fn strip_none(val: String) -> String {
    strip(val,None)
}



#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_add() {
        assert_eq!(3, crate::add(1,2));
    }

}

/// This module is required by `cargo pgrx test` invocations.
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    #[must_use]
    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
