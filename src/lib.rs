use bevy::prelude::*;
use bimap::BiMap;

pub struct BevyEguiIdManager;

impl Plugin for BevyEguiIdManager {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(EguiIdManager {
            ids: BiMap::default(),
            current_id: 0,
        });
    }
}

pub struct EguiIdManager {
    ids: BiMap<u64, Handle<Image>>,
    current_id: u64,
}

impl EguiIdManager {
    pub fn get_id(&mut self, handle: Handle<Image>, asset_server: &AssetServer) -> u64 {
        if let Some(id) = self.ids.get_by_right(&handle) {
            return *id;
        }

        loop {
            self.current_id = self.current_id.wrapping_add(1);
            match self.ids.get_by_left(&self.current_id) {
                Some(handle) => match asset_server.get_load_state(handle) {
                    bevy::asset::LoadState::Unloaded => break,
                    _ => (),
                },
                None => break,
            }
        }

        self.ids.insert(self.current_id, handle.as_weak());
        self.current_id
    }
}
