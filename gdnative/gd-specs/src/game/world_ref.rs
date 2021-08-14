use specs::prelude::*;

#[derive(NativeClass)]
#[inherit(Reference)]
#[no_constuctor]
pub struct WorldReference {
    world: World
}

#[methods]
impl WorldReference {}