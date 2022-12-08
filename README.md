# menu_rs

This is library for Rust that allows the creation of simple and interactable command-line menus.

It's very simple to use, you just create a Menu, adds the option you want it to have with the correspondent
action to be run when selected and that's it!
You can use the arrow keys to move through the options, ENTER to select an option and ESC to exit the menu.

![menu_example](https://user-images.githubusercontent.com/22588915/206564880-1e3c89e6-8b84-4ded-8b93-bd0fa7271562.gif)

### Example

```rust
use menu_rs::{Menu, MenuOption};

fn main() {
    fn action_1() {
        println!("Option 1 called!");
    }
    fn action_2() {}
    fn action_3() {}
    fn action_4() {}

    let menu = Menu::new(vec![
        MenuOption::new("Option 1", action_1).hint("Hint for option 1"),
        MenuOption::new("Option 2", action_2),
        MenuOption::new("Option 3", action_3),
        MenuOption::new("Option 4", action_4),
    ]);

    menu.show();
}
```
