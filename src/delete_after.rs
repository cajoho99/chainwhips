use bevy::prelude::{Commands, Component, Entity, Query};
use std::time::{Duration, Instant};

#[derive(Component)]
pub struct DeleteAt(pub Instant);

impl DeleteAt {
    pub fn after(duration: Duration) -> Self {
        DeleteAt(Instant::now() + duration)
    }
}

pub fn delete_at(mut commands: Commands, delete_at: Query<(Entity, &DeleteAt)>) {
    let now = Instant::now();
    for (entity, DeleteAt(time)) in delete_at {
        if &now >= time {
            commands.entity(entity).despawn();
        }
    }
}
