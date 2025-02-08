use crate::packets::configuration::data::known_pack::KnownPack;
use protocol::prelude::*;

#[derive(Debug, PacketOut)]
#[packet_id("configuration/clientbound/minecraft:select_known_packs")]
pub struct ClientBoundKnownPacksPacket {
    pub known_packs: LengthPaddedVec<KnownPack>,
}

impl Default for ClientBoundKnownPacksPacket {
    fn default() -> Self {
        Self {
            known_packs: vec![KnownPack::default()].into(),
        }
    }
}
