use pgrx::prelude::*;

::pgrx::pg_module_magic!();

#[pg_extern]
fn hello_function_overloading() -> &'static str {
    "Hello, function_overloading"
}

#[pg_extern]
fn add(a: i32, b: i32) -> i32 {
    a+b
}

#[pg_extern(sql= "
        CREATE FUNCTION add(REAL,REAL)
        RETURNS REAL
        AS 'MODULE_PATHNAME', 'add_real'
        LANGUAGE C STRICT;
    ")
]
fn add_real(a: f32, b: f32) -> f32 {
    a+b
}

#[pg_extern(sql= "
        CREATE FUNCTION add(TEXT,TEXT)
        RETURNS TEXT
        AS 'MODULE_PATHNAME', 'add_str'
        LANGUAGE C STRICT;
    ")
]
fn add_str(a: &'static str, b: &'static str) -> &'static str {
    format!("{a}{b}")
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_hello_function_overloading() {
        assert_eq!("Hello, function_overloading", crate::hello_function_overloading());
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
