# Changelog

## Unreleased

## v0.1.1

- Add a README and changelog.
- Move `Return` from `error.rs` to `return.rs`, matching `Return.swift`. The
  public API is unchanged.

## v0.1.0

- Network configurations: host, shared, and bridged modes.
- Subnets, IPv6 prefixes, MTU, external interface.
- DHCP reservations, port forwarding; disabling DHCP, DNS proxy, NAT, router
  advertisement.
- Networks: create, read subnet and prefix.
