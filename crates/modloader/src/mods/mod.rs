use std::{future::Future, path::Path};

use bevy_app::{App, Plugin};
use bevy_ecs_macros::Resource;
use bevy_tasks::{AsyncComputeTaskPool, Task};

mod loading;
use loading::{LoadingMod, LoadingModResult};

pub(crate) struct ModPlugin;

impl Plugin for ModPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Mods>();
    }
}

#[derive(Resource, Default)]
pub struct Mods {
    loading: Vec<Task<LoadingModResult>>,
    // TODO
}

impl Mods {
    pub fn load_from_path<P>(&mut self, path: P)
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref().to_owned();
        self.enque_loading(LoadingMod::try_from_path(path))
    }

    fn enque_loading(&mut self, future: impl Future<Output = LoadingModResult> + Send + 'static) {
        let thread_pool = AsyncComputeTaskPool::get();
        let task = thread_pool.spawn(future);
        self.loading.push(task);
    }
}