// HTTP functions
// These functions can be used when opening and submitting remote web requests, and webhooks.
//
// Function	Description
// http::head()	Perform a remote HTTP HEAD request
// http::get()	Perform a remote HTTP GET request
// http::put()	Perform a remote HTTP PUT request
// http::post()	Perform a remote HTTP POST request
// http::patch()	Perform a remote HTTP PATCH request
// http::delete()	Perform a remote HTTP DELETE request

use crate::{Function, ObjectLike, StrandLike, ValueLike};

/// Represents URL
pub type Url = StrandLike;

fn create_fn_with_two_args(
    url: impl Into<Url>,
    custom_headers: Option<impl Into<ObjectLike>>,
    function_name: &'static str,
) -> Function {
    let function = Function::new(function_name).arg(url.into());
    match custom_headers {
        Some(headers) => function.arg(headers.into()),
        None => function,
    }
}

/// An omitted body or header argument is an empty object.
fn object_or_empty(object: Option<impl Into<ObjectLike>>) -> ValueLike {
    match object {
        Some(object) => ValueLike::from(Into::<ObjectLike>::into(object)),
        None => ValueLike::syntax("{ }"),
    }
}

fn create_fn_with_three_args(
    url: impl Into<Url>,
    request_body: Option<impl Into<ObjectLike>>,
    custom_headers: Option<impl Into<ObjectLike>>,
    function_name: &'static str,
) -> Function {
    Function::new(function_name)
        .arg(url.into())
        .arg(object_or_empty(request_body))
        .arg(object_or_empty(custom_headers))
}

