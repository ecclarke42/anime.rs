use generational_arena::Index;

use crate::target::Target;

use super::{instance::Instance, Engine};

#[derive(Debug, Clone)]
pub struct Handle {
    engine: super::internals::EngineRef,
    index: Index,
}

impl Handle {
    /// Delete this animation, removing it from the engine.
    // TODO: probably doesn't need to consume self, but that likely prevents
    // some misuse as the handle is invalid after this call
    pub async fn remove(self) -> Option<Instance> {
        if let Ok(mut guard) = self.engine.write() {
            guard.instances.remove(self.index)
        } else {
            None
        }
    }
}

#[cfg(feature = "animation-futures")]
mod future {
    use std::{
        future::Future,
        pin::Pin,
        task::{Context, Poll},
    };

    use super::Handle;

    // TODO: instead of polling finished, fire with on_complete?

    impl Future for Handle {
        type Output = ();
        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            todo!()
        }
    }
}

// pub enum PropertyValue {
//     // name: UpdateableProp
// }

// pub struct Keyframes<const N_PROPS: usize, const FRAMES: usize> {
//     props: [Property; N_PROPS],
//     frames: Vec<[(); N_PROPS]>, // TODO: value type
// } // TODO: SoA vs AoS for keyframe props
//   // TODO: or do we need props? with a prop enum wrapper they don't take up much room

// pub enum UpdateableProp {}
