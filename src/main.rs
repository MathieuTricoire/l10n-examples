use crate::fluent::time::Time;
use l10n::fluent_bundle::FluentValue;
use l10n::unic_langid::{langid, LanguageIdentifier};
use l10n::{message, L10nMessage};
use std::borrow::Cow;

mod fluent;

pub fn fluent_transform(s: &str) -> std::borrow::Cow<str> {
    fluent_pseudo::transform(s, false, false)
}

l10n::init!({
    transform: if cfg!(debug_assertions) {
        Some(fluent_transform)
    } else {
        None
    },
    functions: {
        "TIME": crate::fluent::time::time
    }
});

fn main() {
    println!("English (United States):");
    run(&langid!("en"));

    println!();

    println!("English (Great-Britain):");
    run(&langid!("en-GB"));

    println!();

    println!("English (Canada):");
    run(&langid!("en-CA"));

    println!();

    println!("French:");
    run(&langid!("fr"));

    println!();

    println!("Unsupported locale, you have to ensure you will pass a supported locale:");
    run(&langid!("fr-BE"));
}

fn run(lang: &LanguageIdentifier) {
    let username = "Alice";
    let greeting = message!("app", "greeting", username);
    println!("{}", greeting.translate(lang));

    let status = Status::Busy("Working".to_string(), Gender::Female);
    println!("{}", status.translate(lang));

    let status = Status::BusyFor(BusyFor {
        reason: "Break".to_string(),
        gender: Gender::Female,
        time: Time::minutes(125),
    });
    println!("{}", status.translate(lang));

    let change_font_color = message!("settings", "change-font-color");
    println!("{}", change_font_color.translate(lang));

    let colors = [Color::Gray, Color::Black, Color::White];
    for color in colors {
        println!("- {}", color.translate(lang));
    }

    let launch_timer = message!("app", "launch-timer", "seconds" = 60);
    println!("{}", launch_timer.translate(lang));
    let launch_timer = message!("app", "launch-timer", "seconds" = 65);
    println!("{}", launch_timer.translate(lang));
    let launch_timer = message!("app", "launch-timer", "seconds" = 120);
    println!("{}", launch_timer.translate(lang));
    let launch_timer = message!("app", "launch-timer", "seconds" = 150);
    println!("{}", launch_timer.translate(lang));

    let order_a_pop = message!("app", "order-a-pop");
    println!("{}", order_a_pop.translate(lang));
}

#[allow(unused)]
#[derive(L10nMessage)]
#[l10n_message("app", "status")]
enum Status {
    #[l10n_message(".online")]
    Online,
    #[l10n_message(".offline")]
    Offline,
    #[l10n_message(".busy", "reason" = .0, "gender" = .1)]
    Busy(String, Gender),
    #[l10n_message(transparent)]
    BusyFor(#[l10n_from] BusyFor),
}

#[derive(L10nMessage)]
#[l10n_message("app", "status.busy-for", reason, "gender" = gender, time)]
struct BusyFor {
    reason: String,
    gender: Gender,
    time: Time,
}

#[allow(unused)]
enum Gender {
    Female,
    Male,
    Other,
}

impl<'l> Into<FluentValue<'l>> for &'l Gender {
    fn into(self) -> FluentValue<'l> {
        FluentValue::String(Cow::from(match self {
            Gender::Female => "female",
            Gender::Male => "male",
            Gender::Other => "other",
        }))
    }
}

#[derive(L10nMessage)]
#[l10n_message("settings", "font-colors")]
enum Color {
    #[l10n_message(".gray")]
    Gray,
    #[l10n_message(".black")]
    Black,
    #[l10n_message(".white")]
    White,
}
