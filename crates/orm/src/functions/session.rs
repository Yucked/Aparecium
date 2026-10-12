// Session functions
// These functions can be used when working with and manipulating text and string values.
//
// Function	Description
// session::db()	Returns the currently selected database
// session::id()	Returns the current user's session ID
// session::ip()	Returns the current user's session IP address
// session::ns()	Returns the currently selected namespace
// session::origin()	Returns the current user's HTTP origin
// session::ac()	Returns the access method the current user authenticated with
//

macro_rules! create_function {
    ($(#[$attr:meta])* => $function_name: expr) => {
        paste::paste! {
            $(#[$attr])*
            pub fn [<$function_name _fn>]() -> $crate::Function {
                $crate::Function::new(concat!("session::", $function_name))
            }

            $(#[$attr])*
            #[macro_export]
            macro_rules! [<session_ $function_name>] {
                () => {
                    $crate::functions::session::[<$function_name _fn>]()
                };
            }

            pub use [<session_ $function_name>] as [<$function_name>];

        }
    };
}

create_function!(
    /// Returns the currently selected database
    /// Also aliased as `session_db!
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{*, functions::session};
    ///
    /// session::db!();
    /// ```
    =>
"db");

create_function!(
    /// Returns the current user's session ID
    /// Also aliased as `session_id!
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{*, functions::session};
    ///
    /// session::id!();
    /// ```
    =>
"id");

create_function!(
    /// Returns the current user's session IP address
    /// Also aliased as `session_ip!
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{*, functions::session};
    ///
    /// session::ip!();
    /// ```
    =>
"ip");

create_function!(
    /// Returns the currently selected namespace
    /// Also aliased as `session_ns!
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{*, functions::session};
    ///
    /// session::ns!();
    /// ```
    =>
"ns");

create_function!(
    /// Returns the current user's HTTP origin
    /// Also aliased as `session_origin!
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{*, functions::session};
    ///
    /// session::origin!();
    /// ```
    =>
"origin");

create_function!(
    /// Returns the access method the current user authenticated with
    /// Also aliased as `session_ac!
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{*, functions::session};
    ///
    /// assert_eq!(session::ac!().to_debug_sql(), "session::ac()");
    /// ```
    =>
"ac");
