use bevy::prelude::*;

pub trait Registered: serde::Serialize + serde::de::DeserializeOwned + Send + Sync + std::any::Any + 'static {
    fn registered_id(&self) -> u16;
}

macro_rules! register {
    ($type:ty, $id:expr) => {
        impl Registered for $type {
            fn registered_id(&self) -> u16 {
                $id
            }
        }
    };
}

register!(Transform, 0);
// TODO: register all types here