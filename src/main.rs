use crate::create::{create, Config};
use crate::input_data::input_data;
use crate::set_properties::set_properties;
use console::{Color, Emoji, Style};
use models::Property;
use std::collections::HashMap;
use std::io::Error;
mod create;
mod input_data;
mod models;
mod set_properties;
mod utils;

type Properties = HashMap<String, Property>;

fn main() -> Result<(), Error> {
    let main_color = Style::new().cyan();
    let magenta = Style::new().magenta().bg(Color::Black);
    let blue = Style::new().blue().bg(Color::Black);
    let green = Style::new().green().bg(Color::Black);
    let key_color = Style::new().on_cyan().bg(Color::Black);
    let value_color = Style::new().yellow().bg(Color::Black);
    let mut section = String::new();
    let mut section_plural = String::new();
    let mut front_route = String::new();
    let mut api_route = String::new();
    let mut icon = String::new();

    println!(
        "{} This is {} creator {}",
        Emoji("✨", ":-)"),
        &main_color.apply_to("your"),
        Emoji("✨", ":-)")
    );
    input_data(&mut section, "SECTION", &magenta, true, None);
    section = section.trim().replace(' ', "_");
    input_data(
        &mut section_plural,
        "SECTION PLURAL",
        &blue,
        true,
        Some(format!("{}s", section).as_str()),
    );
    section_plural = section_plural.trim().replace(' ', "_");
    input_data(
        &mut front_route,
        "FRONT ROUTE",
        &green,
        false,
        Some(section_plural.as_str()),
    );
    front_route = front_route.trim().replace(' ', "_");
    input_data(
        &mut api_route,
        "API ROUTE",
        &green,
        false,
        Some(front_route.trim()),
    );
    api_route = api_route.trim().replace(' ', "_");

    input_data(&mut icon, "ICON", &main_color, false, Some("fa-user"));
    icon = icon.trim().replace(' ', "_");

    let mut properties: Properties = HashMap::new();
    let options = Property::get_vec();

    set_properties(
        &mut properties,
        options,
        &main_color,
        &key_color,
        &value_color,
    )?;

    println!(
        "{} Your {} is being {} {}",
        Emoji("🙌", ":-)"),
        &main_color.apply_to("component"),
        &magenta.apply_to("generated"),
        Emoji("🙌", ":-)")
    );

    create(Config::new(
        section,
        section_plural,
        api_route,
        front_route,
        icon,
        properties,
    ))?;
    println!();
    println!(
        "{} Creation was {} {}",
        Emoji("✨", ":-)"),
        &main_color.apply_to("successfull"),
        Emoji("✨", ":-)")
    );
    Ok(())
}
