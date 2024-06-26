use bevy::prelude::*;
use rs_openai::client;
use steamworks::*;
use std::sync::Mutex;
use std::any::{ Any, TypeId };

pub struct NetworkingPlugin {
    pub max_players: u16,
    pub max_synced_objects: u32,
    pub app_id: u32,
    pub packet_per_frame_limit: u32,
}

impl Default for NetworkingPlugin {
    fn default() -> Self {
        NetworkingPlugin {
            max_players: 32,
            max_synced_objects: 1024,
            app_id: 480,
            packet_per_frame_limit: 64,
        }
    }
}

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(NetworkingState::new(
                self.max_players,
                self.max_synced_objects,
                self.app_id,
                self.packet_per_frame_limit,
            ));
    }
}

#[derive(Resource)]
pub struct NetworkingState {
    pub max_players: u16,
    pub max_synced_objects: u32,
    pub packet_per_frame_limit: u32,

    pub connected: bool,
    pub client: Client,
    pub player_id: SteamId,
    pub active_players: Vec<SteamId>,

    sync_messages: Vec<SyncMessage>,
    event_queue_out: Mutex<Vec<NetworkingEvent>>,
    event_queue_in: Vec<Mutex<Vec<NetworkingEvent>>>, // The index of the outer vector is the event type
}

impl NetworkingState {
    pub fn new(
        max_players: u16,
        max_synced_objects: u32,
        app_id: u32,
        packet_per_frame_limit: u32
    ) -> Self {
        let (client, _) = Client::init_app(app_id).unwrap();
        let player_id = client.user().steam_id();

        Self {
            max_players,
            max_synced_objects,
            packet_per_frame_limit,
            connected: false,
            client,
            player_id,
            active_players: Vec::new(),
            sync_messages: Vec::new(),
            event_queue_out: Mutex::new(Vec::new()),
            event_queue_in: vec![0; EventType::num_variants() as usize]
                .into_iter()
                .map(|_| Mutex::new(Vec::new()))
                .collect::<Vec<Mutex<Vec<NetworkingEvent>>>>(),
        }
    }
}

impl NetworkingState {
    pub fn queue_event_out(&self, event: NetworkingEvent) {
        self.event_queue_out.lock().unwrap().push(event);
    }
    pub fn get_event_in(&self, event_type: u8) -> Vec<NetworkingEvent> {
        self.event_queue_in[event_type as usize].lock().unwrap().drain(..).collect()
    }
}

struct SyncMessage {
    blocking: bool, // If true, the frame will not progress until this message is processed
    data: Vec<u8>,
}

#[derive(Clone)]
pub struct NetworkingEvent {
    pub event_type: EventType,
    length: u16,
    pub data: Vec<u8>,
}

impl NetworkingEvent {
    fn new(event_type: EventType, data: Vec<u8>) -> NetworkingEvent {
        NetworkingEvent {
            event_type,
            length: data.len() as u16,
            data,
        }
    }
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.push(self.event_type as u8);
        bytes.push(self.length.to_le_bytes()[0]);
        bytes.push(self.length.to_le_bytes()[1]);
        bytes.append(&mut self.data.clone().into());
        bytes
    }
    fn from_bytes(bytes: &[u8]) -> NetworkingEvent {
        let event_type = bytes[0].into();
        let length = u16::from_le_bytes([bytes[1], bytes[2]]);
        let data = bytes[3..(length as usize + 3)].to_vec();
        NetworkingEvent {
            event_type,
            length,
            data,
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum EventType {
    EntityCreate,
    EntityDelete,
    EntityUpdate,
    PlayerJoin,
    PlayerLeave,
    Event,
}

impl EventType {
    #[inline]
    fn num_variants() -> u8 { 6 }
}

impl From<u8> for EventType {
    fn from(value: u8) -> Self {
        match value {
            0 => EventType::EntityCreate,
            1 => EventType::EntityDelete,
            2 => EventType::EntityUpdate,
            3 => EventType::PlayerJoin,
            4 => EventType::PlayerLeave,
            5 => EventType::Event,
            _ => panic!("Invalid EventType"),
        }
    }
}

#[bevy_trait_query::queryable]
pub trait Serializable: Send + Sync + Any {
    fn from_bytes(&mut self, bytes: &[u8]);
    fn to_bytes(&self) -> Vec<u8>;
    
    fn get_length(&self) -> usize;
    //used to identify the type of the component when synchronizing
    fn get_type_id(&self) -> TypeId;
}

impl<T> Serializable for T
where
    T: serde::Serialize + serde::de::DeserializeOwned + Send + Sync + Any + 'static
{
    fn from_bytes(&mut self, bytes: &[u8]) {
        *self = bincode::deserialize(bytes).unwrap();
    }
    fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }
    fn get_length(&self) -> usize {
        bincode::serialize(self).unwrap().len()
    }
    fn get_type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }
}