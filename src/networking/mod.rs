use bevy::prelude::*;
use std::any::Any;
use std::sync::Mutex;
use steamworks::*;

mod type_registry;

use type_registry::Registered;

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
        app.insert_resource(NetworkingState::new(
            self.max_players,
            self.max_synced_objects,
            self.app_id,
            self.packet_per_frame_limit,
        ))
        .add_systems(Update, handle_networking)
        .add_systems(Update, sync_slave_entities)
        .add_systems(Update, sync_master_entities)
        .add_systems(Update, delete_marked_slaves)
        .add_systems(Update, delete_marked_masters);
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
        packet_per_frame_limit: u32,
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
        self.event_queue_in[event_type as usize]
            .lock()
            .unwrap()
            .drain(..)
            .collect()
    }
}

#[derive(Clone)]
struct SyncMessage {
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
            length: data.len() as u16 + 3,
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
        let data = bytes[3..(length as usize)].to_vec();
        NetworkingEvent {
            event_type,
            length,
            data,
        }
    }
}

use EventType::*;
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
    fn num_variants() -> u8 {
        6
    }
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
    fn get_type_id(&self) -> u16;
}

impl<T> Serializable for T
where
    T: serde::Serialize + serde::de::DeserializeOwned + Send + Sync + Any + Registered + 'static,
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
    fn get_type_id(&self) -> u16 {
        self.registered_id()
    }
}

#[derive(Component)]
pub struct SynchronizedSlave {
    object_info: u8, /*First bit marks whether or not to delete,
                      *Second bit marks whether to sync periodically,
                      */
    static_id: u16,
    destroy_on_owner_disconnect: bool,
    owner: u16,
}

#[derive(Component)]
pub struct SynchronizedMaster {
    object_info: u8, /*First bit marks whether or not to delete,
                      *Second bit marks whether to sync periodically,
                      */
    static_id: u16,
}
impl SynchronizedMaster {
    pub fn destroy(&mut self, networking: &NetworkingState) {
        //sets the first bit which signifies whether to delete to 1 marking it for deletion
        self.object_info |= 0b10000000;

        let mut bytes: Vec<u8> = Vec::new();

        bytes.push(EntityDelete as u8);
        bytes.push(self.static_id.to_le_bytes()[0]);
        bytes.push(self.static_id.to_le_bytes()[1]);

        networking.send_all_reliable(bytes);
    }
}

fn handle_networking(mut networking_res: ResMut<NetworkingState>) {
    if !networking_res.connected {
        return;
    }

    let networking = networking_res.client.networking();

    let mut guard = networking_res.event_queue_out.lock().unwrap();
    let events_to_send: Vec<NetworkingEvent> = guard.drain(..).collect();

    drop(guard); //unlocks the mutex

    events_to_send
        .into_iter()
        .map(|event| {
            networking_res.send_all_reliable(event.to_bytes());
        })
        .for_each(drop);

    let mut i: u32 = 0;
    loop {
        //limits the number of packets read per frame to packet_per_frame_limit
        if i >= networking_res.packet_per_frame_limit {
            break;
        }
        i += 1;

        let is_packet_available = networking.is_p2p_packet_available();
        //if no packet is available, return
        if !is_packet_available.is_some() {
            return;
        }

        //creates a buffer with the size of the packet
        let mut buffer: Vec<u8> = Vec::with_capacity(is_packet_available.unwrap());

        //reads the packet into the buffer
        let (_sender, len) = networking.read_p2p_packet(buffer.as_mut_slice()).unwrap();

        //if the sender is not in the active players list, add them
        match EventType::from(buffer[0]) {
            EntityUpdate => networking_res.sync_messages.push(SyncMessage {
                data: buffer[..len].into(),
            }),
            EntityDelete => networking_res.sync_messages.push(SyncMessage {
                data: buffer[..len].into(),
            }),
            EntityCreate => {
                //TODO create entity
            }
            PlayerJoin => {
                //TODO add player
            }
            PlayerLeave => {
                //TODO remove player and all their entities that are destroy_on_owner_disconnect
                //TODO decide who inherits their entities
            }
            Event => {
                //doesn't include the first byte which is the msg type
                let event = NetworkingEvent::from_bytes(&buffer[1..]);

                let mut queue_in = networking_res.event_queue_in[event.event_type as usize]
                    .lock()
                    .unwrap();
                queue_in.push(event);
            }
        }
    }
}

