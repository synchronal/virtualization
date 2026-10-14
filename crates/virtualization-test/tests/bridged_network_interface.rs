use virtualization::BridgedNetworkInterface;

#[test]
fn lists_the_hosts_interfaces_by_their_distinct_bsd_names() {
  let mut identifiers: Vec<String> = BridgedNetworkInterface::network_interfaces()
    .unwrap()
    .iter()
    .map(BridgedNetworkInterface::identifier)
    .collect();

  assert!(!identifiers.is_empty());
  assert!(identifiers.iter().all(|identifier| !identifier.is_empty()));
  identifiers.sort();
  identifiers.dedup();
  assert_eq!(
    identifiers.len(),
    BridgedNetworkInterface::network_interfaces().unwrap().len()
  );
}

#[test]
fn a_clone_is_the_same_object() {
  let interface = &BridgedNetworkInterface::network_interfaces().unwrap()[0];

  assert_eq!(interface.clone().identifier(), interface.identifier());
}

#[test]
fn debugs_as_its_description_its_display_name_and_bsd_name() {
  for interface in BridgedNetworkInterface::network_interfaces().unwrap() {
    if let Some(name) = interface.localized_display_name() {
      assert_eq!(format!("{interface:?}"), format!("{name} ({})", interface.identifier()));
    }
  }
}
