use tauri::{CustomMenuItem, Menu, MenuItem, Submenu, WindowMenuEvent};
fn theme_app_menu() -> Submenu {
    let light = CustomMenuItem::new("light".to_string(), "light");
    let dark = CustomMenuItem::new("dark".to_string(), "dark");
    let cupcake = CustomMenuItem::new("cupcake".to_string(), "cupcake");
    let bumblebee = CustomMenuItem::new("bumblebee".to_string(), "bumblebee");
    let emerald = CustomMenuItem::new("emerald".to_string(), "emerald");
    let corporate = CustomMenuItem::new("corporate".to_string(), "corporate");
    let synthwave = CustomMenuItem::new("synthwave".to_string(), "synthwave");
    let retro = CustomMenuItem::new("retro".to_string(), "retro");
    let cyberpunk = CustomMenuItem::new("cyberpunk".to_string(), "cyberpunk");
    let valentine = CustomMenuItem::new("valentine".to_string(), "valentine");
    let halloween = CustomMenuItem::new("halloween".to_string(), "halloween");
    let garden = CustomMenuItem::new("garden".to_string(), "garden");
    let forest = CustomMenuItem::new("forest".to_string(), "forest");
    let aqua = CustomMenuItem::new("aqua".to_string(), "aqua");
    let lofi = CustomMenuItem::new("lofi".to_string(), "lofi");
    let pastel = CustomMenuItem::new("pastel".to_string(), "pastel");
    let fantasy = CustomMenuItem::new("fantasy".to_string(), "fantasy");
    let wireframe = CustomMenuItem::new("wireframe".to_string(), "wireframe");
    let black = CustomMenuItem::new("black".to_string(), "black");
    let luxury = CustomMenuItem::new("luxury".to_string(), "luxury");
    let dracula = CustomMenuItem::new("dracula".to_string(), "dracula");
    let cmyk = CustomMenuItem::new("cmyk".to_string(), "cmyk");
    let autumn = CustomMenuItem::new("autumn".to_string(), "autumn");
    let business = CustomMenuItem::new("business".to_string(), "business");
    let acid = CustomMenuItem::new("acid".to_string(), "acid");
    let lemonade = CustomMenuItem::new("lemonade".to_string(), "lemonade");
    let night = CustomMenuItem::new("night".to_string(), "night");
    let coffee = CustomMenuItem::new("coffee".to_string(), "coffee");
    let winter = CustomMenuItem::new("winter".to_string(), "winter");
    let theme_submenu = Submenu::new(
        "Theme",
        Menu::new()
            .add_item(light)
            .add_item(dark)
            .add_item(cupcake)
            .add_item(bumblebee)
            .add_item(emerald)
            .add_item(corporate)
            .add_item(synthwave)
            .add_item(retro)
            .add_item(cyberpunk)
            .add_item(valentine)
            .add_item(halloween)
            .add_item(garden)
            .add_item(forest)
            .add_item(aqua)
            .add_item(lofi)
            .add_item(pastel)
            .add_item(fantasy)
            .add_item(wireframe)
            .add_item(black)
            .add_item(luxury)
            .add_item(dracula)
            .add_item(cmyk)
            .add_item(autumn)
            .add_item(business)
            .add_item(acid)
            .add_item(lemonade)
            .add_item(night)
            .add_item(coffee)
            .add_item(winter),
    );
    theme_submenu
}
pub fn build_app_menu() -> Menu {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let close = CustomMenuItem::new("close".to_string(), "Close");
    let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));
    let theme_submenu = theme_app_menu();
    let menu = Menu::new()
        .add_native_item(MenuItem::Copy)
        .add_item(CustomMenuItem::new("hide", "Hide"))
        .add_submenu(submenu)
        .add_submenu(theme_submenu);
    menu
}
pub fn handle_app_menu(event: WindowMenuEvent) {
    match event.menu_item_id() {
        "quit" => {
            std::process::exit(0);
        }
        "close" => {
            event.window().close().unwrap();
        }
        "light" => {
            event.window().emit("menu-selected", "light").unwrap();
        }
        "dark" => {
            event.window().emit("menu-selected", "dark").unwrap();
        }
        "cupcake" => {
            event.window().emit("menu-selected", "cupcake").unwrap();
        }
        "bumblebee" => {
            event.window().emit("menu-selected", "bumblebee").unwrap();
        }
        "emerald" => {
            event.window().emit("menu-selected", "emerald").unwrap();
        }
        "corporate" => {
            event.window().emit("menu-selected", "corporate").unwrap();
        }
        "synthwave" => {
            event.window().emit("menu-selected", "synthwave").unwrap();
        }
        "retro" => {
            event.window().emit("menu-selected", "retro").unwrap();
        }
        "cyberpunk" => {
            event.window().emit("menu-selected", "cyberpunk").unwrap();
        }
        "valentine" => {
            event.window().emit("menu-selected", "valentine").unwrap();
        }
        "halloween" => {
            event.window().emit("menu-selected", "halloween").unwrap();
        }
        "garden" => {
            event.window().emit("menu-selected", "garden").unwrap();
        }
        "forest" => {
            event.window().emit("menu-selected", "forest").unwrap();
        }
        "aqua" => {
            event.window().emit("menu-selected", "aqua").unwrap();
        }
        "lofi" => {
            event.window().emit("menu-selected", "lofi").unwrap();
        }
        "pastel" => {
            event.window().emit("menu-selected", "pastel").unwrap();
        }
        "fantasy" => {
            event.window().emit("menu-selected", "fantasy").unwrap();
        }
        "wireframe" => {
            event.window().emit("menu-selected", "wireframe").unwrap();
        }
        "black" => {
            event.window().emit("menu-selected", "black").unwrap();
        }
        "luxury" => {
            event.window().emit("menu-selected", "luxury").unwrap();
        }
        "dracula" => {
            event.window().emit("menu-selected", "dracula").unwrap();
        }
        "cmyk" => {
            event.window().emit("menu-selected", "cmyk").unwrap();
        }
        "autumn" => {
            event.window().emit("menu-selected", "autumn").unwrap();
        }
        "business" => {
            event.window().emit("menu-selected", "business").unwrap();
        }
        "acid" => {
            event.window().emit("menu-selected", "acid").unwrap();
        }
        "lemonade" => {
            event.window().emit("menu-selected", "lemonade").unwrap();
        }
        "night" => {
            event.window().emit("menu-selected", "night").unwrap();
        }
        "coffee" => {
            event.window().emit("menu-selected", "coffee").unwrap();
        }
        "winter" => {
            event.window().emit("menu-selected", "coffee").unwrap();
        }
        _ => {}
    }
}