macro_rules! create_fn_with_url_and_head {
    ($(#[$attr:meta])* => $function_name: expr) => {
        paste::paste! {
            $(#[$attr])*
            pub fn [<$function_name _fn>](url: impl Into<Url>, custom_headers: Option<impl Into<ObjectLike>>) -> Function {
               create_fn_with_two_args(url, custom_headers, concat!("http::", $function_name))
           }

           $(#[$attr])*
           #[macro_export]
           macro_rules! [<http_ $function_name>] {
               ( $url:expr ) => {
                   $crate::functions::http::[<$function_name _fn>]($url, None as Option<$crate::ObjectLike>)
               };
               ( $url:expr, $custom_headers:expr ) => {
                   $crate::functions::http::[<$function_name _fn>]($url, Some($custom_headers))
               };
           }
           pub use [<http_ $function_name>] as [<$function_name>];


        }
    };
}

create_fn_with_url_and_head!(
    /// The http::head function performs a remote HTTP HEAD request. The first parameter is the URL of the remote endpoint.
    /// If the response does not return a 2XX status code, then the function will fail and return the error.
    /// Also aliased as `http_head!`
    ///
    /// http::head(string) -> null
    /// If an object is given as the second argument, then this can be used to set the request headers.
    ///
    /// http::head(string, object) -> null
    ///
    /// # Arguments
    ///
    /// * `url` - The URL of the remote endpoint. Can be a string or a field or a parameter.
    /// * `request_body` - Optional. The request body. Can be an object or a field or a parameter.
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{*, functions::http};
    /// use std::collections::HashMap;
    /// http::head!("https://example.com");
    ///
    /// # let url_field = Field::new("url_field");
    /// http::head!(url_field);
    ///
    /// # let ur_param = Param::new("url_param");
    /// http::head!(ur_param);
    ///
    /// let headers = HashMap::from([("x-my-header".to_string(), "some unique string".to_string())]);
    /// http::head!("https://example.com", headers);
    => "head"
);

create_fn_with_url_and_head!(
    /// The http::get function performs a remote HTTP GET request. The first parameter is the URL of the remote endpoint.
    /// If the response does not return a 2XX status code, then the function will fail and return the error.
    /// If the remote endpoint returns an application/json content-type, then the response is parsed and returned as a value, otherwise the response is treated as text.
    /// Also aliased as `http_get!`
    ///
    /// http::get(string) -> value
    /// If an object is given as the second argument, then this can be used to set the request headers.
    ///
    /// http::get(string, object) -> value
    ///
    /// # Arguments
    ///
    /// * `url` - The URL of the remote endpoint. Can be a string or a field or a parameter.
    /// * `request_body` - Optional. The request body. Can be an object or a field or a parameter.
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{*, functions::http};
    /// use std::collections::HashMap;
    /// http::get!("https://example.com");
    ///
    /// # let url_field = Field::new("url_field");
    /// http::get!(url_field);
    ///
    /// # let url_param = Param::new("url_param");
    /// http::delete!(url_param);
    ///
    /// let headers = HashMap::from([("x-my-header".to_string(), "some unique string".to_string())]);
    /// http::get!("https://example.com", headers);
    => "get"
);

create_fn_with_url_and_head!(
    /// The http::delete function performs a remote HTTP DELETE request. The first parameter is the URL of the remote endpoint.
    /// If the response does not return a 2XX status code, then the function will fail and return the error.
    /// If the remote endpoint returns an application/json content-type, then the response is parsed and returned as a value, otherwise the response is treated as text.
    /// Also aliased as `http_delete!`
    ///
    /// http::delete(string) -> value
    /// If an object is given as the second argument, then this can be used to set the request headers.
    ///
    /// http::delete(string, object) -> value
    ///
    /// # Arguments
    ///
    /// * `url` - The URL of the remote endpoint. Can be a string or a field or a parameter.
    /// * `request_body` - Optional. The request body. Can be an object or a field or a parameter.
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{*, functions::http};
    /// use std::collections::HashMap;
    /// http::delete!("https://example.com");
    ///
    /// # let url_field = Field::new("url_field");
    /// http::delete!(url_field);
    /// # let url_param = Param::new("url_param");
    /// http::delete!(url_param);
    ///
    /// let headers = HashMap::from([("x-my-header".to_string(), "some unique string".to_string())]);
    /// http::delete!("https://example.com", headers);
    => "delete"
);

macro_rules! create_fn_with_3args_url_body_and_head {
    ($(#[$attr:meta])* => $function_name: expr) => {
        paste::paste! {
            $(#[$attr])*
            pub fn [<$function_name _fn>](
                url: impl Into<Url>,
                request_body: Option<impl Into<ObjectLike>>,
                custom_headers: Option<impl Into<ObjectLike>>,
            ) -> Function {
               create_fn_with_three_args(url, request_body, custom_headers, concat!("http::", $function_name))
           }

            $(#[$attr])*
            #[macro_export]
            macro_rules! [<http_ $function_name>] {
               ( $url:expr, $request_body:expr ) => {
                   $crate::functions::http::[<$function_name _fn>]($url, Some($request_body), None as Option<$crate::ObjectLike>)
               };
               ( $url:expr, $request_body:expr, $custom_headers:expr ) => {
                   $crate::functions::http::[<$function_name _fn>]($url, Some($request_body) ,Some($custom_headers))
               };
            }
            pub use [<http_ $function_name>] as [<$function_name>];


        }
    };
}

create_fn_with_3args_url_body_and_head!(
    /// The http::post function performs a remote HTTP POST request. The first parameter is the URL of the remote endpoint,
    /// and the second parameter is the value to use as the request body, which will be converted to JSON.
    /// If the response does not return a 2XX status code, then the function will fail and return the error.
    /// If the remote endpoint returns an application/json content-type, then the response is parsed and returned as a value,
    /// otherwise the response is treated as text.
    ///
    /// Also aliased as `http__post`.
    ///
    /// http::post(string, object) -> value
    /// If an object is given as the third argument, then this can be used to set the request headers.
    ///
    /// http::post(string, object, object) -> val
    ///
    /// # Arguments
    /// * `url` - The URL of the remote endpoint. Can be a string or a field or a param.
    /// * `body` - The value to use as the request body, which will be converted to JSON. Can be a object or a field or a param.
    /// * `headers` - Optional. If an object is given as the third argument, then this can be used to set the request headers. Can be a object or a field or a param.
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{*, functions::http};
    /// use std::collections::hash_map::HashMap;
    ///
    /// http::post!("https://example.com", HashMap::from([("id".to_string(), 1)]));
    ///
    /// http::post!("https://example.com", HashMap::from([("id".to_string(), 1)]), HashMap::from([("x-my-header".to_string(), "some unique string".to_string())]));
    ///
    /// # let body_field = Field::new("body_field");
    /// # let headers_param = Param::new("headers_param");
    /// http::post!("https://example.com", body_field, headers_param);
    /// ```
=> "post"
);

create_fn_with_3args_url_body_and_head!(
    /// The http::put function performs a remote HTTP PUT request. The first parameter is the URL of the remote endpoint,
    /// and the second parameter is the value to use as the request body, which will be converted to JSON.
    /// If the response does not return a 2XX status code, then the function will fail and return the error.
    /// If the remote endpoint returns an application/json content-type, then the response is parsed and returned as a value,
    /// otherwise the response is treated as text.
    ///
    /// Also aliased as `http__put`.
    ///
    /// http::put(string, object) -> value
    /// If an object is given as the third argument, then this can be used to set the request headers.
    ///
    /// http::put(string, object, object) -> value
    ///
    /// # Arguments
    /// * `url` - The URL of the remote endpoint. Can be a string or a field or a param.
    /// * `body` - The value to use as the request body, which will be converted to JSON. Can be a object or a field or a param.
    /// * `headers` - Optional. If an object is given as the third argument, then this can be used to set the request headers. Can be a object or a field or a param.
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{*, functions::http};
    /// use std::collections::hash_map::HashMap;
    ///
    /// http::put!("https://example.com", HashMap::from([("id".to_string(), 1)]));
    ///
    /// http::put!("https://example.com", HashMap::from([("id".to_string(), 1)]), HashMap::from([("x-my-header".to_string(), "some unique string".to_string())]));
    /// ```
=> "put"
);

create_fn_with_3args_url_body_and_head!(
    /// The http::patch function performs a remote HTTP PATCH request. The first parameter is the URL of the remote endpoint,
    /// and the second parameter is the value to use as the request body, which will be converted to JSON.
    /// If the response does not return a 2XX status code, then the function will fail and return the error.
    /// If the remote endpoint returns an application/json content-type, then the response is parsed and returned as a value,
    /// otherwise the response is treated as text.
    ///
    /// Also aliased as `http_patch`.
    ///
    /// http::patch(string, object) -> value
    /// If an object is given as the third argument, then this can be used to set the request headers.
    ///
    /// http::patch(string, object, object) -> value
    ///
    /// # Arguments
    /// * `url` - The URL of the remote endpoint. Can be a string or a field or a param.
    /// * `body` - The value to use as the request body, which will be converted to JSON. Can be a object or a field or a param.
    /// * `headers` - Optional. If an object is given as the third argument, then this can be used to set the request headers. Can be a object or a field or a param.
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{*, functions::http};
    /// use std::collections::hash_map::HashMap;
    ///
    /// let result = http::patch!("https://example.com", HashMap::from([("id".to_string(), 1)]));
    ///
    /// let result = http::patch!("https://example.com", HashMap::from([("id".to_string(), 1)]), HashMap::from([("x-my-header".to_string(), "some unique string".to_string())]));
    /// ```
=> "patch"
);
