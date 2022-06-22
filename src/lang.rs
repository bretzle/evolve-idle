use std::borrow::Cow;

use fluent::{FluentArgs, FluentBundle, FluentResource};
use once_cell::sync::Lazy;
use unic_langid::langid;

static LANG: Lazy<Locale> = Lazy::new(Locale::new);

struct Locale(FluentBundle<FluentResource>);

impl Locale {
    pub fn new() -> Self {
        let stuff = std::fs::read_to_string("lang/en-US.ftl").unwrap();

        let res = FluentResource::try_new(stuff).expect("Failed to parse an FTL string.");

        let langid_en = langid!("en-US");
        let mut bundle = FluentBundle::new(vec![langid_en]);

        bundle
            .add_resource(res)
            .expect("Failed to add FTL resources to the bundle.");

        bundle.set_use_isolating(false);

        Self(bundle)
    }
}

// Safety: This is ok because only the render thread (the main thread) will be using i18n
unsafe impl Sync for Locale {}
unsafe impl Send for Locale {}

pub fn loc<'a>(key: &'a str, args: Option<&'a FluentArgs>) -> Cow<'a, str> {
    if let Some(msg) = LANG.0.get_message(key) {
        if let Some(pattern) = msg.value() {
            return LANG.0.format_pattern(pattern, args, &mut vec![]);
        }
    }

    Cow::Borrowed(key)
}

#[macro_export]
macro_rules! loc {
    ($key:expr) => {
        crate::lang::loc($key, None)
    };

	($key:expr, $( $arg:expr ),+) => {{
		let mut args = fluent::FluentArgs::new();

		$(
			args.set(stringify!($arg), fluent::FluentValue::from($arg));
		)+

        Cow::Owned(crate::lang::loc($key, Some(&args)).into_owned())
    }};

	($key:expr, $( $arg_key:literal = $arg:expr ),+) => {{
		let mut args = fluent::FluentArgs::new();

		$(
			args.set($arg_key, fluent::FluentValue::from($arg));
		)+

        crate::lang::loc($key, Some(&args))
    }};
}
