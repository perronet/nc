// Code generated by mksysnum_freebsd.py; DO NOT EDIT.

pub type Sysno = usize;

pub const SYS_EXIT: Sysno = 1; // { void|sys||exit(int rval); }
pub const SYS_FORK: Sysno = 2; // { int|sys||fork(void); }
pub const SYS_READ: Sysno = 3; // { ssize_t|sys||read(int fd, void *buf, size_t nbyte); }
pub const SYS_WRITE: Sysno = 4; // { ssize_t|sys||write(int fd, const void *buf, size_t nbyte); }
pub const SYS_OPEN: Sysno = 5; // { int|sys||open(const char *path, int flags, ... mode_t mode); }
pub const SYS_CLOSE: Sysno = 6; // { int|sys||close(int fd); }
pub const SYS_LINK: Sysno = 9; // { int|sys||link(const char *path, const char *link); }
pub const SYS_UNLINK: Sysno = 10; // { int|sys||unlink(const char *path); }
pub const SYS_CHDIR: Sysno = 12; // { int|sys||chdir(const char *path); }
pub const SYS_FCHDIR: Sysno = 13; // { int|sys||fchdir(int fd); }
pub const SYS_CHMOD: Sysno = 15; // { int|sys||chmod(const char *path, mode_t mode); }
pub const SYS_CHOWN: Sysno = 16; // { int|sys||chown(const char *path, uid_t uid, gid_t gid); }
pub const SYS_OBREAK: Sysno = 17; // { int|sys||obreak(char *nsize); }
pub const SYS_GETPID_WITH_PPID: Sysno = 20; // { pid_t|sys||getpid_with_ppid(void); }
pub const SYS_UNMOUNT: Sysno = 22; // { int|sys||unmount(const char *path, int flags); }
pub const SYS_SETUID: Sysno = 23; // { int|sys||setuid(uid_t uid); }
pub const SYS_GETUID_WITH_EUID: Sysno = 24; // { uid_t|sys||getuid_with_euid(void); }
pub const SYS_GETEUID: Sysno = 25; // { uid_t|sys||geteuid(void); }
pub const SYS_RECVMSG: Sysno = 27; // { ssize_t|sys||recvmsg(int s, struct msghdr *msg, int flags); }
pub const SYS_SENDMSG: Sysno = 28; // { ssize_t|sys||sendmsg(int s, const struct msghdr *msg, int flags); }
pub const SYS_RECVFROM: Sysno = 29; // { ssize_t|sys||recvfrom(int s, void *buf, size_t len, int flags, struct sockaddr *from, socklen_t *fromlenaddr); }
pub const SYS_ACCEPT: Sysno = 30; // { int|sys||accept(int s, struct sockaddr *name, socklen_t *anamelen); }
pub const SYS_GETPEERNAME: Sysno = 31; // { int|sys||getpeername(int fdes, struct sockaddr *asa, socklen_t *alen); }
pub const SYS_GETSOCKNAME: Sysno = 32; // { int|sys||getsockname(int fdes, struct sockaddr *asa, socklen_t *alen); }
pub const SYS_ACCESS: Sysno = 33; // { int|sys||access(const char *path, int flags); }
pub const SYS_CHFLAGS: Sysno = 34; // { int|sys||chflags(const char *path, u_long flags); }
pub const SYS_FCHFLAGS: Sysno = 35; // { int|sys||fchflags(int fd, u_long flags); }
pub const SYS_SYNC: Sysno = 36; // { void|sys||sync(void); }
pub const SYS_KILL: Sysno = 37; // { int|sys||kill(pid_t pid, int signum); }
pub const SYS_GETPPID: Sysno = 39; // { pid_t|sys||getppid(void); }
pub const SYS_DUP: Sysno = 41; // { int|sys||dup(int fd); }
pub const SYS_PIPE: Sysno = 42; // { int|sys||pipe(void); }
pub const SYS_GETEGID: Sysno = 43; // { gid_t|sys||getegid(void); }
pub const SYS_PROFIL: Sysno = 44; // { int|sys||profil(char *samples, size_t size, u_long offset, u_int scale); }
pub const SYS_KTRACE: Sysno = 45; // { int|sys||ktrace(const char *fname, int ops, int facs, pid_t pid); }
pub const SYS_GETGID_WITH_EGID: Sysno = 47; // { gid_t|sys||getgid_with_egid(void); }
pub const SYS___GETLOGIN: Sysno = 49; // { int|sys||__getlogin(char *namebuf, size_t namelen); }
pub const SYS___SETLOGIN: Sysno = 50; // { int|sys||__setlogin(const char *namebuf); }
pub const SYS_ACCT: Sysno = 51; // { int|sys||acct(const char *path); }
pub const SYS_IOCTL: Sysno = 54; // { int|sys||ioctl(int fd, u_long com, ... void *data); }
pub const SYS_REVOKE: Sysno = 56; // { int|sys||revoke(const char *path); }
pub const SYS_SYMLINK: Sysno = 57; // { int|sys||symlink(const char *path, const char *link); }
pub const SYS_READLINK: Sysno = 58; // { ssize_t|sys||readlink(const char *path, char *buf, size_t count); }
pub const SYS_EXECVE: Sysno = 59; // { int|sys||execve(const char *path, char * const *argp, char * const *envp); }
pub const SYS_UMASK: Sysno = 60; // { mode_t|sys||umask(mode_t newmask); }
pub const SYS_CHROOT: Sysno = 61; // { int|sys||chroot(const char *path); }
pub const SYS_VFORK: Sysno = 66; // { int|sys||vfork(void); }
pub const SYS_OVADVISE: Sysno = 72; // { int|sys||ovadvise(int anom); }
pub const SYS_MUNMAP: Sysno = 73; // { int|sys||munmap(void *addr, size_t len); }
pub const SYS_MPROTECT: Sysno = 74; // { int|sys||mprotect(void *addr, size_t len, int prot); }
pub const SYS_MADVISE: Sysno = 75; // { int|sys||madvise(void *addr, size_t len, int behav); }
pub const SYS_MINCORE: Sysno = 78; // { int|sys||mincore(void *addr, size_t len, char *vec); }
pub const SYS_GETGROUPS: Sysno = 79; // { int|sys||getgroups(int gidsetsize, gid_t *gidset); }
pub const SYS_SETGROUPS: Sysno = 80; // { int|sys||setgroups(int gidsetsize, const gid_t *gidset); }
pub const SYS_GETPGRP: Sysno = 81; // { int|sys||getpgrp(void); }
pub const SYS_SETPGID: Sysno = 82; // { int|sys||setpgid(pid_t pid, pid_t pgid); }
pub const SYS_DUP2: Sysno = 90; // { int|sys||dup2(int from, int to); }
pub const SYS_FCNTL: Sysno = 92; // { int|sys||fcntl(int fd, int cmd, ... void *arg); }
pub const SYS_FSYNC: Sysno = 95; // { int|sys||fsync(int fd); }
pub const SYS_SETPRIORITY: Sysno = 96; // { int|sys||setpriority(int which, id_t who, int prio); }
pub const SYS_CONNECT: Sysno = 98; // { int|sys||connect(int s, const struct sockaddr *name, socklen_t namelen); }
pub const SYS_GETPRIORITY: Sysno = 100; // { int|sys||getpriority(int which, id_t who); }
pub const SYS_BIND: Sysno = 104; // { int|sys||bind(int s, const struct sockaddr *name, socklen_t namelen); }
pub const SYS_SETSOCKOPT: Sysno = 105; // { int|sys||setsockopt(int s, int level, int name, const void *val, socklen_t valsize); }
pub const SYS_LISTEN: Sysno = 106; // { int|sys||listen(int s, int backlog); }
pub const SYS_GETSOCKOPT: Sysno = 118; // { int|sys||getsockopt(int s, int level, int name, void *val, socklen_t *avalsize); }
pub const SYS_READV: Sysno = 120; // { ssize_t|sys||readv(int fd, const struct iovec *iovp, int iovcnt); }
pub const SYS_WRITEV: Sysno = 121; // { ssize_t|sys||writev(int fd, const struct iovec *iovp, int iovcnt); }
pub const SYS_FCHOWN: Sysno = 123; // { int|sys||fchown(int fd, uid_t uid, gid_t gid); }
pub const SYS_FCHMOD: Sysno = 124; // { int|sys||fchmod(int fd, mode_t mode); }
pub const SYS_SETREUID: Sysno = 126; // { int|sys||setreuid(uid_t ruid, uid_t euid); }
pub const SYS_SETREGID: Sysno = 127; // { int|sys||setregid(gid_t rgid, gid_t egid); }
pub const SYS_RENAME: Sysno = 128; // { int|sys||rename(const char *from, const char *to); }
pub const SYS_FLOCK: Sysno = 131; // { int|sys||flock(int fd, int how); }
pub const SYS_MKFIFO: Sysno = 132; // { int|sys||mkfifo(const char *path, mode_t mode); }
pub const SYS_SENDTO: Sysno = 133; // { ssize_t|sys||sendto(int s, const void *buf, size_t len, int flags, const struct sockaddr *to, socklen_t tolen); }
pub const SYS_SHUTDOWN: Sysno = 134; // { int|sys||shutdown(int s, int how); }
pub const SYS_SOCKETPAIR: Sysno = 135; // { int|sys||socketpair(int domain, int type, int protocol, int *rsv); }
pub const SYS_MKDIR: Sysno = 136; // { int|sys||mkdir(const char *path, mode_t mode); }
pub const SYS_RMDIR: Sysno = 137; // { int|sys||rmdir(const char *path); }
pub const SYS_SETSID: Sysno = 147; // { int|sys||setsid(void); }
pub const SYS_SYSARCH: Sysno = 165; // { int|sys||sysarch(int op, void *parms); }
pub const SYS_PREAD: Sysno = 173; // { ssize_t|sys||pread(int fd, void *buf, size_t nbyte, int PAD, off_t offset); }
pub const SYS_PWRITE: Sysno = 174; // { ssize_t|sys||pwrite(int fd, const void *buf, size_t nbyte, int PAD, off_t offset); }
pub const SYS_NTP_ADJTIME: Sysno = 176; // { int|sys||ntp_adjtime(struct timex *tp); }
pub const SYS_SETGID: Sysno = 181; // { int|sys||setgid(gid_t gid); }
pub const SYS_SETEGID: Sysno = 182; // { int|sys||setegid(gid_t egid); }
pub const SYS_SETEUID: Sysno = 183; // { int|sys||seteuid(uid_t euid); }
pub const SYS_PATHCONF: Sysno = 191; // { long|sys||pathconf(const char *path, int name); }
pub const SYS_FPATHCONF: Sysno = 192; // { long|sys||fpathconf(int fd, int name); }
pub const SYS_GETSOCKOPT2: Sysno = 193; // { int|sys||getsockopt2(int s, int level, int name, void *val, socklen_t *avalsize); }
pub const SYS_GETRLIMIT: Sysno = 194; // { int|sys||getrlimit(int which, struct rlimit *rlp); }
pub const SYS_SETRLIMIT: Sysno = 195; // { int|sys||setrlimit(int which, const struct rlimit *rlp); }
pub const SYS_MMAP: Sysno = 197; // { void *|sys||mmap(void *addr, size_t len, int prot, int flags, int fd, long PAD, off_t pos); }
pub const SYS_LSEEK: Sysno = 199; // { off_t|sys||lseek(int fd, int PAD, off_t offset, int whence); }
pub const SYS_TRUNCATE: Sysno = 200; // { int|sys||truncate(const char *path, int PAD, off_t length); }
pub const SYS_FTRUNCATE: Sysno = 201; // { int|sys||ftruncate(int fd, int PAD, off_t length); }
pub const SYS___SYSCTL: Sysno = 202; // { int|sys||__sysctl(const int *name, u_int namelen, void *oldv, size_t *oldlenp, const void *newv, size_t newlen); }
pub const SYS_MLOCK: Sysno = 203; // { int|sys||mlock(const void *addr, size_t len); }
pub const SYS_MUNLOCK: Sysno = 204; // { int|sys||munlock(const void *addr, size_t len); }
pub const SYS_UNDELETE: Sysno = 205; // { int|sys||undelete(const char *path); }
pub const SYS_GETPGID: Sysno = 207; // { pid_t|sys||getpgid(pid_t pid); }
pub const SYS_REBOOT: Sysno = 208; // { int|sys||reboot(int opt, char *bootstr); }
pub const SYS_POLL: Sysno = 209; // { int|sys||poll(struct pollfd *fds, u_int nfds, int timeout); }
pub const SYS_TIMER_CREATE: Sysno = 235; // { int|sys||timer_create(clockid_t clock_id, struct sigevent *evp, timer_t *timerid); }
pub const SYS_TIMER_DELETE: Sysno = 236; // { int|sys||timer_delete(timer_t timerid); }
pub const SYS_TIMER_GETOVERRUN: Sysno = 239; // { int|sys||timer_getoverrun(timer_t timerid); }
pub const SYS_FDATASYNC: Sysno = 241; // { int|sys||fdatasync(int fd); }
pub const SYS_MLOCKALL: Sysno = 242; // { int|sys||mlockall(int flags); }
pub const SYS_MUNLOCKALL: Sysno = 243; // { int|sys||munlockall(void); }
pub const SYS_SIGQUEUEINFO: Sysno = 245; // { int|sys||sigqueueinfo(pid_t pid, const siginfo_t *info); }
pub const SYS_MODCTL: Sysno = 246; // { int|sys||modctl(int cmd, void *arg); }
pub const SYS___POSIX_RENAME: Sysno = 270; // { int|sys||__posix_rename(const char *from, const char *to); }
pub const SYS_SWAPCTL: Sysno = 271; // { int|sys||swapctl(int cmd, void *arg, int misc); }
pub const SYS_MINHERIT: Sysno = 273; // { int|sys||minherit(void *addr, size_t len, int inherit); }
pub const SYS_LCHMOD: Sysno = 274; // { int|sys||lchmod(const char *path, mode_t mode); }
pub const SYS_LCHOWN: Sysno = 275; // { int|sys||lchown(const char *path, uid_t uid, gid_t gid); }
pub const SYS_MSYNC: Sysno = 277; // { int|sys|13|msync(void *addr, size_t len, int flags); }
pub const SYS_SIGALTSTACK: Sysno = 281; // { int|sys|14|sigaltstack( const struct sigaltstack *nss, struct sigaltstack *oss); }
pub const SYS_VFORK: Sysno = 282; // { int|sys|14|vfork(void); }
pub const SYS___POSIX_CHOWN: Sysno = 283; // { int|sys||__posix_chown(const char *path, uid_t uid, gid_t gid); }
pub const SYS___POSIX_FCHOWN: Sysno = 284; // { int|sys||__posix_fchown(int fd, uid_t uid, gid_t gid); }
pub const SYS___POSIX_LCHOWN: Sysno = 285; // { int|sys||__posix_lchown(const char *path, uid_t uid, gid_t gid); }
pub const SYS_GETSID: Sysno = 286; // { pid_t|sys||getsid(pid_t pid); }
pub const SYS___CLONE: Sysno = 287; // { pid_t|sys||__clone(int flags, void *stack); }
pub const SYS_FKTRACE: Sysno = 288; // { int|sys||fktrace(int fd, int ops, int facs, pid_t pid); }
pub const SYS_PREADV: Sysno = 289; // { ssize_t|sys||preadv(int fd, const struct iovec *iovp, int iovcnt, int PAD, off_t offset); }
pub const SYS_PWRITEV: Sysno = 290; // { ssize_t|sys||pwritev(int fd, const struct iovec *iovp, int iovcnt, int PAD, off_t offset); }
pub const SYS_SIGPENDING: Sysno = 292; // { int|sys|14|sigpending(sigset_t *set); }
pub const SYS_SIGPROCMASK: Sysno = 293; // { int|sys|14|sigprocmask(int how, const sigset_t *set, sigset_t *oset); }
pub const SYS_SIGSUSPEND: Sysno = 294; // { int|sys|14|sigsuspend(const sigset_t *set); }
pub const SYS___GETCWD: Sysno = 296; // { int|sys||__getcwd(char *bufp, size_t length); }
pub const SYS_FCHROOT: Sysno = 297; // { int|sys||fchroot(int fd); }
pub const SYS_LCHFLAGS: Sysno = 304; // { int|sys||lchflags(const char *path, u_long flags); }
pub const SYS_ISSETUGID: Sysno = 305; // { int|sys||issetugid(void); }
pub const SYS_UTRACE: Sysno = 306; // { int|sys||utrace(const char *label, void *addr, size_t len); }
pub const SYS_GETCONTEXT: Sysno = 307; // { int|sys||getcontext(struct __ucontext *ucp); }
pub const SYS_SETCONTEXT: Sysno = 308; // { int|sys||setcontext(const struct __ucontext *ucp); }
pub const SYS__LWP_CREATE: Sysno = 309; // { int|sys||_lwp_create(const struct __ucontext *ucp, u_long flags, lwpid_t *new_lwp); }
pub const SYS__LWP_EXIT: Sysno = 310; // { int|sys||_lwp_exit(void); }
pub const SYS__LWP_SELF: Sysno = 311; // { lwpid_t|sys||_lwp_self(void); }
pub const SYS__LWP_WAIT: Sysno = 312; // { int|sys||_lwp_wait(lwpid_t wait_for, lwpid_t *departed); }
pub const SYS__LWP_SUSPEND: Sysno = 313; // { int|sys||_lwp_suspend(lwpid_t target); }
pub const SYS__LWP_CONTINUE: Sysno = 314; // { int|sys||_lwp_continue(lwpid_t target); }
pub const SYS__LWP_WAKEUP: Sysno = 315; // { int|sys||_lwp_wakeup(lwpid_t target); }
pub const SYS__LWP_GETPRIVATE: Sysno = 316; // { void *|sys||_lwp_getprivate(void); }
pub const SYS__LWP_SETPRIVATE: Sysno = 317; // { void|sys||_lwp_setprivate(void *ptr); }
pub const SYS__LWP_KILL: Sysno = 318; // { int|sys||_lwp_kill(lwpid_t target, int signo); }
pub const SYS__LWP_DETACH: Sysno = 319; // { int|sys||_lwp_detach(lwpid_t target); }
pub const SYS__LWP_UNPARK: Sysno = 321; // { int|sys||_lwp_unpark(lwpid_t target, const void *hint); }
pub const SYS__LWP_UNPARK_ALL: Sysno = 322; // { ssize_t|sys||_lwp_unpark_all(const lwpid_t *targets, size_t ntargets, const void *hint); }
pub const SYS__LWP_SETNAME: Sysno = 323; // { int|sys||_lwp_setname(lwpid_t target, const char *name); }
pub const SYS__LWP_GETNAME: Sysno = 324; // { int|sys||_lwp_getname(lwpid_t target, char *name, size_t len); }
pub const SYS__LWP_CTL: Sysno = 325; // { int|sys||_lwp_ctl(int features, struct lwpctl **address); }
pub const SYS___SIGACTION_SIGTRAMP: Sysno = 340; // { int|sys||__sigaction_sigtramp(int signum, const struct sigaction *nsa, struct sigaction *osa, const void *tramp, int vers); }
pub const SYS_RASCTL: Sysno = 343; // { int|sys||rasctl(void *addr, size_t len, int op); }
pub const SYS_KQUEUE: Sysno = 344; // { int|sys||kqueue(void); }
pub const SYS__SCHED_SETPARAM: Sysno = 346; // { int|sys||_sched_setparam(pid_t pid, lwpid_t lid, int policy, const struct sched_param *params); }
pub const SYS__SCHED_GETPARAM: Sysno = 347; // { int|sys||_sched_getparam(pid_t pid, lwpid_t lid, int *policy, struct sched_param *params); }
pub const SYS__SCHED_SETAFFINITY: Sysno = 348; // { int|sys||_sched_setaffinity(pid_t pid, lwpid_t lid, size_t size, const cpuset_t *cpuset); }
pub const SYS__SCHED_GETAFFINITY: Sysno = 349; // { int|sys||_sched_getaffinity(pid_t pid, lwpid_t lid, size_t size, cpuset_t *cpuset); }
pub const SYS_SCHED_YIELD: Sysno = 350; // { int|sys||sched_yield(void); }
pub const SYS_FSYNC_RANGE: Sysno = 354; // { int|sys||fsync_range(int fd, int flags, off_t start, off_t length); }
pub const SYS_UUIDGEN: Sysno = 355; // { int|sys||uuidgen(struct uuid *store, int count); }
pub const SYS_GETVFSSTAT: Sysno = 356; // { int|sys||getvfsstat(struct statvfs *buf, size_t bufsize, int flags); }
pub const SYS_STATVFS1: Sysno = 357; // { int|sys||statvfs1(const char *path, struct statvfs *buf, int flags); }
pub const SYS_FSTATVFS1: Sysno = 358; // { int|sys||fstatvfs1(int fd, struct statvfs *buf, int flags); }
pub const SYS_EXTATTRCTL: Sysno = 360; // { int|sys||extattrctl(const char *path, int cmd, const char *filename, int attrnamespace, const char *attrname); }
pub const SYS_EXTATTR_SET_FILE: Sysno = 361; // { int|sys||extattr_set_file(const char *path, int attrnamespace, const char *attrname, const void *data, size_t nbytes); }
pub const SYS_EXTATTR_GET_FILE: Sysno = 362; // { ssize_t|sys||extattr_get_file(const char *path, int attrnamespace, const char *attrname, void *data, size_t nbytes); }
pub const SYS_EXTATTR_DELETE_FILE: Sysno = 363; // { int|sys||extattr_delete_file(const char *path, int attrnamespace, const char *attrname); }
pub const SYS_EXTATTR_SET_FD: Sysno = 364; // { int|sys||extattr_set_fd(int fd, int attrnamespace, const char *attrname, const void *data, size_t nbytes); }
pub const SYS_EXTATTR_GET_FD: Sysno = 365; // { ssize_t|sys||extattr_get_fd(int fd, int attrnamespace, const char *attrname, void *data, size_t nbytes); }
pub const SYS_EXTATTR_DELETE_FD: Sysno = 366; // { int|sys||extattr_delete_fd(int fd, int attrnamespace, const char *attrname); }
pub const SYS_EXTATTR_SET_LINK: Sysno = 367; // { int|sys||extattr_set_link(const char *path, int attrnamespace, const char *attrname, const void *data, size_t nbytes); }
pub const SYS_EXTATTR_GET_LINK: Sysno = 368; // { ssize_t|sys||extattr_get_link(const char *path, int attrnamespace, const char *attrname, void *data, size_t nbytes); }
pub const SYS_EXTATTR_DELETE_LINK: Sysno = 369; // { int|sys||extattr_delete_link(const char *path, int attrnamespace, const char *attrname); }
pub const SYS_EXTATTR_LIST_FD: Sysno = 370; // { ssize_t|sys||extattr_list_fd(int fd, int attrnamespace, void *data, size_t nbytes); }
pub const SYS_EXTATTR_LIST_FILE: Sysno = 371; // { ssize_t|sys||extattr_list_file(const char *path, int attrnamespace, void *data, size_t nbytes); }
pub const SYS_EXTATTR_LIST_LINK: Sysno = 372; // { ssize_t|sys||extattr_list_link(const char *path, int attrnamespace, void *data, size_t nbytes); }
pub const SYS_SETXATTR: Sysno = 375; // { int|sys||setxattr(const char *path, const char *name, const void *value, size_t size, int flags); }
pub const SYS_LSETXATTR: Sysno = 376; // { int|sys||lsetxattr(const char *path, const char *name, const void *value, size_t size, int flags); }
pub const SYS_FSETXATTR: Sysno = 377; // { int|sys||fsetxattr(int fd, const char *name, const void *value, size_t size, int flags); }
pub const SYS_GETXATTR: Sysno = 378; // { int|sys||getxattr(const char *path, const char *name, void *value, size_t size); }
pub const SYS_LGETXATTR: Sysno = 379; // { int|sys||lgetxattr(const char *path, const char *name, void *value, size_t size); }
pub const SYS_FGETXATTR: Sysno = 380; // { int|sys||fgetxattr(int fd, const char *name, void *value, size_t size); }
pub const SYS_LISTXATTR: Sysno = 381; // { int|sys||listxattr(const char *path, char *list, size_t size); }
pub const SYS_LLISTXATTR: Sysno = 382; // { int|sys||llistxattr(const char *path, char *list, size_t size); }
pub const SYS_FLISTXATTR: Sysno = 383; // { int|sys||flistxattr(int fd, char *list, size_t size); }
pub const SYS_REMOVEXATTR: Sysno = 384; // { int|sys||removexattr(const char *path, const char *name); }
pub const SYS_LREMOVEXATTR: Sysno = 385; // { int|sys||lremovexattr(const char *path, const char *name); }
pub const SYS_FREMOVEXATTR: Sysno = 386; // { int|sys||fremovexattr(int fd, const char *name); }
pub const SYS_GETDENTS: Sysno = 390; // { int|sys|30|getdents(int fd, char *buf, size_t count); }
pub const SYS_SOCKET: Sysno = 394; // { int|sys|30|socket(int domain, int type, int protocol); }
pub const SYS_GETFH: Sysno = 395; // { int|sys|30|getfh(const char *fname, void *fhp, size_t *fh_size); }
pub const SYS_FHOPEN: Sysno = 396; // { int|sys|40|fhopen(const void *fhp, size_t fh_size, int flags); }
pub const SYS_FHSTATVFS1: Sysno = 397; // { int|sys|40|fhstatvfs1(const void *fhp, size_t fh_size, struct statvfs *buf, int flags); }
pub const SYS_MOUNT: Sysno = 410; // { int|sys|50|mount(const char *type, const char *path, int flags, void *data, size_t data_len); }
pub const SYS_MREMAP: Sysno = 411; // { void *|sys||mremap(void *old_address, size_t old_size, void *new_address, size_t new_size, int flags); }
pub const SYS_PSET_CREATE: Sysno = 412; // { int|sys||pset_create(psetid_t *psid); }
pub const SYS_PSET_DESTROY: Sysno = 413; // { int|sys||pset_destroy(psetid_t psid); }
pub const SYS_PSET_ASSIGN: Sysno = 414; // { int|sys||pset_assign(psetid_t psid, cpuid_t cpuid, psetid_t *opsid); }
pub const SYS__PSET_BIND: Sysno = 415; // { int|sys||_pset_bind(idtype_t idtype, id_t first_id, id_t second_id, psetid_t psid, psetid_t *opsid); }
pub const SYS_POSIX_FADVISE: Sysno = 416; // { int|sys|50|posix_fadvise(int fd, int PAD, off_t offset, off_t len, int advice); }
pub const SYS_SELECT: Sysno = 417; // { int|sys|50|select(int nd, fd_set *in, fd_set *ou, fd_set *ex, struct timeval *tv); }
pub const SYS_GETTIMEOFDAY: Sysno = 418; // { int|sys|50|gettimeofday(struct timeval *tp, void *tzp); }
pub const SYS_SETTIMEOFDAY: Sysno = 419; // { int|sys|50|settimeofday(const struct timeval *tv, const void *tzp); }
pub const SYS_UTIMES: Sysno = 420; // { int|sys|50|utimes(const char *path, const struct timeval *tptr); }
pub const SYS_ADJTIME: Sysno = 421; // { int|sys|50|adjtime(const struct timeval *delta, struct timeval *olddelta); }
pub const SYS_FUTIMES: Sysno = 423; // { int|sys|50|futimes(int fd, const struct timeval *tptr); }
pub const SYS_LUTIMES: Sysno = 424; // { int|sys|50|lutimes(const char *path, const struct timeval *tptr); }
pub const SYS_SETITIMER: Sysno = 425; // { int|sys|50|setitimer(int which, const struct itimerval *itv, struct itimerval *oitv); }
pub const SYS_GETITIMER: Sysno = 426; // { int|sys|50|getitimer(int which, struct itimerval *itv); }
pub const SYS_CLOCK_GETTIME: Sysno = 427; // { int|sys|50|clock_gettime(clockid_t clock_id, struct timespec *tp); }
pub const SYS_CLOCK_SETTIME: Sysno = 428; // { int|sys|50|clock_settime(clockid_t clock_id, const struct timespec *tp); }
pub const SYS_CLOCK_GETRES: Sysno = 429; // { int|sys|50|clock_getres(clockid_t clock_id, struct timespec *tp); }
pub const SYS_NANOSLEEP: Sysno = 430; // { int|sys|50|nanosleep(const struct timespec *rqtp, struct timespec *rmtp); }
pub const SYS___SIGTIMEDWAIT: Sysno = 431; // { int|sys|50|__sigtimedwait(const sigset_t *set, siginfo_t *info, struct timespec *timeout); }
pub const SYS_KEVENT: Sysno = 435; // { int|sys|50|kevent(int fd, const struct kevent *changelist, size_t nchanges, struct kevent *eventlist, size_t nevents, const struct timespec *timeout); }
pub const SYS_PSELECT: Sysno = 436; // { int|sys|50|pselect(int nd, fd_set *in, fd_set *ou, fd_set *ex, const struct timespec *ts, const sigset_t *mask); }
pub const SYS_POLLTS: Sysno = 437; // { int|sys|50|pollts(struct pollfd *fds, u_int nfds, const struct timespec *ts, const sigset_t *mask); }
pub const SYS_STAT: Sysno = 439; // { int|sys|50|stat(const char *path, struct stat *ub); }
pub const SYS_FSTAT: Sysno = 440; // { int|sys|50|fstat(int fd, struct stat *sb); }
pub const SYS_LSTAT: Sysno = 441; // { int|sys|50|lstat(const char *path, struct stat *ub); }
pub const SYS_GETRUSAGE: Sysno = 445; // { int|sys|50|getrusage(int who, struct rusage *rusage); }
pub const SYS_TIMER_SETTIME: Sysno = 446; // { int|sys|50|timer_settime(timer_t timerid, int flags, const struct itimerspec *value, struct itimerspec *ovalue); }
pub const SYS_TIMER_GETTIME: Sysno = 447; // { int|sys|50|timer_gettime(timer_t timerid, struct itimerspec *value); }
pub const SYS_NTP_GETTIME: Sysno = 448; // { int|sys|50|ntp_gettime(struct ntptimeval *ntvp); }
pub const SYS_WAIT4: Sysno = 449; // { int|sys|50|wait4(pid_t pid, int *status, int options, struct rusage *rusage); }
pub const SYS_MKNOD: Sysno = 450; // { int|sys|50|mknod(const char *path, mode_t mode, dev_t dev); }
pub const SYS_FHSTAT: Sysno = 451; // { int|sys|50|fhstat(const void *fhp, size_t fh_size, struct stat *sb); }
pub const SYS_PIPE2: Sysno = 453; // { int|sys||pipe2(int *fildes, int flags); }
pub const SYS_DUP3: Sysno = 454; // { int|sys||dup3(int from, int to, int flags); }
pub const SYS_KQUEUE1: Sysno = 455; // { int|sys||kqueue1(int flags); }
pub const SYS_PACCEPT: Sysno = 456; // { int|sys||paccept(int s, struct sockaddr *name, socklen_t *anamelen, const sigset_t *mask, int flags); }
pub const SYS_LINKAT: Sysno = 457; // { int|sys||linkat(int fd1, const char *name1, int fd2, const char *name2, int flags); }
pub const SYS_RENAMEAT: Sysno = 458; // { int|sys||renameat(int fromfd, const char *from, int tofd, const char *to); }
pub const SYS_MKFIFOAT: Sysno = 459; // { int|sys||mkfifoat(int fd, const char *path, mode_t mode); }
pub const SYS_MKNODAT: Sysno = 460; // { int|sys||mknodat(int fd, const char *path, mode_t mode, int PAD, dev_t dev); }
pub const SYS_MKDIRAT: Sysno = 461; // { int|sys||mkdirat(int fd, const char *path, mode_t mode); }
pub const SYS_FACCESSAT: Sysno = 462; // { int|sys||faccessat(int fd, const char *path, int amode, int flag); }
pub const SYS_FCHMODAT: Sysno = 463; // { int|sys||fchmodat(int fd, const char *path, mode_t mode, int flag); }
pub const SYS_FCHOWNAT: Sysno = 464; // { int|sys||fchownat(int fd, const char *path, uid_t owner, gid_t group, int flag); }
pub const SYS_FEXECVE: Sysno = 465; // { int|sys||fexecve(int fd, char * const *argp, char * const *envp); }
pub const SYS_FSTATAT: Sysno = 466; // { int|sys||fstatat(int fd, const char *path, struct stat *buf, int flag); }
pub const SYS_UTIMENSAT: Sysno = 467; // { int|sys||utimensat(int fd, const char *path, const struct timespec *tptr, int flag); }
pub const SYS_OPENAT: Sysno = 468; // { int|sys||openat(int fd, const char *path, int oflags, ... mode_t mode); }
pub const SYS_READLINKAT: Sysno = 469; // { ssize_t|sys||readlinkat(int fd, const char *path, char *buf, size_t bufsize); }
pub const SYS_SYMLINKAT: Sysno = 470; // { int|sys||symlinkat(const char *path1, int fd, const char *path2); }
pub const SYS_UNLINKAT: Sysno = 471; // { int|sys||unlinkat(int fd, const char *path, int flag); }
pub const SYS_FUTIMENS: Sysno = 472; // { int|sys||futimens(int fd, const struct timespec *tptr); }
pub const SYS___QUOTACTL: Sysno = 473; // { int|sys||__quotactl(const char *path, struct quotactl_args *args); }
pub const SYS_POSIX_SPAWN: Sysno = 474; // { int|sys||posix_spawn(pid_t *pid, const char *path, const struct posix_spawn_file_actions *file_actions, const struct posix_spawnattr *attrp, char *const *argv, char *const *envp); }
pub const SYS_RECVMMSG: Sysno = 475; // { int|sys||recvmmsg(int s, struct mmsghdr *mmsg, unsigned int vlen, unsigned int flags, struct timespec *timeout); }
pub const SYS_SENDMMSG: Sysno = 476; // { int|sys||sendmmsg(int s, struct mmsghdr *mmsg, unsigned int vlen, unsigned int flags); }
pub const SYS_CLOCK_NANOSLEEP: Sysno = 477; // { int|sys||clock_nanosleep(clockid_t clock_id, int flags, const struct timespec *rqtp, struct timespec *rmtp); }
pub const SYS__LWP_PARK: Sysno = 478; // { int|sys|60|_lwp_park(clockid_t clock_id, int flags, struct timespec *ts, lwpid_t unpark, const void *hint, const void *unparkhint); }
pub const SYS_POSIX_FALLOCATE: Sysno = 479; // { int|sys||posix_fallocate(int fd, int PAD, off_t pos, off_t len); }
pub const SYS_FDISCARD: Sysno = 480; // { int|sys||fdiscard(int fd, int PAD, off_t pos, off_t len); }
pub const SYS_WAIT6: Sysno = 481; // { int|sys||wait6(idtype_t idtype, id_t id, int *status, int options, struct wrusage *wru, siginfo_t *info); }
pub const SYS_CLOCK_GETCPUCLOCKID2: Sysno = 482; // { int|sys||clock_getcpuclockid2(idtype_t idtype, id_t id, clockid_t *clock_id); }