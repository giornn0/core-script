use console::Style;
use std::io::stdin;

pub fn input_data(var: &mut String, label: &str, color: &Style, required: bool, def: Option<&str>) {
    let defaulty = def.map_or_else(|| "", |d| d);
    println!(
        "Please input {} {}",
        color.apply_to(label),
        if defaulty.is_empty() {
            "".to_owned()
        } else {
            format!("({})", defaulty)
        }
    );

    stdin().read_line(var).expect("Error while tryying to read");
    if var.len() < 3 {
        var.insert_str(0, defaulty);
    }
    if var.len() < 3 && required {
        input_data(var, label, color, required, def);
    } else {
        let lowered = var.to_owned();
        var.clear();
        var.insert_str(0, lowered.as_str());
    }
}
