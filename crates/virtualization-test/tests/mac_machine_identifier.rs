use virtualization::MacMachineIdentifier;

#[test]
fn is_unique_when_new() {
  assert_ne!(
    MacMachineIdentifier::new().unwrap(),
    MacMachineIdentifier::new().unwrap()
  );
}

#[test]
fn equals_one_made_from_its_data_representation() {
  let identifier = MacMachineIdentifier::new().unwrap();

  let from_data = MacMachineIdentifier::with_data_representation(&identifier.data_representation())
    .unwrap()
    .unwrap();

  assert_eq!(from_data, identifier);
  assert_eq!(from_data.data_representation(), identifier.data_representation());
}

#[test]
fn is_none_from_invalid_data() {
  assert!(
    MacMachineIdentifier::with_data_representation(b"not an identifier")
      .unwrap()
      .is_none()
  );
}

#[test]
fn a_clone_is_equal() {
  let identifier = MacMachineIdentifier::new().unwrap();

  assert_eq!(identifier.clone(), identifier);
}

#[test]
fn debugs_as_its_description() {
  let identifier = MacMachineIdentifier::new().unwrap();

  assert!(format!("{identifier:?}").starts_with("<VZMacMachineIdentifier: 0x"));
}
