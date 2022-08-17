use console::{Style, Term};
use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use std::io::Error;

use crate::{input_data::input_data, models::Property, Properties};

pub fn set_properties(
    properties: &mut Properties,
    options: Vec<Property>,
    main_color: &Style,
    color_key: &Style,
    color_value: &Style,
) -> Result<(), Error> {
    if let Some(want_properties) = Confirm::new()
        .with_prompt(format!(
            "Want to add {} {}",
            if !properties.is_empty() {
                "more"
            } else {
                "some"
            },
            &main_color.apply_to("properties")
        ))
        .interact_opt()?
    {
        if want_properties {
            let mut key = String::new();
            input_data(&mut key, "PROPERTY NAME", color_key, true, None);
            println!("Choice {}", color_value.apply_to("PROPERTY VALUE"));
            let selected = Select::with_theme(&ColorfulTheme::default())
                .items(&options)
                .default(0)
                .interact_on_opt(&Term::stderr())?
                .unwrap();
            let selected_parsed = options.get(selected).unwrap();
            println!("{}", color_value.apply_to(selected_parsed));
            properties.insert(key.trim().replace(' ', "_"), selected_parsed.to_owned());
            set_properties(properties, options, main_color, color_key, color_value)?;
        } else {
            return Ok(());
        }
    }
    Ok(())
}
