
/// This class is used to pass the entity ID around in Godot.
#[derive(NativeClass)]
#[inherit(Reference)]
#[no_constuctor]
pub struct EntityRef {
    entity: Entity
}

#[methods]
impl EntityRef {}


#[derive(NativeClass)]
#[inherit(Node2D)]
#[no_constuctor]
pub struct EntityNode2D {

}

#[methods]
impl EntityNode2D {
    
}