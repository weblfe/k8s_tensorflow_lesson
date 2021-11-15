s! {
    pub struct termios2 {
        pub c_iflag: ::tcflag_t,
        pub c_oflag: ::tcflag_t,
        pub c_cflag: ::tcflag_t,
        pub c_lflag: ::tcflag_t,
        pub c_line: ::cc_t,
        pub c_cc: [::cc_t; 23],
        pub c_ispeed: ::speed_t,
        pub c_ospeed: ::speed_t,
    }
}

// arch/mips/include/uapi/asm/socket.h
pub const SOL_SOCKET: ::c_int = 0xffff;

// Defined in unix/linux_like/mod.rs
// pub const SO_DEBUG: ::c_int = 0x0001;
pub const SO_REUSEADDR: ::c_int = 0x0004;
pub const SO_KEEPALIVE: ::c_int = 0x0008;
pub const SO_DONTROUTE: ::c_int = 0x0010;
pub const SO_BROADCAST: ::c_int = 0x0020;
pub const SO_LINGER: ::c_int = 0x0080;
pub const SO_OOBINLINE: ::c_int = 0x0100;
pub const SO_REUSEPORT: ::c_int = 0x0200;
pub const SO_TYPE: ::c_int = 0x1008;
// pub const SO_STYLE: ::c_int = SO_TYPE;
pub const SO_ERROR: ::c_int = 0x1007;
pub const SO_SNDBUF: ::c_int = 0x1001;
pub const SO_RCVBUF: ::c_int = 0x1002;
pub const SO_SNDLOWAT: ::c_int = 0x1003;
pub const SO_RCVLOWAT: ::c_int = 0x1004;
// NOTE: These definitions are now being renamed with _OLD postfix,
// but CI haven't support them yet.
// Some related consts could be found in b32.rs and b64.rs
pub const SO_SNDTIMEO: ::c_int = 0x1005;
pub const SO_RCVTIMEO: ::c_int = 0x1006;
// pub const SO_SNDTIMEO_OLD: ::c_int = 0x1005;
// pub const SO_RCVTIMEO_OLD: ::c_int = 0x1006;
pub const SO_ACCEPTCONN: ::c_int = 0x1009;
pub const SO_PROTOCOL: ::c_int = 0x1028;
pub const SO_DOMAIN: ::c_int = 0x1029;

pub const SO_NO_CHECK: ::c_int = 11;
pub const SO_PRIORITY: ::c_int = 12;
pub const SO_BSDCOMPAT: ::c_int = 14;
pub const SO_PASSCRED: ::c_int = 17;
pub const SO_PEERCRED: ::c_int = 18;
pub const SO_SECURITY_AUTHENTICATION: ::c_int = 22;
pub const SO_SECURITY_ENCRYPTION_TRANSPORT: ::c_int = 23;
pub const SO_SECURITY_ENCRYPTION_NETWORK: ::c_int = 24;
pub const SO_BINDTODEVICE: ::c_int = 25;
pub const SO_ATTACH_FILTER: ::c_int = 26;
pub const SO_DETACH_FILTER: ::c_int = 27;
pub const SO_GET_FILTER: ::c_int = SO_ATTACH_FILTER;
pub const SO_PEERNAME: ::c_int = 28;
pub const SO_PEERSEC: ::c_int = 30;
pub const SO_SNDBUFFORCE: ::c_int = 31;
pub const SO_RCVBUFFORCE: ::c_int = 33;
pub const SO_PASSSEC: ::c_int = 34;
pub const SO_MARK: ::c_int = 36;
pub const SO_RXQ_OVFL: ::c_int = 40;
pub const SO_WIFI_STATUS: ::c_int = 41;
pub const SCM_WIFI_STATUS: ::c_int = SO_WIFI_STATUS;
pub const SO_PEEK_OFF: ::c_int = 42;
pub const SO_NOFCS: ::c_int = 43;
pub const SO_LOCK_FILTER: ::c_int = 44;
pub const SO_SELECT_ERR_QUEUE: ::c_int = 45;
pub const SO_BUSY_POLL: ::c_int = 46;
pub const SO_MAX_PACING_RATE: ::c_int = 47;
pub const SO_BPF_EXTENSIONS: ::c_int = 48;
pub const SO_INCOMING_CPU: ::c_int = 49;
pub const SO_ATTACH_BPF: ::c_int = 50;
pub const SO_DETACH_BPF: ::c_int = SO_DETACH_FILTER;
pub const SO_ATTACH_REUSEPORT_CBPF: ::c_int = 51;
pub const SO_ATTACH_REUSEPORT_EBPF: ::c_int = 52;
pub const SO_CNX_ADVICE: ::c_int = 53;
pub const SCM_TIMESTAMPING_OPT_STATS: ::c_int = 54;
pub const SO_MEMINFO: ::c_int = 55;
pub const SO_INCOMING_NAPI_ID: ::c_int = 56;
pub const SO_COOKIE: ::c_int = 57;
pub const SCM_TIMESTAMPING_PKTINFO: ::c_int = 58;
pub const SO_PEERGROUPS: ::c_int = 59;
pub const SO_ZEROCOPY: ::c_int = 60;
pub const SO_TXTIME: ::c_int = 61;
pub const SCM_TXTIME: ::c_int = SO_TXTIME;
pub const SO_BINDTOIFINDEX: ::c_int = 62;
// NOTE: These definitions are now being renamed with _OLD postfix,
// but CI haven't support them yet.
// Some related consts could be found in b32.rs and b64.rs
pub const SO_TIMESTAMP: ::c_int = 29;
pub const SO_TIMESTAMPNS: ::c_int = 35;
pub const SO_TIMESTAMPING: ::c_int = 37;
// pub const SO_TIMESTAMP_OLD: ::c_int = 29;
// pub const SO_TIMESTAMPNS_OLD: ::c_int = 35;
// pub const SO_TIMESTAMPING_OLD: ::c_int = 37;
// pub const SO_TIMESTAMP_NEW: ::c_int = 63;
// pub const SO_TIMESTAMPNS_NEW: ::c_int = 64;
// pub const SO_TIMESTAMPING_NEW: ::c_int = 65;
// pub const SO_RCVTIMEO_NEW: ::c_int = 66;
// pub const SO_SNDTIMEO_NEW: ::c_int = 67;
// pub const SO_DETACH_REUSEPORT_BPF: ::c_int = 68;
// pub const SO_PREFER_BUSY_POLL: ::c_int = 69;
// pub const SO_BUSY_POLL_BUDGET: ::c_int = 70;

// Defined in unix/linux_like/mod.rs
// pub const SCM_TIMESTAMP: ::c_int = SO_TIMESTAMP;
pub const SCM_TIMESTAMPNS: ::c_int = SO_TIMESTAMPNS;
pub const SCM_TIMESTAMPING: ::c_int = SO_TIMESTAMPING;

pub const TCGETS2: ::c_ulong = 0x4030542a;
pub const TCSETS2: ::c_ulong = 0x8030542b;
pub const TCSETSW2: ::c_ulong = 0x8030542c;
pub const TCSETSF2: ::c_ulong = 0x8030542d;
