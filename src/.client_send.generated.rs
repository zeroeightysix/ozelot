/* This file is automatically generated by packets.clj
Do not manually edit this file, if you wish to make
changes here, then edit and rerun packets.clj */

#[derive(Debug, PartialEq, Clone)]
pub struct Handshake {
    protocol_version: i32,
    server_address: String,
    server_port: u16,
    next_state: i32,
}
impl Handshake {
    pub fn new(protocol_version: i32, server_address: String, server_port: u16, next_state: i32) -> Self {
        Handshake {
            protocol_version: protocol_version,
            server_address: server_address,
            server_port: server_port,
            next_state: next_state,
        }
    }
}
impl Sendable for Handshake {
    fn to_u8(&self) -> io::Result<Vec<u8>> {
        let mut ret = Vec::new();
        write_varint(&Handshake::get_id(), &mut ret)?;
        write_varint(&self.protocol_version, &mut ret)?;
        write_String(&self.server_address, &mut ret)?;
        write_u16(&self.server_port, &mut ret)?;
        write_varint(&self.next_state, &mut ret)?;
        Ok(ret)
    }
    fn get_state() -> ClientState {
        ClientState::Handshake
    }
    fn get_id() -> i32 {
        0
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct StatusRequest {
}
impl StatusRequest {
    pub fn new() -> Self {
        StatusRequest {
        }
    }
}
impl Sendable for StatusRequest {
    fn to_u8(&self) -> io::Result<Vec<u8>> {
        let mut ret = Vec::new();
        write_varint(&StatusRequest::get_id(), &mut ret)?;
        Ok(ret)
    }
    fn get_state() -> ClientState {
        ClientState::Status
    }
    fn get_id() -> i32 {
        0
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct StatusPing {
    id: u64,
}
impl StatusPing {
    pub fn new(id: u64) -> Self {
        StatusPing {
            id: id,
        }
    }
}
impl Sendable for StatusPing {
    fn to_u8(&self) -> io::Result<Vec<u8>> {
        let mut ret = Vec::new();
        write_varint(&StatusPing::get_id(), &mut ret)?;
        write_u64(&self.id, &mut ret)?;
        Ok(ret)
    }
    fn get_state() -> ClientState {
        ClientState::Status
    }
    fn get_id() -> i32 {
        1
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct LoginStart {
    name: String,
}
impl LoginStart {
    pub fn new(name: String) -> Self {
        LoginStart {
            name: name,
        }
    }
}
impl Sendable for LoginStart {
    fn to_u8(&self) -> io::Result<Vec<u8>> {
        let mut ret = Vec::new();
        write_varint(&LoginStart::get_id(), &mut ret)?;
        write_String(&self.name, &mut ret)?;
        Ok(ret)
    }
    fn get_state() -> ClientState {
        ClientState::Login
    }
    fn get_id() -> i32 {
        0
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EncryptionResponse {
    shared_secret: Vec<u8>,
    verify_token: Vec<u8>,
}
impl EncryptionResponse {
    pub fn new(shared_secret: Vec<u8>, verify_token: Vec<u8>) -> Self {
        EncryptionResponse {
            shared_secret: shared_secret,
            verify_token: verify_token,
        }
    }
}
impl Sendable for EncryptionResponse {
    fn to_u8(&self) -> io::Result<Vec<u8>> {
        let mut ret = Vec::new();
        write_varint(&EncryptionResponse::get_id(), &mut ret)?;
        write_prefixed_bytearray(&self.shared_secret, &mut ret)?;
        write_prefixed_bytearray(&self.verify_token, &mut ret)?;
        Ok(ret)
    }
    fn get_state() -> ClientState {
        ClientState::Login
    }
    fn get_id() -> i32 {
        1
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TeleportConfirm {
    id: i32,
}
impl TeleportConfirm {
    pub fn new(id: i32) -> Self {
        TeleportConfirm {
            id: id,
        }
    }
}
impl Sendable for TeleportConfirm {
    fn to_u8(&self) -> io::Result<Vec<u8>> {
        let mut ret = Vec::new();
        write_varint(&TeleportConfirm::get_id(), &mut ret)?;
        write_varint(&self.id, &mut ret)?;
        Ok(ret)
    }
    fn get_state() -> ClientState {
        ClientState::Play
    }
    fn get_id() -> i32 {
        0
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TabComplete {
    text: String,
    assume_command: bool,
    looked_at_block: Option<u64>,
}
impl TabComplete {
    pub fn new(text: String, assume_command: bool, looked_at_block: Option<u64>) -> Self {
        TabComplete {
            text: text,
            assume_command: assume_command,
            looked_at_block: looked_at_block,
        }
    }
}
impl Sendable for TabComplete {
    fn to_u8(&self) -> io::Result<Vec<u8>> {
        /* This is because we can't split the implementation of a trait
        * across more than one impl block, so when we want to create a custom
        * to_u8() implementation, we name it to_u8_custom and then just have this
        * default to calling that */
        self.to_u8_custom()
    }
    fn get_state() -> ClientState {
        ClientState::Play
    }
    fn get_id() -> i32 {
        1
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ChatMessage {
    message: String,
}
impl ChatMessage {
    pub fn new(message: String) -> Self {
        ChatMessage {
            message: message,
        }
    }
}
impl Sendable for ChatMessage {
    fn to_u8(&self) -> io::Result<Vec<u8>> {
        let mut ret = Vec::new();
        write_varint(&ChatMessage::get_id(), &mut ret)?;
        write_String(&self.message, &mut ret)?;
        Ok(ret)
    }
    fn get_state() -> ClientState {
        ClientState::Play
    }
    fn get_id() -> i32 {
        2
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ClientStatus {
    action: i32,
}
impl ClientStatus {
    pub fn new(action: i32) -> Self {
        ClientStatus {
            action: action,
        }
    }
}
impl Sendable for ClientStatus {
    fn to_u8(&self) -> io::Result<Vec<u8>> {
        let mut ret = Vec::new();
        write_varint(&ClientStatus::get_id(), &mut ret)?;
        write_varint(&self.action, &mut ret)?;
        Ok(ret)
    }
    fn get_state() -> ClientState {
        ClientState::Play
    }
    fn get_id() -> i32 {
        3
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ClientSettings {
    locale: String,
    view_distance: u8,
    chat_mode: i32,
    chat_colors: bool,
    displayed_skin_parts: u8,
    main_hand: i32,
}
impl ClientSettings {
    pub fn new(locale: String, view_distance: u8, chat_mode: i32, chat_colors: bool, displayed_skin_parts: u8, main_hand: i32) -> Self {
        ClientSettings {
            locale: locale,
            view_distance: view_distance,
            chat_mode: chat_mode,
            chat_colors: chat_colors,
            displayed_skin_parts: displayed_skin_parts,
            main_hand: main_hand,
        }
    }
}
impl Sendable for ClientSettings {
    fn to_u8(&self) -> io::Result<Vec<u8>> {
        let mut ret = Vec::new();
        write_varint(&ClientSettings::get_id(), &mut ret)?;
        write_String(&self.locale, &mut ret)?;
        write_u8(&self.view_distance, &mut ret)?;
        write_varint(&self.chat_mode, &mut ret)?;
        write_bool(&self.chat_colors, &mut ret)?;
        write_u8(&self.displayed_skin_parts, &mut ret)?;
        write_varint(&self.main_hand, &mut ret)?;
        Ok(ret)
    }
    fn get_state() -> ClientState {
        ClientState::Play
    }
    fn get_id() -> i32 {
        4
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ConfirmTransaction {
    window_id: u8,
    id: i16,
    accepted: bool,
}
impl ConfirmTransaction {
    pub fn new(window_id: u8, id: i16, accepted: bool) -> Self {
        ConfirmTransaction {
            window_id: window_id,
            id: id,
            accepted: accepted,
        }
    }
}
impl Sendable for ConfirmTransaction {
    fn to_u8(&self) -> io::Result<Vec<u8>> {
        let mut ret = Vec::new();
        write_varint(&ConfirmTransaction::get_id(), &mut ret)?;
        write_u8(&self.window_id, &mut ret)?;
        write_i16(&self.id, &mut ret)?;
        write_bool(&self.accepted, &mut ret)?;
        Ok(ret)
    }
    fn get_state() -> ClientState {
        ClientState::Play
    }
    fn get_id() -> i32 {
        5
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EnchantItem {
    window_id: u8,
    enchantment: i8,
}
impl EnchantItem {
    pub fn new(window_id: u8, enchantment: i8) -> Self {
        EnchantItem {
            window_id: window_id,
            enchantment: enchantment,
        }
    }
}
impl Sendable for EnchantItem {
    fn to_u8(&self) -> io::Result<Vec<u8>> {
        let mut ret = Vec::new();
        write_varint(&EnchantItem::get_id(), &mut ret)?;
        write_u8(&self.window_id, &mut ret)?;
        write_i8(&self.enchantment, &mut ret)?;
        Ok(ret)
    }
    fn get_state() -> ClientState {
        ClientState::Play
    }
    fn get_id() -> i32 {
        6
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ClickWindow {
    window_id: u8,
    slot_id: i16,
    button: i8,
    id: i16,
    mode: i32,
    slot: Vec<u8>,
}
impl ClickWindow {
    pub fn new(window_id: u8, slot_id: i16, button: i8, id: i16, mode: i32, slot: Vec<u8>) -> Self {
        ClickWindow {
            window_id: window_id,
            slot_id: slot_id,
            button: button,
            id: id,
            mode: mode,
            slot: slot,
        }
    }
}
impl Sendable for ClickWindow {
    fn to_u8(&self) -> io::Result<Vec<u8>> {
        let mut ret = Vec::new();
        write_varint(&ClickWindow::get_id(), &mut ret)?;
        write_u8(&self.window_id, &mut ret)?;
        write_i16(&self.slot_id, &mut ret)?;
        write_i8(&self.button, &mut ret)?;
        write_i16(&self.id, &mut ret)?;
        write_varint(&self.mode, &mut ret)?;
        write_bytearray(&self.slot, &mut ret)?;
        Ok(ret)
    }
    fn get_state() -> ClientState {
        ClientState::Play
    }
    fn get_id() -> i32 {
        7
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CloseWindow {
    window_id: u8,
}
impl CloseWindow {
    pub fn new(window_id: u8) -> Self {
        CloseWindow {
            window_id: window_id,
        }
    }
}
impl Sendable for CloseWindow {
    fn to_u8(&self) -> io::Result<Vec<u8>> {
        let mut ret = Vec::new();
        write_varint(&CloseWindow::get_id(), &mut ret)?;
        write_u8(&self.window_id, &mut ret)?;
        Ok(ret)
    }
    fn get_state() -> ClientState {
        ClientState::Play
    }
    fn get_id() -> i32 {
        8
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PluginMessage {
    channel: String,
    data: Vec<u8>,
}
impl PluginMessage {
    pub fn new(channel: String, data: Vec<u8>) -> Self {
        PluginMessage {
            channel: channel,
            data: data,
        }
    }
}
impl Sendable for PluginMessage {
    fn to_u8(&self) -> io::Result<Vec<u8>> {
        let mut ret = Vec::new();
        write_varint(&PluginMessage::get_id(), &mut ret)?;
        write_String(&self.channel, &mut ret)?;
        write_bytearray(&self.data, &mut ret)?;
        Ok(ret)
    }
    fn get_state() -> ClientState {
        ClientState::Play
    }
    fn get_id() -> i32 {
        9
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct UseEntity {
    target: i32,
    action: i32,
    location: Option<(f32, f32, f32)>,
    hand: Option<i32>,
}
impl UseEntity {
    pub fn new(target: i32, action: i32, location: Option<(f32, f32, f32)>, hand: Option<i32>) -> Self {
        UseEntity {
            target: target,
            action: action,
            location: location,
            hand: hand,
        }
    }
}
impl Sendable for UseEntity {
    fn to_u8(&self) -> io::Result<Vec<u8>> {
        /* This is because we can't split the implementation of a trait
        * across more than one impl block, so when we want to create a custom
        * to_u8() implementation, we name it to_u8_custom and then just have this
        * default to calling that */
        self.to_u8_custom()
    }
    fn get_state() -> ClientState {
        ClientState::Play
    }
    fn get_id() -> i32 {
        10
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct KeepAlive {
    id: i32,
}
impl KeepAlive {
    pub fn new(id: i32) -> Self {
        KeepAlive {
            id: id,
        }
    }
}
impl Sendable for KeepAlive {
    fn to_u8(&self) -> io::Result<Vec<u8>> {
        let mut ret = Vec::new();
        write_varint(&KeepAlive::get_id(), &mut ret)?;
        write_varint(&self.id, &mut ret)?;
        Ok(ret)
    }
    fn get_state() -> ClientState {
        ClientState::Play
    }
    fn get_id() -> i32 {
        11
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PlayerPosition {
    x: f64,
    y: f64,
    z: f64,
    on_ground: bool,
}
impl PlayerPosition {
    pub fn new(x: f64, y: f64, z: f64, on_ground: bool) -> Self {
        PlayerPosition {
            x: x,
            y: y,
            z: z,
            on_ground: on_ground,
        }
    }
}
impl Sendable for PlayerPosition {
    fn to_u8(&self) -> io::Result<Vec<u8>> {
        let mut ret = Vec::new();
        write_varint(&PlayerPosition::get_id(), &mut ret)?;
        write_f64(&self.x, &mut ret)?;
        write_f64(&self.y, &mut ret)?;
        write_f64(&self.z, &mut ret)?;
        write_bool(&self.on_ground, &mut ret)?;
        Ok(ret)
    }
    fn get_state() -> ClientState {
        ClientState::Play
    }
    fn get_id() -> i32 {
        12
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PlayerPositionAndLook {
    x: f64,
    y: f64,
    z: f64,
    yaw: f32,
    pitch: f32,
    on_ground: bool,
}
impl PlayerPositionAndLook {
    pub fn new(x: f64, y: f64, z: f64, yaw: f32, pitch: f32, on_ground: bool) -> Self {
        PlayerPositionAndLook {
            x: x,
            y: y,
            z: z,
            yaw: yaw,
            pitch: pitch,
            on_ground: on_ground,
        }
    }
}
impl Sendable for PlayerPositionAndLook {
    fn to_u8(&self) -> io::Result<Vec<u8>> {
        let mut ret = Vec::new();
        write_varint(&PlayerPositionAndLook::get_id(), &mut ret)?;
        write_f64(&self.x, &mut ret)?;
        write_f64(&self.y, &mut ret)?;
        write_f64(&self.z, &mut ret)?;
        write_f32(&self.yaw, &mut ret)?;
        write_f32(&self.pitch, &mut ret)?;
        write_bool(&self.on_ground, &mut ret)?;
        Ok(ret)
    }
    fn get_state() -> ClientState {
        ClientState::Play
    }
    fn get_id() -> i32 {
        13
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PlayerLook {
    yaw: f32,
    pitch: f32,
    on_ground: bool,
}
impl PlayerLook {
    pub fn new(yaw: f32, pitch: f32, on_ground: bool) -> Self {
        PlayerLook {
            yaw: yaw,
            pitch: pitch,
            on_ground: on_ground,
        }
    }
}
impl Sendable for PlayerLook {
    fn to_u8(&self) -> io::Result<Vec<u8>> {
        let mut ret = Vec::new();
        write_varint(&PlayerLook::get_id(), &mut ret)?;
        write_f32(&self.yaw, &mut ret)?;
        write_f32(&self.pitch, &mut ret)?;
        write_bool(&self.on_ground, &mut ret)?;
        Ok(ret)
    }
    fn get_state() -> ClientState {
        ClientState::Play
    }
    fn get_id() -> i32 {
        14
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Player {
    on_ground: bool,
}
impl Player {
    pub fn new(on_ground: bool) -> Self {
        Player {
            on_ground: on_ground,
        }
    }
}
impl Sendable for Player {
    fn to_u8(&self) -> io::Result<Vec<u8>> {
        let mut ret = Vec::new();
        write_varint(&Player::get_id(), &mut ret)?;
        write_bool(&self.on_ground, &mut ret)?;
        Ok(ret)
    }
    fn get_state() -> ClientState {
        ClientState::Play
    }
    fn get_id() -> i32 {
        15
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct VehicleMove {
    x: f64,
    y: f64,
    z: f64,
    yaw: f32,
    pitch: f32,
}
impl VehicleMove {
    pub fn new(x: f64, y: f64, z: f64, yaw: f32, pitch: f32) -> Self {
        VehicleMove {
            x: x,
            y: y,
            z: z,
            yaw: yaw,
            pitch: pitch,
        }
    }
}
impl Sendable for VehicleMove {
    fn to_u8(&self) -> io::Result<Vec<u8>> {
        let mut ret = Vec::new();
        write_varint(&VehicleMove::get_id(), &mut ret)?;
        write_f64(&self.x, &mut ret)?;
        write_f64(&self.y, &mut ret)?;
        write_f64(&self.z, &mut ret)?;
        write_f32(&self.yaw, &mut ret)?;
        write_f32(&self.pitch, &mut ret)?;
        Ok(ret)
    }
    fn get_state() -> ClientState {
        ClientState::Play
    }
    fn get_id() -> i32 {
        16
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SteerBoat {
    right: bool,
    left: bool,
}
impl SteerBoat {
    pub fn new(right: bool, left: bool) -> Self {
        SteerBoat {
            right: right,
            left: left,
        }
    }
}
impl Sendable for SteerBoat {
    fn to_u8(&self) -> io::Result<Vec<u8>> {
        let mut ret = Vec::new();
        write_varint(&SteerBoat::get_id(), &mut ret)?;
        write_bool(&self.right, &mut ret)?;
        write_bool(&self.left, &mut ret)?;
        Ok(ret)
    }
    fn get_state() -> ClientState {
        ClientState::Play
    }
    fn get_id() -> i32 {
        17
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PlayerAbilities {
    flags: u8,
    flying_speed: f32,
    walking_speed: f32,
}
impl PlayerAbilities {
    pub fn new(flags: u8, flying_speed: f32, walking_speed: f32) -> Self {
        PlayerAbilities {
            flags: flags,
            flying_speed: flying_speed,
            walking_speed: walking_speed,
        }
    }
}
impl Sendable for PlayerAbilities {
    fn to_u8(&self) -> io::Result<Vec<u8>> {
        let mut ret = Vec::new();
        write_varint(&PlayerAbilities::get_id(), &mut ret)?;
        write_u8(&self.flags, &mut ret)?;
        write_f32(&self.flying_speed, &mut ret)?;
        write_f32(&self.walking_speed, &mut ret)?;
        Ok(ret)
    }
    fn get_state() -> ClientState {
        ClientState::Play
    }
    fn get_id() -> i32 {
        18
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PlayerDigging {
    status: i32,
    location: u64,
    face: u8,
}
impl PlayerDigging {
    pub fn new(status: i32, location: u64, face: u8) -> Self {
        PlayerDigging {
            status: status,
            location: location,
            face: face,
        }
    }
}
impl Sendable for PlayerDigging {
    fn to_u8(&self) -> io::Result<Vec<u8>> {
        let mut ret = Vec::new();
        write_varint(&PlayerDigging::get_id(), &mut ret)?;
        write_varint(&self.status, &mut ret)?;
        write_u64(&self.location, &mut ret)?;
        write_u8(&self.face, &mut ret)?;
        Ok(ret)
    }
    fn get_state() -> ClientState {
        ClientState::Play
    }
    fn get_id() -> i32 {
        19
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EntityAction {
    entity_id: i32,
    action: i32,
    jump_boost: i32,
}
impl EntityAction {
    pub fn new(entity_id: i32, action: i32, jump_boost: i32) -> Self {
        EntityAction {
            entity_id: entity_id,
            action: action,
            jump_boost: jump_boost,
        }
    }
}
impl Sendable for EntityAction {
    fn to_u8(&self) -> io::Result<Vec<u8>> {
        let mut ret = Vec::new();
        write_varint(&EntityAction::get_id(), &mut ret)?;
        write_varint(&self.entity_id, &mut ret)?;
        write_varint(&self.action, &mut ret)?;
        write_varint(&self.jump_boost, &mut ret)?;
        Ok(ret)
    }
    fn get_state() -> ClientState {
        ClientState::Play
    }
    fn get_id() -> i32 {
        20
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SteerVehicle {
    sideways: f32,
    forward: f32,
    flags: u8,
}
impl SteerVehicle {
    pub fn new(sideways: f32, forward: f32, flags: u8) -> Self {
        SteerVehicle {
            sideways: sideways,
            forward: forward,
            flags: flags,
        }
    }
}
impl Sendable for SteerVehicle {
    fn to_u8(&self) -> io::Result<Vec<u8>> {
        let mut ret = Vec::new();
        write_varint(&SteerVehicle::get_id(), &mut ret)?;
        write_f32(&self.sideways, &mut ret)?;
        write_f32(&self.forward, &mut ret)?;
        write_u8(&self.flags, &mut ret)?;
        Ok(ret)
    }
    fn get_state() -> ClientState {
        ClientState::Play
    }
    fn get_id() -> i32 {
        21
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ResourcePackStatus {
    result: i32,
}
impl ResourcePackStatus {
    pub fn new(result: i32) -> Self {
        ResourcePackStatus {
            result: result,
        }
    }
}
impl Sendable for ResourcePackStatus {
    fn to_u8(&self) -> io::Result<Vec<u8>> {
        let mut ret = Vec::new();
        write_varint(&ResourcePackStatus::get_id(), &mut ret)?;
        write_varint(&self.result, &mut ret)?;
        Ok(ret)
    }
    fn get_state() -> ClientState {
        ClientState::Play
    }
    fn get_id() -> i32 {
        22
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct HeldItemChange {
    slot: i16,
}
impl HeldItemChange {
    pub fn new(slot: i16) -> Self {
        HeldItemChange {
            slot: slot,
        }
    }
}
impl Sendable for HeldItemChange {
    fn to_u8(&self) -> io::Result<Vec<u8>> {
        let mut ret = Vec::new();
        write_varint(&HeldItemChange::get_id(), &mut ret)?;
        write_i16(&self.slot, &mut ret)?;
        Ok(ret)
    }
    fn get_state() -> ClientState {
        ClientState::Play
    }
    fn get_id() -> i32 {
        23
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CreativeInventoryAction {
    slot_id: i16,
    slot: Vec<u8>,
}
impl CreativeInventoryAction {
    pub fn new(slot_id: i16, slot: Vec<u8>) -> Self {
        CreativeInventoryAction {
            slot_id: slot_id,
            slot: slot,
        }
    }
}
impl Sendable for CreativeInventoryAction {
    fn to_u8(&self) -> io::Result<Vec<u8>> {
        let mut ret = Vec::new();
        write_varint(&CreativeInventoryAction::get_id(), &mut ret)?;
        write_i16(&self.slot_id, &mut ret)?;
        write_bytearray(&self.slot, &mut ret)?;
        Ok(ret)
    }
    fn get_state() -> ClientState {
        ClientState::Play
    }
    fn get_id() -> i32 {
        24
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct UpdateSign {
    location: u64,
    line1: String,
    line2: String,
    line3: String,
    line4: String,
}
impl UpdateSign {
    pub fn new(location: u64, line1: String, line2: String, line3: String, line4: String) -> Self {
        UpdateSign {
            location: location,
            line1: line1,
            line2: line2,
            line3: line3,
            line4: line4,
        }
    }
}
impl Sendable for UpdateSign {
    fn to_u8(&self) -> io::Result<Vec<u8>> {
        let mut ret = Vec::new();
        write_varint(&UpdateSign::get_id(), &mut ret)?;
        write_u64(&self.location, &mut ret)?;
        write_String(&self.line1, &mut ret)?;
        write_String(&self.line2, &mut ret)?;
        write_String(&self.line3, &mut ret)?;
        write_String(&self.line4, &mut ret)?;
        Ok(ret)
    }
    fn get_state() -> ClientState {
        ClientState::Play
    }
    fn get_id() -> i32 {
        25
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Animation {
    hand: i32,
}
impl Animation {
    pub fn new(hand: i32) -> Self {
        Animation {
            hand: hand,
        }
    }
}
impl Sendable for Animation {
    fn to_u8(&self) -> io::Result<Vec<u8>> {
        let mut ret = Vec::new();
        write_varint(&Animation::get_id(), &mut ret)?;
        write_varint(&self.hand, &mut ret)?;
        Ok(ret)
    }
    fn get_state() -> ClientState {
        ClientState::Play
    }
    fn get_id() -> i32 {
        26
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Spectate {
    target: u128,
}
impl Spectate {
    pub fn new(target: u128) -> Self {
        Spectate {
            target: target,
        }
    }
}
impl Sendable for Spectate {
    fn to_u8(&self) -> io::Result<Vec<u8>> {
        let mut ret = Vec::new();
        write_varint(&Spectate::get_id(), &mut ret)?;
        write_uuid(&self.target, &mut ret)?;
        Ok(ret)
    }
    fn get_state() -> ClientState {
        ClientState::Play
    }
    fn get_id() -> i32 {
        27
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PlayerBlockPlacement {
    location: u64,
    face: i32,
    hand: i32,
    x: f32,
    y: f32,
    z: f32,
}
impl PlayerBlockPlacement {
    pub fn new(location: u64, face: i32, hand: i32, x: f32, y: f32, z: f32) -> Self {
        PlayerBlockPlacement {
            location: location,
            face: face,
            hand: hand,
            x: x,
            y: y,
            z: z,
        }
    }
}
impl Sendable for PlayerBlockPlacement {
    fn to_u8(&self) -> io::Result<Vec<u8>> {
        let mut ret = Vec::new();
        write_varint(&PlayerBlockPlacement::get_id(), &mut ret)?;
        write_u64(&self.location, &mut ret)?;
        write_varint(&self.face, &mut ret)?;
        write_varint(&self.hand, &mut ret)?;
        write_f32(&self.x, &mut ret)?;
        write_f32(&self.y, &mut ret)?;
        write_f32(&self.z, &mut ret)?;
        Ok(ret)
    }
    fn get_state() -> ClientState {
        ClientState::Play
    }
    fn get_id() -> i32 {
        28
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct UseItem {
    hand: i32,
}
impl UseItem {
    pub fn new(hand: i32) -> Self {
        UseItem {
            hand: hand,
        }
    }
}
impl Sendable for UseItem {
    fn to_u8(&self) -> io::Result<Vec<u8>> {
        let mut ret = Vec::new();
        write_varint(&UseItem::get_id(), &mut ret)?;
        write_varint(&self.hand, &mut ret)?;
        Ok(ret)
    }
    fn get_state() -> ClientState {
        ClientState::Play
    }
    fn get_id() -> i32 {
        29
    }
}

