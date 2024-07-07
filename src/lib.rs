pub fn horizontal(start_red: u8, start_green: u8, start_blue: u8, end_red: u8, end_green: u8, end_blue: u8, text: &str) {
    let mut red = start_red as f32;
    let mut green = start_green as f32;
    let mut blue = start_blue as f32;

    let max_line = text.lines()
        .map(|line| line.len())
        .max()
        .unwrap_or(0);

    let red_step = (end_red as f32 - start_red as f32) / max_line as f32;
    let green_step = (end_green as f32 - start_green as f32) / max_line as f32;
    let blue_step = (end_blue as f32 - start_blue as f32) / max_line as f32;

    for line in text.lines() {
        for char in line.chars() {
            print!("\x1b[38;2;{};{};{}m{}", red as u8, green as u8, blue as u8, char);
            red += red_step;
            green += green_step;
            blue += blue_step;
        }
        println!();
        red = start_red as f32;
        green = start_green as f32;
        blue = start_blue as f32;
    }
    print!("\x1b[0m");
}

pub fn vertical(start_red: u8, start_green: u8, start_blue: u8, end_red: u8, end_green: u8, end_blue: u8, text: &str) {
    let mut red = start_red as f32;
    let mut green = start_green as f32;
    let mut blue = start_blue as f32;

    let lines = text.lines().count();

    let red_step = (end_red as f32 - start_red as f32) / lines as f32;
    let green_step = (end_green as f32 - start_green as f32) / lines as f32;
    let blue_step = (end_blue as f32 - start_blue as f32) / lines as f32;

    for line in text.lines() {
        println!("\x1b[38;2;{};{};{}m{}", red as u8, green as u8, blue as u8, line);
        red += red_step;
        green += green_step;
        blue += blue_step;
    }
    print!("\x1b[0m");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logo() {
        println!("vertical:");
        vertical(255, 0, 0, 0, 0, 255, "AAAAAAAAAA\nAAAAAAAAAA\nAAAAAAAAAA\nAAAAAAAAAA\nAAAAAAAAAA\nAAAAAAAAAA\nAAAAAAAAAA\nAAAAAAAAAA\nAAAAAAAAAA\nAAAAAAAAAA");
        println!("horizontal:");
        horizontal(255, 0, 0, 0, 0, 255, "AAAAAAAAAA\nAAAAAAAAAA\nAAAAAAAAAA\nAAAAAAAAAA\nAAAAAAAAAA\nAAAAAAAAAA\nAAAAAAAAAA\nAAAAAAAAAA\nAAAAAAAAAA\nAAAAAAAAAA");
        println!("end");
    }
}
