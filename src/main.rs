use netstat2::*;
use sysinfo::{Pid, Signal, System};

fn get_pids(port: u16) -> Vec<u32> {
    let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
    let proto_flags = ProtocolFlags::TCP | ProtocolFlags::UDP;
    let sockets_info = get_sockets_info(af_flags, proto_flags).unwrap();

    let mut pids = Vec::new();
    for si in sockets_info {
        match si.protocol_socket_info {
            ProtocolSocketInfo::Tcp(tcp_si) => {
                if tcp_si.local_port == port {
                    pids.push(si.associated_pids[0]);
                }
            }
            ProtocolSocketInfo::Udp(udp_si) => {
                if udp_si.local_port == port {
                    pids.push(si.associated_pids[0]);
                }
            }
        }
    }
    pids
}

fn main() {
    // usage: close <port>
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2  && args.len() != 3 {
        println!("Usage: close <port> (<signal>)");
        return;
    }

    let port = args[1].parse::<u16>().unwrap_or_else(|_| {
        println!("Invalid port number");
        std::process::exit(1);
    });

    let signal = args.get(2).map(|s| s.as_str()).unwrap_or("-9");

    let signal_num = signal
        .get(1..)
        .unwrap_or_else(|| {
            println!("Invalid signal number");
            std::process::exit(1);
        })
        .parse::<i32>()
        .unwrap_or_else(|_| {
            println!("Invalid signal number");
            std::process::exit(1);
        });

    let signal_obj = match signal_num {
        /*
           hangup, interrupt, quit, kill, terminate, illegal, trap, bus, floating point exception, kill, pipe, alarm, term
        */
        // correspond to unix signal numbers
        1 => Signal::Hangup,
        2 => Signal::Interrupt,
        3 => Signal::Quit,
        4 => Signal::Illegal,
        5 => Signal::Trap,
        6 => Signal::Abort,
        7 => Signal::Bus,
        8 => Signal::FloatingPointException,
        9 => Signal::Kill,
        10 => Signal::User1,
        11 => Signal::Segv,
        12 => Signal::User2,
        13 => Signal::Pipe,
        15 => Signal::Term,
        17 => Signal::Child,
        19 => Signal::Stop,
        _ => {
            println!("Invalid signal number");
            std::process::exit(1);
        }
    };

    let pids = get_pids(port);

    for pid in pids {
        let mut system = System::new();
        system.refresh_all();
        let process = system.process(Pid::from_u32(pid));
        if let Some(process) = process {
            println!("Killing {} ({}) on {}", process.name(), pid, port);
            match process.kill_with(signal_obj) {
                Some(b) => {
                    if b {
                        println!("Signal sent");
                        continue;
                    } else {
                        println!("Failed to kill, trying SIGKILL");
                    }
                }
                None => {
                    println!("Failed to kill, trying SIGKILL");
                }
            }

            // failed to kill with specified signal, try to kill with SIGKILL
            match process.kill() {
                true => {
                    println!("Signal sent");
                }
                false => {
                    println!("Failed to kill, even with SIGKILL");
                }
            }
        } else {
            println!("Failed to find process {}", pid);
        }
    }
}
