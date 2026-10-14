# vmnet

Apple's vmnet framework as of the macOS 27 SDK, in C. Macs only.

vmnet makes virtual networks. A `vmnet_network_ref` can back a virtual machine's network device through Virtualization's `VZVmnetNetworkDeviceAttachment.init(network:)`.

Swift imports vmnet's reference types as `OpaquePointer`, so Swift doesn't manage them. Each is a Core Foundation object: release it with `CFRelease`.

vmnet has two halves. The networks below (macOS 26+) are what Virtualization uses. The older interface API (`interface_ref`, `vmnet_start_interface`, `vmnet_read`/`vmnet_write`, port forwarding on a started interface, the XPC dictionary keys) runs a network interface in-process, and takes and returns XPC objects. So do a network's serialization functions (`vmnet_network_copy_serialization`, `vmnet_network_create_with_serialization`).

## vmnet_network_configuration_ref

`typealias vmnet_network_configuration_ref = OpaquePointer`

- `func vmnet_network_configuration_create(_ mode: vmnet_mode_t, _ status: UnsafeMutablePointer<vmnet_return_t>?) -> vmnet_network_configuration_ref?`: shared or host-only mode; `NULL` on failure, with `status` set. Defaults: the routing table's default external interface; NAT44, NAT66, DHCP, DNS proxy, and router advertisement enabled; a /24 IPv4 subnet under 192.168/16; a random unique local IPv6 prefix; no port forwarding rules or DHCP reservations; an MTU of 1500.
- `func vmnet_network_configuration_add_dhcp_reservation(_ config: vmnet_network_configuration_ref, _ client: UnsafePointer<ether_addr_t>, _ reservation: UnsafePointer<in_addr>) -> vmnet_return_t`: reserves an address for a MAC address. Not while a network is active.
- `func vmnet_network_configuration_add_port_forwarding_rule(_ config: vmnet_network_configuration_ref, _ protocol: UInt8, _ address_family: sa_family_t, _ internal_port: UInt16, _ external_port: UInt16, _ internal_address: UnsafeRawPointer) -> vmnet_return_t`: `protocol` is `IPPROTO_TCP` or `IPPROTO_UDP`; `address_family` is `AF_INET` (an `in_addr`) or `AF_INET6` (an `in6_addr`); ports in host byte order.
- `func vmnet_network_configuration_disable_router_advertisement(_ config: vmnet_network_configuration_ref)`
- `func vmnet_network_configuration_disable_nat66(_ config: vmnet_network_configuration_ref)`
- `func vmnet_network_configuration_disable_nat44(_ config: vmnet_network_configuration_ref)`
- `func vmnet_network_configuration_disable_dns_proxy(_ config: vmnet_network_configuration_ref)`
- `func vmnet_network_configuration_disable_dhcp(_ config: vmnet_network_configuration_ref)`
- `func vmnet_network_configuration_set_external_interface(_ config: vmnet_network_configuration_ref, _ interface_name: UnsafePointer<CChar>) -> vmnet_return_t`: shared mode only.
- `func vmnet_network_configuration_set_ipv4_subnet(_ config: vmnet_network_configuration_ref, _ subnet_addr: UnsafePointer<in_addr>, _ subnet_mask: UnsafePointer<in_addr>) -> vmnet_return_t`: the first, second, and last addresses are reserved; the second is the host's.
- `func vmnet_network_configuration_set_ipv6_prefix(_ config: vmnet_network_configuration_ref, _ prefix: UnsafePointer<in6_addr>, _ len: UInt8) -> vmnet_return_t`
- `func vmnet_network_configuration_set_mtu(_ config: vmnet_network_configuration_ref, _ mtu: UInt32) -> vmnet_return_t`

## vmnet_network_ref

`typealias vmnet_network_ref = OpaquePointer`

- `func vmnet_network_create(_ configuration: vmnet_network_configuration_ref, _ status: UnsafeMutablePointer<vmnet_return_t>?) -> vmnet_network_ref?`: reserves the configuration's resources for the network's lifetime, so interfaces started on it don't fail for contention. `NULL` on failure.
- `func vmnet_network_get_ipv4_subnet(_ network: vmnet_network_ref, _ subnet: UnsafeMutablePointer<in_addr>, _ mask: UnsafeMutablePointer<in_addr>)`
- `func vmnet_network_get_ipv6_prefix(_ network: vmnet_network_ref, _ prefix: UnsafeMutablePointer<in6_addr>, _ prefix_len: UnsafeMutablePointer<UInt8>)`
- `func vmnet_network_copy_serialization(_ network: vmnet_network_ref, _ status: UnsafeMutablePointer<vmnet_return_t>?) -> xpc_object_t?`
- `func vmnet_network_create_with_serialization(_ network: xpc_object_t, _ status: UnsafeMutablePointer<vmnet_return_t>?) -> vmnet_network_ref?`

## vmnet_mode_t

`typealias vmnet_mode_t = operating_modes_t`

`enum operating_modes_t: UInt32`

- `VMNET_HOST_MODE`
- `VMNET_SHARED_MODE`
- `VMNET_BRIDGED_MODE`

## vmnet_return_t

`enum vmnet_return_t: UInt32`

- `VMNET_SUCCESS`
- `VMNET_FAILURE`
- `VMNET_MEM_FAILURE`
- `VMNET_INVALID_ARGUMENT`
- `VMNET_SETUP_INCOMPLETE`
- `VMNET_INVALID_ACCESS`
- `VMNET_PACKET_TOO_BIG`
- `VMNET_BUFFER_EXHAUSTED`
- `VMNET_TOO_MANY_PACKETS`
- `VMNET_SHARING_SERVICE_BUSY`
- `VMNET_NOT_AUTHORIZED`
