use bevy::prelude::*;
use bimap::BiMap;

pub struct BevyEguiIdManager;

impl Plugin for BevyEguiIdManager {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(EguiIdManager {
            ids: BiMap::default(),
            current_id: 0,
        })
        .add_system(remove_unloaded_handles);
    }
}

pub struct EguiIdManager {
    ids: BiMap<u64, Handle<Image>>,
    current_id: u64,
}

impl EguiIdManager {
    pub fn get_id(&mut self, handle: Handle<Image>) -> u64 {
        if let Some(id) = self.ids.get_by_right(&handle) {
            return *id;
        }

        loop {
            self.current_id = self.current_id.wrapping_add(1);
            if !self.ids.contains_left(&self.current_id) {
                break;
            }
        }

        self.ids.insert(self.current_id, handle.as_weak());
        self.current_id
    }
}

fn remove_unloaded_handles(
    mut ev_asset: EventReader<AssetEvent<Image>>,
    mut id_manager: ResMut<EguiIdManager>,
) {
    for ev in ev_asset.iter() {
        if let AssetEvent::Removed { handle } = ev {
            id_manager.ids.remove_by_right(handle);
        }
    }
}
