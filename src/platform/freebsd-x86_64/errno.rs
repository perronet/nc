
// Code generated by mkerrno_freebsd.py; DO NOT EDIT.

pub type Errno = usize;

pub const EPERM: Errno = 1;
pub const ENOENT: Errno = 2;
pub const ESRCH: Errno = 3;
pub const EINTR: Errno = 4;
pub const EIO: Errno = 5;
pub const ENXIO: Errno = 6;
pub const E2BIG: Errno = 7;
pub const ENOEXEC: Errno = 8;
pub const EBADF: Errno = 9;
pub const ECHILD: Errno = 10;
pub const EDEADLK: Errno = 11;
pub const ENOMEM: Errno = 12;
pub const EACCES: Errno = 13;
pub const EFAULT: Errno = 14;
pub const ENOTBLK: Errno = 15;
pub const EBUSY: Errno = 16;
pub const EEXIST: Errno = 17;
pub const EXDEV: Errno = 18;
pub const ENODEV: Errno = 19;
pub const ENOTDIR: Errno = 20;
pub const EISDIR: Errno = 21;
pub const EINVAL: Errno = 22;
pub const ENFILE: Errno = 23;
pub const EMFILE: Errno = 24;
pub const ENOTTY: Errno = 25;
pub const ETXTBSY: Errno = 26;
pub const EFBIG: Errno = 27;
pub const ENOSPC: Errno = 28;
pub const ESPIPE: Errno = 29;
pub const EROFS: Errno = 30;
pub const EMLINK: Errno = 31;
pub const EPIPE: Errno = 32;
pub const EDOM: Errno = 33;
pub const ERANGE: Errno = 34;
pub const EAGAIN: Errno = 35;
pub const EWOULDBLOCK: Errno = EAGAIN;
pub const EINPROGRESS: Errno = 36;
pub const EALREADY: Errno = 37;
pub const ENOTSOCK: Errno = 38;
pub const EDESTADDRREQ: Errno = 39;
pub const EMSGSIZE: Errno = 40;
pub const EPROTOTYPE: Errno = 41;
pub const ENOPROTOOPT: Errno = 42;
pub const EPROTONOSUPPORT: Errno = 43;
pub const ESOCKTNOSUPPORT: Errno = 44;
pub const EOPNOTSUPP: Errno = 45;
pub const ENOTSUP: Errno = EOPNOTSUPP;
pub const EPFNOSUPPORT: Errno = 46;
pub const EAFNOSUPPORT: Errno = 47;
pub const EADDRINUSE: Errno = 48;
pub const EADDRNOTAVAIL: Errno = 49;
pub const ENETDOWN: Errno = 50;
pub const ENETUNREACH: Errno = 51;
pub const ENETRESET: Errno = 52;
pub const ECONNABORTED: Errno = 53;
pub const ECONNRESET: Errno = 54;
pub const ENOBUFS: Errno = 55;
pub const EISCONN: Errno = 56;
pub const ENOTCONN: Errno = 57;
pub const ESHUTDOWN: Errno = 58;
pub const ETOOMANYREFS: Errno = 59;
pub const ETIMEDOUT: Errno = 60;
pub const ECONNREFUSED: Errno = 61;
pub const ELOOP: Errno = 62;
pub const ENAMETOOLONG: Errno = 63;
pub const EHOSTDOWN: Errno = 64;
pub const EHOSTUNREACH: Errno = 65;
pub const ENOTEMPTY: Errno = 66;
pub const EPROCLIM: Errno = 67;
pub const EUSERS: Errno = 68;
pub const EDQUOT: Errno = 69;
pub const ESTALE: Errno = 70;
pub const EREMOTE: Errno = 71;
pub const EBADRPC: Errno = 72;
pub const ERPCMISMATCH: Errno = 73;
pub const EPROGUNAVAIL: Errno = 74;
pub const EPROGMISMATCH: Errno = 75;
pub const EPROCUNAVAIL: Errno = 76;
pub const ENOLCK: Errno = 77;
pub const ENOSYS: Errno = 78;
pub const EFTYPE: Errno = 79;
pub const EAUTH: Errno = 80;
pub const ENEEDAUTH: Errno = 81;
pub const EIDRM: Errno = 82;
pub const ENOMSG: Errno = 83;
pub const EOVERFLOW: Errno = 84;
pub const ECANCELED: Errno = 85;
pub const EILSEQ: Errno = 86;
pub const ENOATTR: Errno = 87;
pub const EDOOFUS: Errno = 88;
pub const EBADMSG: Errno = 89;
pub const EMULTIHOP: Errno = 90;
pub const ENOLINK: Errno = 91;
pub const EPROTO: Errno = 92;
pub const ENOTCAPABLE: Errno = 93;
pub const ECAPMODE: Errno = 94;
pub const ENOTRECOVERABLE: Errno = 95;
pub const EOWNERDEAD: Errno = 96;
pub const ELAST: Errno = 96;
