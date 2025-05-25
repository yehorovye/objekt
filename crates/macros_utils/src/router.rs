/// Macro for declaring Actix Web route configurations with a fluent interface.
///
/// # Features
/// - Add guards with `with guard(expression)`
/// - Nest modules with `load module_name`
/// - Include routes with `route handler_name`
/// - Optional base path with `on "/base_path"`
/// - Preserves declaration order for execution order
///
/// # Usage
/// # ```rust
/// # macros_utils::routes! {
/// #    // Execution order:
/// #    with guard(global_auth),    // Applied first
/// #    load admin_dashboard,       // Configured second
/// #    with guard(rate_limiter),   // Applied third
/// #    route health_check,         // Added fourth
/// #    on "/api/v1"                // Base path (optional)
/// # }
/// # ```
///
/// # Generates
/// - A `routes` function that configures a ServiceConfig
/// - Documentation listing all components in order
#[macro_export]
macro_rules! routes {
    (
        @build
        $($(with guard($guard:expr)),+)?
        $($(load $mod:ident),+)?
        $($(route $route:ident),+)?
        ;
        ($cfg:ident) $body:expr
    )  => {
        /// Create a scope with the following stuff:
        $( /// Guards:
        $(#[doc = concat!(" - ", stringify!($guard))])+)?
        $( /// Modules:
        $(#[doc = concat!(" - ", stringify!($mod))])+)?
        $( /// Routes:
        $(#[doc = concat!(" - ", stringify!($route))])+)?
        ///
        pub fn routes($cfg: &mut ::actix_web::web::ServiceConfig) {
            $body;
        }
    };

    (
        $($(with guard($guard:expr)),+ $(,)?)?
        $($(load $mod:ident),+ $(,)?)?
        $($(route $route:ident),+ $(,)?)?
        on $base:literal
    ) => {
        $crate::routes! {
            @build
            $($(with guard($guard)),+ )?
            $($(load $mod),+ )?
            $($(route $route),+ )?
            ;
            (_cfg)
            _cfg.service(
                ::actix_web::Scope::new($base)
                $($(.guard($guard))+)?
                $($(.configure($mod::routes))+)?
                $($(.service($route))+)?
            )
        }
    };

    (
        $($(with guard($guard:expr)),+ $(,)?)?
        $($(load $mod:ident),+ $(,)?)?
        $($(route $route:ident),+ $(,)?)?
    ) => {
        $crate::routes! {
            @build
            $($(with guard($guard)),+ )?
            $($(load $mod),+ )?
            $($(route $route),+ )?
            ;
            (_cfg)
            _cfg
            $($(.guard($guard))+)?
            $($(.configure($mod::routes))+)?
            $($(.service($route))+)?
        }
    };
}

#[allow(unused)]
pub use routes;
