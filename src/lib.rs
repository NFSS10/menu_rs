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
//! fn action_1() {
//!     println!("action 1")
//! }
//! fn action_2(val: u32) {
//!     println!("action 2 with number {}", val)
//! }
//! fn action_3(msg: &str, val: f32) {
//!     println!("action 3 with string {} and float {}", msg, val)
//! }
//! fn action_4() {
//!     println!("action 4")
//! }
//!
//! let menu = Menu::new(vec![
//!     MenuOption::new("Option 1", action_1).hint("Hint for option 1"),
//!     MenuOption::new("Option 2", || action_2(42)),
//!     MenuOption::new("Option 3", || action_3("example", 3.14)),
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
    func: Box<dyn FnMut()>,
    hint: Option<String>,
}

/// The Menu to be shown in the command line interface.
pub struct Menu {
    title: Option<String>,
    options: Vec<MenuOption>,
    selected_option: i32,
    selected_style: Style,
    normal_style: Style,
    hint_style: Style,
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
    pub fn new<F>(label: &str, func: F) -> MenuOption
    where
        F: FnMut() + 'static,
    {
        return MenuOption {
            label: label.to_owned(),
            func: Box::new(func),
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
    /// # Examples
    ///
    /// ```
    /// fn action_example() {}
    /// let menu_option = MenuOption::new("Option example", action_example);
    /// let menu = Menu::new(vec![menu_option]);
    /// ```
    ///
    /// You can use closures to easily use arguments in your functions.
    ///
    /// ```
    /// fn action_example(msg: &str, val: f32) {
    ///     println!("action 3 with string {} and float {}", msg, val)
    /// }
    /// let menu_option = MenuOption::new("Option example", || action_example("example", 3.514));
    /// let menu = Menu::new(vec![menu_option]);
    /// ```
    pub fn new(options: Vec<MenuOption>) -> Menu {
        return Menu {
            title: None,
            options: options,
            selected_option: 0,
            normal_style: Style::new(),
            selected_style: Style::new().on_blue(),
            hint_style: Style::new().color256(187),
        };
    }

    /// Sets a title for the menu.
    ///
    /// # Example
    ///
    /// ```
    /// fn action_example() {}
    /// let menu_option = MenuOption::new("Option example", action_example);
    /// let menu = Menu::new(vec![menu_option]).title("Title example");
    /// ```
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

        // return on exit selection
        if self.selected_option == -1 {
            return;
        }

        // runs the action function
        let option = &mut self.options[self.selected_option as usize];
        (option.func)();
    }

    fn menu_navigation(&mut self, stdout: &Term) {
        let options_limit_num: i32 = (self.options.len() - 1) as i32;
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
                    self.selected_option = -1;
                    stdout.show_cursor().unwrap();
                    return;
                }
                Key::Enter => {
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
            let option_idx: usize = self.selected_option as usize;
            let label_style = match i == option_idx {
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
