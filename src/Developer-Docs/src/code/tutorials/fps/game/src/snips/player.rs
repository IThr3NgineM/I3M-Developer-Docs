// ANCHOR: player_imports
use i3m::graph::SceneGraph;
use i3m::{
    core::{
        algebra::{UnitQuaternion, UnitVector3, Vector3},
        pool::Handle,
        reflect::prelude::*,
        type_traits::prelude::*,
        variable::InheritableVariable,
        visitor::prelude::*,
    },
    event::{DeviceEvent, ElementState, Event, MouseButton, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
    scene::{node::Node, rigidbody::RigidBody},
    script::{ScriptContext, ScriptTrait, ScriptDeinitContext},
    
};

// ANCHOR_END: player_imports