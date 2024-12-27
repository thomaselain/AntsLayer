use crate::Menu;

impl Menu {
    pub fn map_editor(self) {
        'map_editor: loop {
            // Display menu

            // Catch mousebutton with menu.buttons and SDL events

            // open file (or create new world)
            // Self::load_world(clicked_button.output())

            // Update Map
            // Map::delete_world(self.map.unwrap().name);
            //self.map = Map::new();

            // Load world
            // self.load_world();

            // List of all biomesettings
            //  -      base height          +
            //  -     height variation      +

            //           biomes

            //           name
            //           etc ...

            break 'map_editor;
        }
    }
}
