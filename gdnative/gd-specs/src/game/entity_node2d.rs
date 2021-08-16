
/// This is a Variantable Entity reference that can be safely passed into GDScript for reference by other functions.
#[derive(NativeClass)]
#[inherit(Reference)]
#[no_constuctor]
pub struct EntityRef {
    entity: Entity
}

#[methods]
impl EntityRef {}

/// This is a GodotAccessible Entity that is tailor made for use with 
#[derive(NativeClass)]
#[inherit(Node2D)]
#[no_constuctor]
pub struct EntityNode2D {
    pub (crate) components: HashMap<String, Component>,
    entity: Entity
}

#[methods]
impl EntityNode2D {
    
    pub fn entity(&self, _: &Node2D) -> Instance<EntityRef> {
        EntityRef {

        }
    }
}