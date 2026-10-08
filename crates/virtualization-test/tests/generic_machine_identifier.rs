use virtualization::GenericMachineIdentifier;

#[test]
fn is_unique_when_new() {
  assert_ne!(
    GenericMachineIdentifier::new().unwrap(),
    GenericMachineIdentifier::new().unwrap()
  );
}

#[test]
fn equals_one_made_from_its_data_representation() {
  let identifier = GenericMachineIdentifier::new().unwrap();

  let from_data = GenericMachineIdentifier::with_data_representation(&identifier.data_representation())
    .unwrap()
    .unwrap();

  assert_eq!(from_data, identifier);
  assert_eq!(from_data.data_representation(), identifier.data_representation());
}

#[test]
fn is_none_from_invalid_data() {
  assert!(
    GenericMachineIdentifier::with_data_representation(b"not an identifier")
      .unwrap()
      .is_none()
  );
}

#[test]
fn a_clone_is_equal() {
  let identifier = GenericMachineIdentifier::new().unwrap();

  assert_eq!(identifier.clone(), identifier);
}

#[test]
fn debugs_as_its_description() {
  let identifier = GenericMachineIdentifier::new().unwrap();

  assert!(format!("{identifier:?}").starts_with("<VZGenericMachineIdentifier: 0x"));
}
