use l10n::{
    fluent_bundle::{types::FluentType, FluentArgs, FluentValue},
    intl_memoizer::{self, Memoizable},
    unic_langid::LanguageIdentifier,
    L10nMessage,
};
use std::any::TypeId;

pub fn time<'a>(positional: &[FluentValue<'a>], _named: &FluentArgs) -> FluentValue<'a> {
    match positional.get(0) {
        Some(FluentValue::Number(n)) => {
            FluentValue::Custom(Box::new(Time::seconds(n.value as usize)))
        }
        Some(FluentValue::Custom(time)) if time.type_id() == TypeId::of::<Time>() => {
            FluentValue::Custom(time.duplicate())
        }
        Some(v) => v.to_owned(),
        _ => FluentValue::Error,
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Time {
    seconds: usize,
}

impl Time {
    pub fn seconds(seconds: usize) -> Self {
        Self { seconds }
    }

    pub fn minutes(minutes: usize) -> Self {
        Self {
            seconds: minutes * 60,
        }
    }
}

impl FluentType for Time {
    fn duplicate(&self) -> Box<dyn FluentType + Send> {
        Box::new(Time::seconds(self.seconds))
    }

    fn as_string(&self, intls: &intl_memoizer::IntlLangMemoizer) -> std::borrow::Cow<'static, str> {
        intls
            .with_try_get::<TimeFormatter, _, _>((), |time_formatter| {
                time_formatter.format(self.seconds).into()
            })
            .expect("Failed to format time.")
    }

    fn as_string_threadsafe(
        &self,
        intls: &intl_memoizer::concurrent::IntlLangMemoizer,
    ) -> std::borrow::Cow<'static, str> {
        intls
            .with_try_get::<TimeFormatter, _, _>((), |time_formatter| {
                time_formatter.format(self.seconds).into()
            })
            .expect("Failed to format time.")
    }
}

impl<'l> Into<FluentValue<'l>> for &'l Time {
    fn into(self) -> FluentValue<'l> {
        FluentValue::Custom(Box::new(Time::seconds(self.seconds)))
    }
}

struct TimeFormatter {
    lang: LanguageIdentifier,
}

impl TimeFormatter {
    pub fn new(lang: LanguageIdentifier) -> Result<Self, ()> {
        Ok(Self { lang })
    }

    // This formatter is not optimal but it's just to show how it could work...
    pub fn format(&self, seconds: usize) -> String {
        if seconds < 60 {
            TimeFormatterValues::Seconds { seconds }
                .translate(&self.lang)
                .to_string()
        } else if seconds < 60 * 60 {
            let minutes = seconds / 60;
            let seconds = seconds - minutes * 60;
            let seconds = match seconds {
                0 => String::new(),
                seconds => format!("{seconds:02}"),
            };
            TimeFormatterValues::Minutes { minutes, seconds }
                .translate(&self.lang)
                .to_string()
        } else {
            let hours = seconds / 60 / 60;
            let minutes = seconds / 60 - hours * 60;
            let minutes = match minutes {
                0 => String::new(),
                minutes => format!("{minutes:02}"),
            };
            TimeFormatterValues::Hours { hours, minutes }
                .translate(&self.lang)
                .to_string()
        }
    }
}

impl Memoizable for TimeFormatter {
    type Args = ();
    type Error = ();

    fn construct(lang: LanguageIdentifier, _: Self::Args) -> Result<Self, Self::Error> {
        Self::new(lang)
    }
}

#[derive(L10nMessage)]
#[l10n_message("formatters", "time")]
enum TimeFormatterValues {
    #[l10n_message(".seconds", seconds)]
    Seconds { seconds: usize },
    #[l10n_message(".minutes", minutes, seconds)]
    Minutes { minutes: usize, seconds: String },
    #[l10n_message(".hours", hours, minutes)]
    Hours { hours: usize, minutes: String },
}
