# `shut`
> because sometimes a process needs to shut up

<img src="static/shut.png" width="100">

`shut` is a simple, cross platform tool for shutting down processes.
## supported signals
1 - SIGHUP
2 - SIGINT
3 - SIGQUIT
4 - SIGILL
5 - SIGTRAP
6 - SIGABRT
7 - SIGBUS
8 - SIGFPE
9 - SIGKILL
10 - SIGUSR1
11 - SIGSEGV
12 - SIGUSR2
13 - SIGPIPE
15 - SIGTERM
17 - SIGCHLD
19 - SIGSTOP
## usage
```sh
shut <pid> (<signal>)
```
`signal` defaults to `SIGKILL`
## errors
- if a signal does not exist on a platform, or the first kill fails, `shut` will try to use `SIGKILL`
- if this still doesn't work, it will print an error message