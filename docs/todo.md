# To Do list

- [ ] Imeplement ECS Transform functionality
  - [ ] Add scale component
  - [ ] Add rotation components
  - [ ] Add systems that can modify the rotation and scale.
- [ ] Visual Server Systems
  - [ ] CanvasItemUpdateSystem
    - [ ] System should check for CanvasItems that have been added or modified via flagged storage. This should also create
  - [ ] Update Transform
  - [ ] Update ShaderParams
- [ ] Hybrid System
  - [ ] GDHybridEntity native class that can be instanced and contains all the node/scene data for use with the godot editor.
    - [ ] GDHybridEntity leverages godot to create the necessary canvas entities. 
    - [ ] Register all components settings from the editor.
    - [ ] Free itself and associated references when the entity is no longer alive.
  - [ ] Specs systems update Visual Server updates the transform, rotation, scale, material, etc directly