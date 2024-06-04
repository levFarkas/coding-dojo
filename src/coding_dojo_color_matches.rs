use std::f32;

pub struct Color<'a> {
    name: &'a str,
    red: f32,
    green: f32,
    blue: f32,
}
fn get_nearest_color(color: &Color, colors: &Vec<Color>) {
    let mut min = 1000_f32;
    let mut min_color: &Color = &colors[0];
    for i in colors {
        let temp = (i.blue - color.blue) * (i.blue - color.blue)
            + (i.red - color.red) * (i.red - color.red)
            + (i.green - color.green) * (i.green - color.green);

        let d = temp.abs().sqrt();
        if d < min {
            min = d;
            min_color = i
        }
    }
    println!("The nearest color is {} of {}", min_color.name, color.name)
}
pub fn do_match() {
    let color = Color {
        name: "Violet",
        red: 127_f32,
        green: 0_f32,
        blue: 255_f32,
    };
    let colors: Vec<Color> = vec![
        Color {
            name: "Red",
            red: hexa_to_dec("FF"),
            blue: hexa_to_dec("00"),
            green: hexa_to_dec("00"),
        },
        Color {
            name: "Blue",
            red: hexa_to_dec("00"),
            blue: hexa_to_dec("FF"),
            green: hexa_to_dec("00"),
        },
        Color {
            name: "Green",
            red: hexa_to_dec("00"),
            blue: hexa_to_dec("00"),
            green: hexa_to_dec("FF"),
        },
    ];
    get_nearest_color(&color, &colors);
    get_nearest_color_v2(&color, &colors);
}

pub fn hexa_to_dec(hexa: &str) -> f32 {
    return i64::from_str_radix(hexa, 16).unwrap() as f32;
}

pub fn get_nearest_color_v2(color: &Color, colors: &Vec<Color>) {
    let binary_red = f32_to_binary(255);
}

fn f32_to_binary(number: i32) -> String {
    let mut act = number.clone();
    let mut binary = String::from("");
    while act != 0 {
        if act % 2 == 1 {
            binary.push_str("1")
        } else {
            binary.push_str("0")
        }
        act = act / 2
    }
    return binary;
}