fn sync_slave_entities(
    mut networking: ResMut<NetworkingState>,
    mut query: Query<(&mut dyn Serializable, &mut SynchronizedSlave)>,
) {
    if !networking.connected {
        return;
    }
    //clones the sync messages to prevent borrowing issues
    let sync_messages = networking.sync_messages.clone();

    for message in sync_messages.into_iter() {
        match message.data[0].into() {
            EntityUpdate => {
                let static_id = u16::from_le_bytes([message.data[1], message.data[2]]);
                for mut entity in query.iter_mut() {
                    if entity.1.static_id == static_id {
                        //skips the first 3 bytes which are the message type and static id
                        let mut i = 3;

                        //used to keep track of the number of components updated
                        let mut n = 0;
                        while i < message.data.len() {
                            const MAX_COMPONENTS_PER_ENTITY: usize = 16; //arbitrary number, should be removed after testing
                            if n > MAX_COMPONENTS_PER_ENTITY {
                                println!("Exceeded maximum ");
                                break;
                            }
                            let component_id =
                                u16::from_le_bytes([message.data[i], message.data[i + 1]]);
                            i += 2;

                            //finds the component with the matching id and updates it
                            for mut component in &mut entity.0 {
                                if component.get_type_id() == component_id {
                                    let len = component.get_length();
                                    component.from_bytes(&message.data[i..i + len]);
                                    i += len;
                                    break;
                                }
                            }
                            n += 1;
                        }
                        break;
                    }
                }
            }
            EntityDelete => {
                let static_id = u16::from_le_bytes([message.data[1], message.data[2]]);
                for mut entity in query.iter_mut() {
                    if entity.1.static_id == static_id {
                        entity.1.object_info |= 0b10000000;
                        break;
                    }
                }
            }
            _ => {
                panic!("Invalid sync message");
            } //Should be impossible to reach
        }
    }

    networking.sync_messages.clear();
}

fn sync_master_entities(
    networking: Res<NetworkingState>,
    query: Query<(&dyn Serializable, &SynchronizedMaster)>,
) {
    if !networking.connected {
        return;
    }

    for entity in query.iter() {
        if (entity.1.object_info & 0b01000000) != 0 {
            //checks whether or not to sync periodically
            let mut bytes: Vec<u8> = Vec::new();

            //Adds header data (message type and static id)
            bytes.push(EntityUpdate as u8);
            bytes.extend_from_slice(&entity.1.static_id.to_le_bytes());

            for component in entity.0 {
                //Adds component type id and component data
                //  (length is constant per type and
                //   therefore doesn't need to be sent)
                bytes.extend_from_slice(&component.get_type_id().to_le_bytes());
                bytes.extend_from_slice(&component.to_bytes());
            }

            //sends all unreliable because it's ok if some packets are dropped
            //because they are sent every frame anyways
            networking.send_all_unreliable(bytes);
        }
    }
}

fn delete_marked_slaves(
    networking: Res<NetworkingState>,
    mut commands: Commands,
    query: Query<(Entity, &SynchronizedSlave)>,
) {
    if !networking.connected {
        return;
    }

    for entity in query.iter() {
        if (entity.1.object_info & 0b10000000) != 0 {
            commands.entity(entity.0).despawn_recursive();
        }
    }
}

fn delete_marked_masters(
    networking: Res<NetworkingState>,
    mut commands: Commands,
    query: Query<(Entity, &SynchronizedMaster)>,
) {
    if !networking.connected {
        return;
    }

    for entity in query.iter() {
        if (entity.1.object_info & 0b10000000) != 0 {
            commands.entity(entity.0).despawn_recursive();
        }
    }
}

impl NetworkingState {
    fn send_all_unreliable(&self, bytes: Vec<u8>) {
        let networking = self.client.networking();

        for player in self.active_players.iter() {
            networking.send_p2p_packet(*player, SendType::Unreliable, &bytes);
        }
    }

    fn send_all_reliable(&self, bytes: Vec<u8>) {
        let networking = self.client.networking();

        for player in self.active_players.iter() {
            networking.send_p2p_packet(*player, SendType::Reliable, &bytes);
        }
    }

    pub fn create_networked_entity(
        &self,
        commands: &mut Commands,
        components: &[Box<impl Serializable>],
        entity: &Entity,
        sync_periodically: bool,
        static_id: u16,
    ) {
        let mut object_info: u8 = 0;
        if sync_periodically {
            object_info |= 0b01000000;
        }

        commands.entity(*entity).insert(SynchronizedMaster {
            object_info,
            static_id,
        });

        let mut bytes: Vec<u8> = Vec::new();

        bytes.push(EntityCreate as u8);
        bytes.extend_from_slice(&static_id.to_le_bytes());
        bytes.push(object_info);
        bytes.push(components.len().try_into().unwrap());

        for component in components {
            let type_id = component.get_type_id().to_le_bytes();
            let comp_bytes = component.to_bytes();

            bytes.extend_from_slice(&type_id);
            bytes.push(comp_bytes.len().try_into().unwrap());
            bytes.extend_from_slice(&comp_bytes);
        }

        self.send_all_reliable(bytes);
    }
}
