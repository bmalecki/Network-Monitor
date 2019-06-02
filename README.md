# Network monitoring using PF_NETLINK socket

## Running
    curl https://sh.rustup.rs -sSf | sh     # Install Rust language

    cargo run                               # Dev verison compilation

or 

    cargo build --realese                   # Prod version compilation
    ./target/release/projekt-zaliczeniowy

## Test commands

    ip route add 192.168.122.0/25 via 192.168.0.33
    ip route del 192.168.122.0/25

    ip addr add 10.0.0.2/8 dev vboxnet1
    ip addr del 10.0.0.2/8 dev vboxnet1

    ip -6 addr del 2001:0db8:0:f101::1/64 dev lo
    ip -6 addr add 2001:0db8:0:f101::1/64 dev lo

    ip link add link vboxnet1 name vboxnet4 type vlan id 10
    ip link delete dev vboxnet4
    ip link set dev vboxnet4 up
    ip link set vboxnet4 promisc on

    /etc/init.d/network stop
    ip link set eth0 address 02:01:02:03:g04:08
    /etc/init.d/network start

    ip n flush all

## Helpfull links:
 - https://www.linuxjournal.com/article/8498