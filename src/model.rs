//
// model.rs
// Copyright (C) 2021 gmg137 <gmg137 AT live.com>
// snap7-rs is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan PSL v2.
// You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND,
// EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT,
// MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.
//
#![allow(warnings)]
pub const EVC_SERVER_STARTED: u32 = 0x00000001;
pub const EVC_SERVER_STOPPED: u32 = 0x00000002;
pub const EVC_LISTENER_CANNOT_START: u32 = 0x00000004;
pub const EVC_CLIENT_ADDED: u32 = 0x00000008;
pub const EVC_CLIENT_REJECTED: u32 = 0x00000010;
pub const EVC_CLIENT_NO_ROOM: u32 = 0x00000020;
pub const EVC_CLIENT_EXCEPTION: u32 = 0x00000040;
pub const EVC_CLIENT_DISCONNECTED: u32 = 0x00000080;
pub const EVC_CLIENT_TERMINATED: u32 = 0x00000100;
pub const EVC_CLIENTS_DROPPED: u32 = 0x00000200;
pub const EVC_RESERVED_00000400: u32 = 0x00000400;
pub const EVC_RESERVED_00000800: u32 = 0x00000800;
pub const EVC_RESERVED_00001000: u32 = 0x00001000;
pub const EVC_RESERVED_00002000: u32 = 0x00002000;
pub const EVC_RESERVED_00004000: u32 = 0x00004000;
pub const EVC_RESERVED_00008000: u32 = 0x00008000;
pub const EVC_PDU_INCOMING: u32 = 0x00010000;
pub const EVC_DATA_READ: u32 = 0x00020000;
pub const EVC_DATA_WRITE: u32 = 0x00040000;
pub const EVC_NEGOTIATE_PDU: u32 = 0x00080000;
pub const EVC_READ_SZL: u32 = 0x00100000;
pub const EVC_CLOCK: u32 = 0x00200000;
pub const EVC_UPLOAD: u32 = 0x00400000;
pub const EVC_DOWNLOAD: u32 = 0x00800000;
pub const EVC_DIRECTORY: u32 = 0x01000000;
pub const EVC_SECURITY: u32 = 0x02000000;
pub const EVC_CONTROL: u32 = 0x04000000;
pub const EVC_RESERVED_08000000: u32 = 0x08000000;
pub const EVC_RESERVED_10000000: u32 = 0x10000000;
pub const EVC_RESERVED_20000000: u32 = 0x20000000;
pub const EVC_RESERVED_40000000: u32 = 0x40000000;
pub const EVC_RESERVED_80000000: u32 = 0x80000000;

/// ?????????????????????
pub enum ConnType {
    PG,
    OP,
    S7Basic(u16),
}

/// ?????????????????????
pub enum MaskKind {
    /// ??????
    Event = 0,
    /// ??????
    Log = 1,
}

/// ?????????????????????
pub enum AreaCode {
    /// ??????(Inputs)
    S7AreaPE = 0,
    /// ??????(Outputs)
    S7AreaPA = 1,
    /// ???????????????(Merkers)
    S7AreaMK = 2,
    /// ?????????(Counters)
    S7AreaCT = 3,
    /// ?????????(Timers)
    S7AreaTM = 4,
    /// ?????????(DB)
    S7AreaDB = 5,
}

/// Snap 7 ????????????
#[derive(Debug)]
pub enum InternalParam {
    /// Socket ????????????
    LocalPort = 1,
    /// Socket ????????????
    RemotePort,
    /// Client Ping ??????
    PingTimeout,
    /// Socket ????????????
    SendTimeout,
    /// Socket ????????????
    RecvTimeout,
    /// Socket ????????????
    WorkInterval,
    /// ISOTcp Source reference
    SrcRef,
    /// ISOTcp Destination reference
    DstRef,
    /// ISOTcp Source TSAP
    SrcTSap,
    /// ?????? PDU ????????????
    PDURequest,
    /// ???????????????????????????
    MaxClients,
    /// BSend ????????????
    BSendTimeout,
    /// BSend ????????????
    BRecvTimeout,
    /// ??????????????????
    RecoveryTime,
    /// (PLC)????????????????????????
    KeepAliveTime,
}

/// Snap7 ???????????????
#[derive(Debug)]
pub enum InternalParamValue {
    U16(u16),
    I32(i32),
    U32(u32),
}

/// Area ???
#[derive(Debug)]
pub enum AreaTable {
    /// ??????(Inputs)
    S7AreaPE = 0x81,
    /// ??????(Outputs)
    S7AreaPA = 0x82,
    /// ???????????????(Merkers)
    S7AreaMK = 0x83,
    /// ?????????(DB)
    S7AreaDB = 0x84,
    /// ?????????(Counters)
    S7AreaCT = 0x1c,
    /// ?????????(Timers)
    S7AreaTM = 0x1d,
}

/// WordLen ???
#[derive(Debug)]
pub enum WordLenTable {
    /// ???????????? 1
    S7WLBit = 0x01,
    /// ???????????? 1
    S7WLByte = 0x02,
    /// ???????????? 2
    S7WLWord = 0x04,
    /// ???????????? 4
    S7WLDWord = 0x06,
    /// ???????????? 4
    S7WLReal = 0x08,
    /// ???????????? 2
    S7WLCounter = 0x1c,
    /// ???????????? 2
    S7WLTimer = 0x1d,
}

/// ????????????
#[derive(Debug)]
pub enum BlockType {
    BlockOB = 0x38,
    BlockDB = 0x41,
    BlockSDB = 0x42,
    BlockFC = 0x43,
    BlockSFC = 0x44,
    BlockFB = 0x45,
    BlockSFB = 0x46,
}
