use crate::{ ant::{ colony::Colony, explorer::Explorer, worker::Worker, ColonyMember }, renderer::Renderer };

impl Renderer<'_> {
    pub fn draw_ants(&mut self, colony: &Colony, timestamp: f64) {
        colony.queen.clone().render(self);
        for ant in colony.ants.iter() {
            println!("caca");

            if let Some(worker) = ant.as_ref().as_any().downcast_ref::<Worker>() {
                worker.render(self);
                println!("Drawing ants !");
            } else if let Some(explorer) = ant.as_ref().as_any().downcast_ref::<Explorer>() {
                explorer.render(self);
            }
        }
    }
}
