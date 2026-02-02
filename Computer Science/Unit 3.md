# 5 - Network and web technologies
## 57 Purpose of a network
**Definition**: **2+** computers/devices **connected** to **communicate**/share resources
## 58 Protocols
**Definition**: A set of **rules** for **communication**
### List of protocols
- HTTP/HTTPS -> Hypertext Transfer Protocol
- TCP/IP -> Transmission Control Program/Internet Protocol
- UDP -> User Datagram Protocol
... 
## 59 Layering protocols
Layering protocols is beneficial for abstraction and modularity.
### TCP/IP Stack
- **Application**: Request is sent HTTP(S), FTP, POP3
- **Transport**: Splits into **packets** and **adds ports** (TCP/UDP)
- **Internet**: Adds **IP** addresses (IP, ICMP)
- **Link**: Adds **MAC** addresses (MAC, ETHERNET, WIFI)

Computers have ATIL stacks, routers only have IL stacks (because application and transport is not necessary).
### Protocols on the stack
- **Application**: HTTP(S), FTP, POP3
- **Transport**: TCP/UDP
- **Internet**: IP, ICMP
- **Link**: MAC, ETHERNET, WIFI
### Software/hardware at each layer
- **Application**: HTTP(S), FTP, POP3
- **Transport**: TCP/UDP
- **Internet**: router, firewall
- **Link**: NIC, switch, WAP
## 60 Networks
- **LAN**: Local area network (small geographical area)
- **WAN**: Wide area network (large geographical area)
## 61 DNS
### How it works
- Domain name is sent to the **DNS**
- Resolvers (or DNS servers) return the URL
	- If not found, it looks up the chain (another DNS server)
		- Until it reaches the authoritative name server
		- Or an error is returned

**Resolvers** cache DNS records. These include: your computer, the LAN, the ISP.
## 62 Packet and circuit switching

| Circuit switching                            | Packet switching                                             |
| -------------------------------------------- | ------------------------------------------------------------ |
| Direct/fixed link for duration of connection | Data split up into packets and sent through different routes |
| Data arrives in the order it was sent        | Data can arrive out of order                                 |
| Makes sense to pay per minute                | Makes sense to pay per file size                             |

Packets have **labels** that include the addresses being sent to. 
## 63 Security issues and threat

| Threats             | Prevention         |
| ------------------- | ------------------ |
| Hackers             | Password policies  |
| Unauthorised access | Software firewalls |
| DoS                 | Hardware firewalls |
| Spyware             | Anti-spyware       |
| SQL Injection       | Sanitisation       |

Pharming tampers DNS server.
Firewall definition: Hardware/software that monitors and filters traffic going <ins>to and from</ins> a network.
## 64 Internet hardware
- Switch: sends data to the correct device (depending on the MAC address)
- Hub: transmits data to all devices
- Routers: forwards packets between networks
- WAP: A hub but wireless
- Modem: sends/receives signals to/from ISP
## 65 & 66 CS and P2P
Client-server vs peer to peer are ways of providing services.
In **client-server**, all nodes are connected to a central server (the internet). In a **peer-to-peer** network, all nodes are connected to each other.
### Benefits and drawbacks of each

| Client-server                  | Peer-to-peer                    |
| ------------------------------ | ------------------------------- |
| Centralised control            | Easy setup                      |
| Scalable with powerful servers | No expensive servers            |
| Backup from one place only     | Peers share directly            |
| Servers can be expensive       | No single point of failure      |
| Requires sysadmin skills       | Can't control centrally         |
| Single point of failure        | Peers may be slow at some tasks |
