// Automatically generated rust module for 'p2p_network_kad_message.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use std::borrow::Cow;
use quick_protobuf::{MessageInfo, MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result};
use quick_protobuf::sizeofs::*;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Record<'a> {
    pub key: Cow<'a, [u8]>,
    pub value: Cow<'a, [u8]>,
    pub timeReceived: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Record<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.key = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.value = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.timeReceived = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Record<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.key == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.key).len()) }
        + if self.value == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.value).len()) }
        + if self.timeReceived == "" { 0 } else { 1 + sizeof_len((&self.timeReceived).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.key != Cow::Borrowed(b"") { w.write_with_tag(10, |w| w.write_bytes(&**&self.key))?; }
        if self.value != Cow::Borrowed(b"") { w.write_with_tag(18, |w| w.write_bytes(&**&self.value))?; }
        if self.timeReceived != "" { w.write_with_tag(42, |w| w.write_string(&**&self.timeReceived))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Message<'a> {
    pub type_pb: mod_Message::MessageType,
    pub clusterLevelRaw: i32,
    pub key: Cow<'a, [u8]>,
    pub record: Option<Record<'a>>,
    pub closerPeers: Vec<mod_Message::Peer<'a>>,
    pub providerPeers: Vec<mod_Message::Peer<'a>>,
}

impl<'a> MessageRead<'a> for Message<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.type_pb = r.read_enum(bytes)?,
                Ok(80) => msg.clusterLevelRaw = r.read_int32(bytes)?,
                Ok(18) => msg.key = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.record = Some(r.read_message::<Record>(bytes)?),
                Ok(66) => msg.closerPeers.push(r.read_message::<mod_Message::Peer>(bytes)?),
                Ok(74) => msg.providerPeers.push(r.read_message::<mod_Message::Peer>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Message<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.type_pb == p2p_network_kad_message::mod_Message::MessageType::PUT_VALUE { 0 } else { 1 + sizeof_varint(*(&self.type_pb) as u64) }
        + if self.clusterLevelRaw == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.clusterLevelRaw) as u64) }
        + if self.key == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.key).len()) }
        + self.record.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.closerPeers.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.providerPeers.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.type_pb != p2p_network_kad_message::mod_Message::MessageType::PUT_VALUE { w.write_with_tag(8, |w| w.write_enum(*&self.type_pb as i32))?; }
        if self.clusterLevelRaw != 0i32 { w.write_with_tag(80, |w| w.write_int32(*&self.clusterLevelRaw))?; }
        if self.key != Cow::Borrowed(b"") { w.write_with_tag(18, |w| w.write_bytes(&**&self.key))?; }
        if let Some(ref s) = self.record { w.write_with_tag(26, |w| w.write_message(s))?; }
        for s in &self.closerPeers { w.write_with_tag(66, |w| w.write_message(s))?; }
        for s in &self.providerPeers { w.write_with_tag(74, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_Message {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Peer<'a> {
    pub id: Cow<'a, [u8]>,
    pub addrs: Vec<Cow<'a, [u8]>>,
    pub connection: mod_Message::ConnectionType,
}

impl<'a> MessageRead<'a> for Peer<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.addrs.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(24) => msg.connection = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Peer<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + self.addrs.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + if self.connection == p2p_network_kad_message::mod_Message::ConnectionType::NOT_CONNECTED { 0 } else { 1 + sizeof_varint(*(&self.connection) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != Cow::Borrowed(b"") { w.write_with_tag(10, |w| w.write_bytes(&**&self.id))?; }
        for s in &self.addrs { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        if self.connection != p2p_network_kad_message::mod_Message::ConnectionType::NOT_CONNECTED { w.write_with_tag(24, |w| w.write_enum(*&self.connection as i32))?; }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MessageType {
    PUT_VALUE = 0,
    GET_VALUE = 1,
    ADD_PROVIDER = 2,
    GET_PROVIDERS = 3,
    FIND_NODE = 4,
    PING = 5,
}

impl Default for MessageType {
    fn default() -> Self {
        MessageType::PUT_VALUE
    }
}

impl From<i32> for MessageType {
    fn from(i: i32) -> Self {
        match i {
            0 => MessageType::PUT_VALUE,
            1 => MessageType::GET_VALUE,
            2 => MessageType::ADD_PROVIDER,
            3 => MessageType::GET_PROVIDERS,
            4 => MessageType::FIND_NODE,
            5 => MessageType::PING,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for MessageType {
    fn from(s: &'a str) -> Self {
        match s {
            "PUT_VALUE" => MessageType::PUT_VALUE,
            "GET_VALUE" => MessageType::GET_VALUE,
            "ADD_PROVIDER" => MessageType::ADD_PROVIDER,
            "GET_PROVIDERS" => MessageType::GET_PROVIDERS,
            "FIND_NODE" => MessageType::FIND_NODE,
            "PING" => MessageType::PING,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ConnectionType {
    NOT_CONNECTED = 0,
    CONNECTED = 1,
    CAN_CONNECT = 2,
    CANNOT_CONNECT = 3,
}

impl Default for ConnectionType {
    fn default() -> Self {
        ConnectionType::NOT_CONNECTED
    }
}

impl From<i32> for ConnectionType {
    fn from(i: i32) -> Self {
        match i {
            0 => ConnectionType::NOT_CONNECTED,
            1 => ConnectionType::CONNECTED,
            2 => ConnectionType::CAN_CONNECT,
            3 => ConnectionType::CANNOT_CONNECT,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ConnectionType {
    fn from(s: &'a str) -> Self {
        match s {
            "NOT_CONNECTED" => ConnectionType::NOT_CONNECTED,
            "CONNECTED" => ConnectionType::CONNECTED,
            "CAN_CONNECT" => ConnectionType::CAN_CONNECT,
            "CANNOT_CONNECT" => ConnectionType::CANNOT_CONNECT,
            _ => Self::default(),
        }
    }
}

}

