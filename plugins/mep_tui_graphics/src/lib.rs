use crossterm::tty::IsTty;
use libmep::{
    graphics::{Color, Style},
    plugin::{GraphicPlugin, Plugin, PluginInfo, PluginType},
};
use log::{error, info, warn};
use std::{io::Write, rc::Rc};

struct MepTuiVideoPlugin {
    buffer: Vec<Color>,
    width: u16,
    height: u16,
    style: Rc<Style>,
}

impl MepTuiVideoPlugin {
    fn read_buffer(&self, x: u16, y: u16) -> Color {
        if x > self.width || y > self.height {
            error!("Reading outside of terminal, exiting");
            std::process::exit(1);
        }

        return self.buffer[(x as usize) + ((y as usize) * (self.width as usize))];
    }

    fn write_buffer(&mut self, x: u16, y: u16, data: Color) {
        if x > self.width || y > self.height {
            error!("Reading outside of terminal, exiting");
            std::process::exit(1);
        }

        self.buffer[(x + (y * self.width)) as usize] = data;
    }

    fn compress_buffer(&mut self) -> Vec<(String, Color)> {
        // Contains (string section, color of screen section) for faster drawing for crossterm
        let mut buffer: Vec<(String, Color)> = Vec::new();

        // Go through the entire buffer and split the buffer into chunks
        for y in 0..self.height {
            // The last pixel we looked at
            let mut last_pixel = self.read_buffer(0, y);

            // The current size of the strings
            let mut string_sizes = 0;

            // Compress a line
            for x in 1..self.width {
                // The pixel we are currently working with
                let current_pixel = self.read_buffer(x, y);

                // Add one to the string size
                string_sizes += 1;

                // If the last pixel and the new one do not match, generate a string the length of our counter, and add the old pixel to represent the colors
                if current_pixel != last_pixel {
                    // Generate the string and push the info
                    buffer.push((
                        // The string we are generating
                        vec!['█'; string_sizes].into_iter().collect::<String>(),
                        // The color
                        last_pixel,
                    ));

                    // Set the size of the string to zero again
                    string_sizes = 0;
                }

                // Write to the old pixel data
                last_pixel = current_pixel;
            }

            // Close the data to ensure its all been written
            buffer.push((
                // The string we are generating
                vec!['█'; string_sizes + 1].into_iter().collect::<String>(),
                // The color
                last_pixel,
            ));

            // Insert a new line to make sure the formatting is correct
            buffer.push((
                "\n".to_string(),
                Color {
                    red: 0,
                    green: 0,
                    blue: 0,
                    alpha: 0,
                },
            ));
        }

        // Return the buffer
        return buffer;
    }
}

impl Plugin for MepTuiVideoPlugin {
    fn new() -> Self
    where
        Self: Sized,
    {
        // Ensure we are actually being handed a tty of some kind
        if !std::io::stdout().is_tty() {
            warn!("Stdout was detected as not being a tty.");
        }

        let size = match crossterm::terminal::size() {
            Ok(tmp) => tmp,
            Err(err) => {
                warn!("Could not retrieve terminal size because of: {}.", err);
                (100, 100)
            }
        };

        info!("{} x {}", size.0, size.1);

        return Self {
            // Just fill it with nothing for now
            buffer: vec![
                Color {
                    red: 0,
                    green: 0,
                    blue: 0,
                    alpha: 0
                };
                (size.0 as usize) * (size.1 as usize)
            ],

            // Get the width
            width: size.0,

            // Get the height
            height: size.1,

            // Make a arbitary style and assign it here
            style: Rc::new(Style::new()),
        };
    }

    fn init(&mut self) {}

    fn get_plugin_info(&self) -> PluginInfo {
        return PluginInfo {
            plugin_name: "Tui Graphics",
            plugin_version: option_env!("CARGO_PKG_VERSION").unwrap_or("unknown"),
            plugin_type: PluginType::GraphicsPlugin,
        };
    }
}

impl GraphicPlugin for MepTuiVideoPlugin {
    fn commit_buffer(&mut self) {
        // StdOut being used for this section
        let mut stdout = std::io::stdout();

        // Contains (string section, color of screen section) for faster drawing for crossterm
        let buffer = self.compress_buffer();

        // It doesnt matter what the background is
        crossterm::queue!(
            stdout,
            crossterm::style::SetBackgroundColor(crossterm::style::Color::Black)
        )
        .unwrap();

        // Move to the start
        crossterm::queue!(stdout, crossterm::cursor::MoveTo(0, 0)).unwrap();

        // Write the buffer
        for x in &buffer {
            // Set the color
            crossterm::queue!(
                stdout,
                crossterm::style::SetForegroundColor(crossterm::style::Color::Rgb {
                    r: x.1.red,
                    g: x.1.green,
                    b: x.1.blue,
                })
            )
            .unwrap();

            // Print the chunk
            crossterm::queue!(stdout, crossterm::style::Print(&x.0)).unwrap();
        }

        // Finish the buffer
        stdout.flush().unwrap();
    }

    fn draw_rect(&mut self, x: u16, y: u16, width: u16, height: u16) {}

    fn fill_rect(&mut self, x: u16, y: u16, width: u16, height: u16) {
        for a in x..width {
            for b in y..height {
                self.write_buffer(
                    a,
                    b,
                    Color {
                        red: 0,
                        green: 0,
                        blue: 0,
                        alpha: 0,
                    },
                )
            }
        }
    }

    fn draw_line(&mut self, x1: u16, y1: u16, x2: u16, y2: u16) {
        // https://www.geeksforgeeks.org/bresenhams-line-generation-algorithm/
        // Need to cast as a int for a little in order to do this calculation
        let dx = ((x2 as i32) - (x1 as i32)).abs() as u16;
        let dy = ((y2 as i32) - (y1 as i32)).abs() as u16;

        if dx > dy {
        } else {
        }
    }

    fn set_style(&mut self, style: Rc<Style>) {
        self.style = style;
    }
}

#[no_mangle]
pub fn mep_get_plugin() -> Rc<dyn Plugin> {
    return Rc::new(MepTuiVideoPlugin::new());
}
