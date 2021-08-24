use rapier2d::prelude::*;
/// This is a container for all of the Physics related structs that will be injected into the relevant World
pub struct RapierPhysicsResource {
    pub (crate) gravity: Vector<f32>,
    pub (crate) integration_parameters: IntegrationParameters,
    pub (crate) rigid_bodies: RigidBodySet,
    pub (crate) colliders: ColliderSet,
    pub (crate) islands: IslandManager,
    pub (crate) broad_phase: BroadPhase,
    pub (crate) narrow_phase: NarrowPhase,
    pub (crate) joints: JointSet,
    pub (crate) ccd_solver: CCDSolver,
    pipeline: PhysicsPipeline,
}

impl RapierPhysicsResource {
    pub fn default() -> Self {
        Self {
            gravity: vector![0.0, 0.0],
            integration_parameters: IntegrationParameters::default(),
            rigid_bodies: RigidBodySet::new(),
            colliders: ColliderSet::new(),
            islands: IslandManager::new(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            joints: JointSet::new(),
            ccd_solver: CCDSolver::new(),
            pipeline: PhysicsPipeline::new(),
        }
    }
    pub fn run(mut self) {
        self.pipeline.step(
            &mut self.gravity, 
            &mut self.integration_parameters,
            &mut self.islands,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_bodies,
            &mut self.colliders,
            &mut self.joints,
            &mut self.ccd_solver,
            // Neither hooks nor events are necessary for this
            &(),
            &())
    }
}