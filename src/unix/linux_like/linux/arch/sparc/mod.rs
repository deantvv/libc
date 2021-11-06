s! {
    pub struct termios2 {
        pub c_iflag: ::tcflag_t,
        pub c_oflag: ::tcflag_t,
        pub c_cflag: ::tcflag_t,
        pub c_lflag: ::tcflag_t,
        pub c_line: ::cc_t,
        pub c_cc: [::cc_t; 19],
        pub c_ispeed: ::speed_t,
        pub c_ospeed: ::speed_t,
    }
}

// arch/sparc/include/uapi/asm/socket.h
pub const SOL_SOCKET: ::c_int = 0xffff;

// Defined in unix/linux_like/mod.rs
// pub const SO_DEBUG: ::c_int = 0x0001;
pub const SO_PASSCRED: ::c_int = 0x0002;
pub const SO_REUSEADDR: ::c_int = 0x0004;
pub const SO_KEEPALIVE: ::c_int = 0x0008;
pub const SO_DONTROUTE: ::c_int = 0x0010;
pub const SO_BROADCAST: ::c_int = 0x0020;
pub const SO_PEERCRED: ::c_int = 0x0040;
pub const SO_LINGER: ::c_int = 0x0080;
pub const SO_OOBINLINE: ::c_int = 0x0100;
pub const SO_REUSEPORT: ::c_int = 0x0200;
pub const SO_BSDCOMPAT: ::c_int = 0x0400;
pub const SO_RCVLOWAT: ::c_int = 0x0800;
pub const SO_SNDLOWAT: ::c_int = 0x1000;
pub const SO_RCVTIMEO: ::c_int = 0x2000;
pub const SO_SNDTIMEO: ::c_int = 0x4000;
// pub const SO_RCVTIMEO_OLD: ::c_int = 0x2000;
// pub const SO_SNDTIMEO_OLD: ::c_int = 0x4000;
pub const SO_ACCEPTCONN: ::c_int = 0x8000;
pub const SO_SNDBUF: ::c_int = 0x1001;
pub const SO_RCVBUF: ::c_int = 0x1002;
pub const SO_SNDBUFFORCE: ::c_int = 0x100a;
pub const SO_RCVBUFFORCE: ::c_int = 0x100b;
pub const SO_ERROR: ::c_int = 0x1007;
pub const SO_TYPE: ::c_int = 0x1008;
pub const SO_PROTOCOL: ::c_int = 0x1028;
pub const SO_DOMAIN: ::c_int = 0x1029;
pub const SO_NO_CHECK: ::c_int = 0x000b;
pub const SO_PRIORITY: ::c_int = 0x000c;
pub const SO_BINDTODEVICE: ::c_int = 0x000d;
pub const SO_ATTACH_FILTER: ::c_int = 0x001a;
pub const SO_DETACH_FILTER: ::c_int = 0x001b;
pub const SO_GET_FILTER: ::c_int = SO_ATTACH_FILTER;
pub const SO_PEERNAME: ::c_int = 0x001c;
pub const SO_PEERSEC: ::c_int = 0x001e;
pub const SO_PASSSEC: ::c_int = 0x001f;
pub const SO_MARK: ::c_int = 0x0022;
pub const SO_RXQ_OVFL: ::c_int = 0x0024;
pub const SO_WIFI_STATUS: ::c_int = 0x0025;
pub const SCM_WIFI_STATUS: ::c_int = SO_WIFI_STATUS;
pub const SO_PEEK_OFF: ::c_int = 0x0026;
pub const SO_NOFCS: ::c_int = 0x0027;
pub const SO_LOCK_FILTER: ::c_int = 0x0028;
pub const SO_SELECT_ERR_QUEUE: ::c_int = 0x0029;
pub const SO_BUSY_POLL: ::c_int = 0x0030;
pub const SO_MAX_PACING_RATE: ::c_int = 0x0031;
pub const SO_BPF_EXTENSIONS: ::c_int = 0x0032;
pub const SO_INCOMING_CPU: ::c_int = 0x0033;
pub const SO_ATTACH_BPF: ::c_int = 0x0034;
pub const SO_DETACH_BPF: ::c_int = SO_DETACH_FILTER;
pub const SO_ATTACH_REUSEPORT_CBPF: ::c_int = 0x0035;
pub const SO_ATTACH_REUSEPORT_EBPF: ::c_int = 0x0036;
pub const SO_CNX_ADVICE: ::c_int = 0x0037;
pub const SCM_TIMESTAMPING_OPT_STATS: ::c_int = 0x0038;
pub const SO_MEMINFO: ::c_int = 0x0039;
pub const SO_INCOMING_NAPI_ID: ::c_int = 0x003a;
pub const SO_COOKIE: ::c_int = 0x003b;
pub const SCM_TIMESTAMPING_PKTINFO: ::c_int = 0x003c;
pub const SO_PEERGROUPS: ::c_int = 0x003d;
pub const SO_ZEROCOPY: ::c_int = 0x003e;
pub const SO_TXTIME: ::c_int = 0x003f;
pub const SCM_TXTIME: ::c_int = SO_TXTIME;
pub const SO_BINDTOIFINDEX: ::c_int = 0x0041;
pub const SO_SECURITY_AUTHENTICATION: ::c_int = 0x5001;
pub const SO_SECURITY_ENCRYPTION_TRANSPORT: ::c_int = 0x5002;
pub const SO_SECURITY_ENCRYPTION_NETWORK: ::c_int = 0x5004;
pub const SO_TIMESTAMP: ::c_int = 0x001d;
pub const SO_TIMESTAMPNS: ::c_int = 0x0021;
pub const SO_TIMESTAMPING: ::c_int = 0x0023;
// pub const SO_TIMESTAMP_OLD: ::c_int = 0x001d;
// pub const SO_TIMESTAMPNS_OLD: ::c_int = 0x0021;
// pub const SO_TIMESTAMPING_OLD: ::c_int = 0x0023;
// pub const SO_TIMESTAMP_NEW: ::c_int = 0x0046;
// pub const SO_TIMESTAMPNS_NEW: ::c_int = 0x0042;
// pub const SO_TIMESTAMPING_NEW: ::c_int = 0x0043;
// pub const SO_RCVTIMEO_NEW: ::c_int = 0x0044;
// pub const SO_SNDTIMEO_NEW: ::c_int = 0x0045;
// pub const SO_DETACH_REUSEPORT_BPF: ::c_int = 0x0047;
// pub const SO_PREFER_BUSY_POLL: ::c_int = 0x0048;
// pub const SO_BUSY_POLL_BUDGET: ::c_int = 0x0049;

// Defined in unix/linux_like/mod.rs
// pub const SCM_TIMESTAMP: ::c_int = SO_TIMESTAMP;
pub const SCM_TIMESTAMPNS: ::c_int = SO_TIMESTAMPNS;
pub const SCM_TIMESTAMPING: ::c_int = SO_TIMESTAMPING;

pub const TCGETS2: ::c_ulong = 0x402c540c;
pub const TCSETS2: ::c_ulong = 0x802c540d;
pub const TCSETSW2: ::c_ulong = 0x802c540e;
pub const TCSETSF2: ::c_ulong = 0x802c540f;
