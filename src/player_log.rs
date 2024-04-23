use std::io::Cursor;
use std::net::Ipv4Addr;

use anyhow::Result;
use anyhow::{bail, Context};
use bitflags::bitflags;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use phf::phf_map;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub fn serialize_vec(logs: &Vec<PlayerLog>) -> Vec<u8> {
    let mut serialized = vec![];
    serialized
        .write_u64::<BigEndian>(logs.len() as u64)
        .unwrap();

    for log in logs {
        log.serialize(&mut serialized).unwrap();
    }

    serialized
}

pub fn deserialize_vec(serialized: &[u8]) -> Vec<PlayerLog> {
    let mut cursor = Cursor::new(serialized);
    let len = cursor.read_u64::<BigEndian>().unwrap() as usize;

    let mut logs = Vec::with_capacity(len);
    for _ in 0..len {
        logs.push(PlayerLog::deserialize(&mut cursor).unwrap());
    }

    logs
}

pub static VERSIONS: phf::Map<&'static str, u8> = phf_map! {
    "1.8" => 1,
    "1.9" => 2,
    "1.10" => 3,
    "1.11" => 4,
    "1.12" => 5,
    "1.13" => 6,
    "1.14" => 7,
    "1.15" => 8,
    "1.16" => 9,
    "1.17" => 10,
    "1.18" => 11,
    "1.19" => 12,
    "1.20" => 13,
    "1.21" => 14,
};

bitflags! {
    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
    #[serde(transparent)]
    pub struct LogFlags: u8 {
        const PLAYER_AUTH = 1;
        const IS_ONLINE = 1 << 1; // (has uuid)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct PlayerLogBuilder {
    pub flags: LogFlags,
    pub player_uuid: Option<Uuid>, // 128 bits (16 bytes)
    pub player_name: String,       // max 16 bytes
    pub player_ip: Ipv4Addr,
    pub server_ip: Ipv4Addr,
    pub server_port: u16, // max 16 bits (1-65535)
    pub server_domain: String,
    pub server_version: String,
}

impl PlayerLogBuilder {
    pub fn build(&self) -> Result<PlayerLog> {
        if self.player_name.len() > 16 {
            bail!("Player name too long");
        }

        let player_uuid = self.player_uuid.map(|uuid| {
            let mut uuid_array = [0; 16];
            uuid_array.copy_from_slice(uuid.as_bytes());
            uuid_array
        });

        let player_name_bytes = self.player_name.as_bytes().to_vec();

        let player_ip = self.player_ip.octets();
        let server_ip = self.server_ip.octets();

        let mut server_domain_bytes = self.server_domain.as_bytes().to_vec();
        server_domain_bytes.truncate(255);

        let server_version = *VERSIONS
            .get(&self.server_version)
            .context("invalid server version")?;

        Ok(PlayerLog {
            binary_version: 1,
            flags: self.flags.bits(),
            player_uuid,
            player_name: player_name_bytes,
            player_ip,
            server_ip,
            server_port: self.server_port,
            server_domain: server_domain_bytes,
            server_version,
        })
    }

    pub fn from_log(log: &PlayerLog) -> Result<PlayerLogBuilder> {
        let flags = LogFlags::from_bits(log.flags).context("invalid flags")?;

        let player_uuid = log.player_uuid.map(Uuid::from_bytes);

        let player_name =
            String::from_utf8(log.player_name.clone()).context("invalid player name")?;

        let player_ip = Ipv4Addr::from(log.player_ip);
        let server_ip = Ipv4Addr::from(log.server_ip);

        let server_domain =
            String::from_utf8(log.server_domain.clone()).context("invalid server domain")?;

        let server_version = VERSIONS
            .entries()
            .find(|(_, n)| **n == log.server_version)
            .context("invalid server version")?
            .0
            .to_string();

        Ok(PlayerLogBuilder {
            flags,
            player_uuid,
            player_name,
            player_ip: Ipv4Addr::from(player_ip),
            server_ip: Ipv4Addr::from(server_ip),
            server_port: log.server_port,
            server_domain,
            server_version,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct PlayerLog {
    pub binary_version: u8,
    pub flags: u8,
    pub player_uuid: Option<[u8; 16]>, // 128 bits (16 bytes)
    pub player_name: Vec<u8>,          // max 16 bytes
    pub player_ip: [u8; 4],
    pub server_ip: [u8; 4],
    pub server_port: u16, // max 16 bits (1-65535)
    pub server_domain: Vec<u8>,
    pub server_version: u8,
}

impl PlayerLog {
    pub fn serialize<W: WriteBytesExt>(&self, writer: &mut W) -> Result<()> {
        writer.write_u8(self.binary_version)?;
        writer.write_u8(self.flags)?;

        if LogFlags::from_bits_retain(self.flags).contains(LogFlags::IS_ONLINE) {
            let uuid = self.player_uuid.as_ref().context("missing player uuid")?;
            writer.write_all(uuid)?;
        }

        writer.write_u8(self.player_name.len() as u8)?;
        writer.write_all(&self.player_name)?;

        writer.write_all(&self.player_ip)?;
        writer.write_all(&self.server_ip)?;
        writer.write_u16::<BigEndian>(self.server_port)?;

        writer.write_u8(self.server_domain.len() as u8)?;
        writer.write_all(&self.server_domain)?;

        writer.write_u8(self.server_version)?;

        Ok(())
    }

    pub fn deserialize<R: ReadBytesExt>(reader: &mut R) -> Result<Self> {
        let binary_version = reader.read_u8()?;
        if binary_version != 1 {
            bail!("invalid binary version");
        }

        let flags = reader.read_u8()?;
        let parsed_flags = LogFlags::from_bits(flags).context("invalid flags")?;

        let player_uuid = if parsed_flags.contains(LogFlags::IS_ONLINE) {
            let mut uuid = [0; 16];
            reader.read_exact(&mut uuid)?;
            Some(uuid)
        } else {
            None
        };

        let name_len = reader.read_u8()?;
        let mut player_name = vec![0; name_len as usize];
        reader.read_exact(&mut player_name)?;

        let mut player_ip = [0; 4];
        reader.read_exact(&mut player_ip)?;

        let mut server_ip = [0; 4];
        reader.read_exact(&mut server_ip)?;

        let server_port = reader.read_u16::<BigEndian>()?;

        let domain_len = reader.read_u8()?;
        let mut server_domain = vec![0; domain_len as usize];
        reader.read_exact(&mut server_domain)?;

        let server_version = reader.read_u8()?;

        Ok(Self {
            binary_version,
            flags,
            player_uuid,
            player_name,
            player_ip,
            server_ip,
            server_port,
            server_domain,
            server_version,
        })
    }
}
