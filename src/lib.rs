//! menu_rs is a library for Rust that allows the creation of simple and interactable command-line menus.
//!
//! It's very simple to use, you just create a Menu, adds the option you want it to have with the correspondent
//! action to be run when selected and that's it!
//! You can use the arrow keys to move through the options, ENTER to select an option and ESC to exit the menu.
//!
//! # Example
//!
//! ```
//! use menu_rs::{Menu, MenuOption};
//!
//! fn action_1() {}
//! fn action_2() {}
//! fn action_3() {}
//! fn action_4() {}
//!
//! let menu = Menu::new(vec![
//!     MenuOption::new("Option 1", action_1).hint("Hint for option 1"),
//!     MenuOption::new("Option 2", action_2),
//!     MenuOption::new("Option 3", action_3),
//!     MenuOption::new("Option 4", action_4),
//! ]);
//!
//! menu.show();
//! ```

#![allow(clippy::needless_return)]
#![allow(clippy::redundant_field_names)]

use console::{Key, Style, Term};

/// A option that can be added to a Menu.
pub struct MenuOption {
    label: String,
    func: fn() -> (),
    hint: Option<String>,
}

/// The Menu to be shown in the command line interface.
pub struct Menu {
    title: Option<String>,
    options: Vec<MenuOption>,
    selected_option: usize,
    selected_style: Style,
    normal_style: Style,
    hint_style: Style,
    action_func: fn() -> (),
}

impl MenuOption {
    /// Creates a new Menu option that can then be used by a Menu.
    ///
    /// # Example
    ///
    /// ```
    /// fn action_example() {}
    /// let menu_option = MenuOption::new("Option example", action_example);
    /// ```
    pub fn new(label: &str, func: fn() -> ()) -> MenuOption {
        return MenuOption {
            label: label.to_owned(),
            func: func,
            hint: None,
        };
    }

    /// Sets the hint label with the given text.
    ///
    /// # Example
    ///
    /// ```
    /// fn action_1() {}
    /// let menu_option_1 = MenuOption::new("Option 1", action_1).hint("Hint example");
    /// ```
    pub fn hint(mut self, text: &str) -> MenuOption {
        self.hint = Some(text.to_owned());
        return self;
    }
}

impl Menu {
    /// Creates a new interactable Menu.
    ///
    /// # Example
    ///
    /// ```
    /// fn action_example() {}
    /// let menu_option = MenuOption::new("Option example", action_example);
    /// ```
    pub fn new(options: Vec<MenuOption>) -> Menu {
        return Menu {
            title: None,
            options: options,
            selected_option: 0,
            normal_style: Style::new(),
            selected_style: Style::new().on_blue(),
            hint_style: Style::new().color256(187),
            action_func: dummy_func,
        };
    }

    // Sets a title for the menu.
    pub fn title(mut self, text: &str) -> Menu {
        self.title = Some(text.to_owned());
        return self;
    }

    /// Shows the menu in the command line interface allowing the user
    /// to interact with the menu.
    pub fn show(mut self) {
        let stdout = Term::buffered_stdout();
        stdout.hide_cursor().unwrap();

        // clears the screen and shows the menu
        stdout.clear_screen().unwrap();
        self.draw_menu(&stdout);

        // runs the menu navigation
        self.menu_navigation(&stdout);

        // clears the screen and runs the action function before exiting
        stdout.clear_screen().unwrap();
        stdout.flush().unwrap();
        (self.action_func)();
    }

    fn menu_navigation(&mut self, stdout: &Term) {
        let options_limit_num = self.options.len() - 1;
        loop {
            // gets pressed key
            let key = match stdout.read_key() {
                Ok(val) => val,
                Err(_e) => {
                    println!("Error reading key");
                    return;
                }
            };

            // handles the pressed key
            match key {
                Key::ArrowUp => {
                    self.selected_option = match self.selected_option == 0 {
                        true => options_limit_num,
                        false => self.selected_option - 1,
                    }
                }
                Key::ArrowDown => {
                    self.selected_option = match self.selected_option == options_limit_num {
                        true => 0,
                        false => self.selected_option + 1,
                    }
                }
                Key::Escape => {
                    stdout.show_cursor().unwrap();
                    return;
                }
                Key::Enter => {
                    self.action_func = self.options[self.selected_option].func;
                    stdout.show_cursor().unwrap();
                    return;
                }
                // Key::Char(c) => println!("char {}", c),
                _ => {}
            }

            // redraws the menu
            self.draw_menu(stdout);
        }
    }

    fn draw_menu(&self, stdout: &Term) {
        // clears the screen
        stdout.clear_screen().unwrap();

        // draw title
        match &self.title {
            Some(text) => {
                let title_style = Style::new().bold();
                let title = title_style.apply_to(text);
                let title = format!("  {}", title);
                stdout.write_line(title.as_str()).unwrap()
            }
            None => {}
        };

        // draw the menu to stdout
        for (i, option) in self.options.iter().enumerate() {
            let label_style = match i == self.selected_option {
                true => self.selected_style.clone(),
                false => self.normal_style.clone(),
            };

            // styles the menu entry
            let label = label_style.apply_to(option.label.as_str());
            let hint_str = match &self.options[i].hint {
                Some(hint) => hint,
                None => "",
            };
            let hint = self.hint_style.apply_to(hint_str);

            // builds and writes the menu entry
            let line = format!("- {: <25}\t{}", label, hint);
            stdout.write_line(line.as_str()).unwrap();
        }

        // draws to terminal
        stdout.flush().unwrap();
    }
}

fn dummy_func() {}
