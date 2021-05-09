pub mod list;

pub struct DrawConfig {
    pub sidebar_percentage_size: u16,
    pub main_panel_percentage_size: u16,
}

impl Default for DrawConfig {
    fn default() -> DrawConfig {
        DrawConfig {
            sidebar_percentage_size: 20,
            main_panel_percentage_size: 80,
        }
    }
}
