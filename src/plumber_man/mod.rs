use bevy::prelude::PluginGroup;

pub mod movement;
pub mod player;

pub struct PlumberMan;

impl PluginGroup for PlumberMan {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        todo!()
    }
}