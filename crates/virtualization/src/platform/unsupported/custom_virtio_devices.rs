use std::any::Any;
use std::sync::Arc;

use dispatch2::{DispatchQueue, DispatchRetained};

use super::{
  BridgedNil, BridgedThrown, GUARDED, Local, MemoryPages, NSError, NSObject, Never, Retained, VZGuestMemoryMapping,
};

pub(crate) struct VZCustomVirtioDevice {
  never: Never,
}

impl VZCustomVirtioDevice {
  pub(crate) fn bridge_retained(&self) -> VZCustomVirtioDevice {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_delegate(&self) -> Option<Arc<dyn VZCustomVirtioDeviceDelegate>> {
    match self.never {}
  }

  pub(crate) fn vz_set_delegate(&self, _delegate: Option<Arc<dyn VZCustomVirtioDeviceDelegate>>) {
    match self.never {}
  }

  pub(crate) fn vz_device_queue(&self) -> DispatchRetained<DispatchQueue> {
    match self.never {}
  }

  pub(crate) fn vz_negotiated_features(&self) -> Result<VZNegotiatedVirtioFeatureSet, BridgedNil> {
    match self.never {}
  }

  pub(crate) fn vz_shared_memory_regions(&self) -> Vec<VZVirtioSharedMemoryRegion> {
    match self.never {}
  }

  pub(crate) fn vz_guest_memory_mapping(
    &self,
    _physical_address: u64,
    _length: isize,
  ) -> Result<VZGuestMemoryMapping, BridgedNil> {
    match self.never {}
  }

  pub(crate) fn vz_queue(&self, _index: u16) -> Result<VZVirtioQueue, BridgedNil> {
    match self.never {}
  }

  pub(crate) fn vz_request_reset(&self) {
    match self.never {}
  }

  pub(crate) fn vz_update(
    &self,
    _new_configuration: &VZVirtioDeviceSpecificConfiguration,
    _completion_handler: impl FnOnce(Option<Retained<NSError>>) + Send + 'static,
  ) {
    match self.never {}
  }
}

pub(crate) trait VZCustomVirtioDeviceDelegate: Any + Send + Sync {}

impl<T: Any + Send + Sync> VZCustomVirtioDeviceDelegate for T {}

pub(crate) struct VZCustomVirtioDeviceConfiguration {
  never: Never,
  local: Local,
}

pub(crate) fn vz_custom_virtio_device_configuration_init() -> VZCustomVirtioDeviceConfiguration {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_custom_virtio_device_configuration_maximum_allowed_shared_memory_region_count() -> isize {
  unreachable!("{GUARDED}")
}

impl VZCustomVirtioDeviceConfiguration {
  pub(crate) fn bridge_retained(&self) -> VZCustomVirtioDeviceConfiguration {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_device_id(&self) -> u16 {
    match self.never {}
  }

  pub(crate) fn vz_set_device_id(&self, _device_id: u16) {
    match self.never {}
  }

  pub(crate) fn vz_pci_class_id(&self) -> u8 {
    match self.never {}
  }

  pub(crate) fn vz_set_pci_class_id(&self, _pci_class_id: u8) {
    match self.never {}
  }

  pub(crate) fn vz_pci_subclass_id(&self) -> u8 {
    match self.never {}
  }

  pub(crate) fn vz_set_pci_subclass_id(&self, _pci_subclass_id: u8) {
    match self.never {}
  }

  pub(crate) fn vz_virtio_queue_count(&self) -> u16 {
    match self.never {}
  }

  pub(crate) fn vz_set_virtio_queue_count(&self, _virtio_queue_count: u16) {
    match self.never {}
  }

  pub(crate) fn vz_mandatory_features(&self) -> VZVirtioFeatureSet {
    match self.never {}
  }

  pub(crate) fn vz_optional_features(&self) -> VZVirtioFeatureSet {
    match self.never {}
  }

  pub(crate) fn vz_device_specific_configuration(&self) -> Result<VZVirtioDeviceSpecificConfiguration, BridgedNil> {
    match self.never {}
  }

  pub(crate) fn vz_set_device_specific_configuration(
    &self,
    _device_specific_configuration: VZVirtioDeviceSpecificConfiguration,
  ) {
    match self.never {}
  }

  pub(crate) fn bridge_set_device_specific_configuration_nil(&self) {
    match self.never {}
  }

  pub(crate) fn vz_provider(&self) -> Option<Retained<NSObject>> {
    match self.never {}
  }

  pub(crate) fn vz_set_provider(&self, _provider: Option<&NSObject>) {
    match self.never {}
  }

  pub(crate) fn vz_shared_memory_regions(&self) -> Vec<VZVirtioSharedMemoryRegionConfiguration> {
    match self.never {}
  }

  pub(crate) fn vz_set_shared_memory_regions(
    &self,
    _shared_memory_regions: &[&VZVirtioSharedMemoryRegionConfiguration],
  ) {
    match self.never {}
  }

  pub(crate) fn vz_supports_save_restore(&self) -> bool {
    match self.never {}
  }

  pub(crate) fn vz_set_supports_save_restore(&self, _supports_save_restore: bool) {
    match self.never {}
  }
}

pub(crate) struct VZCustomVirtioDeviceDelegateProvider {
  never: Never,
  local: Local,
}

pub(crate) fn vz_custom_virtio_device_delegate_provider_init(
  _device_queue: &DispatchQueue,
  _delegate: Arc<dyn VZCustomVirtioDeviceConfigurationDelegate>,
) -> VZCustomVirtioDeviceDelegateProvider {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_custom_virtio_device_delegate_provider_bridge_cast(
  object: &NSObject,
) -> Option<VZCustomVirtioDeviceDelegateProvider> {
  object.never()
}

impl VZCustomVirtioDeviceDelegateProvider {
  pub(crate) fn bridge_retained(&self) -> VZCustomVirtioDeviceDelegateProvider {
    match self.never {}
  }

  pub(crate) fn bridge_object(&self) -> &NSObject {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_device_queue(&self) -> DispatchRetained<DispatchQueue> {
    match self.never {}
  }

  pub(crate) fn vz_delegate(&self) -> Option<Arc<dyn VZCustomVirtioDeviceConfigurationDelegate>> {
    match self.never {}
  }
}

pub(crate) trait VZCustomVirtioDeviceConfigurationDelegate: Any + Send + Sync {}

impl<T: Any + Send + Sync> VZCustomVirtioDeviceConfigurationDelegate for T {}

pub(crate) struct VZNegotiatedVirtioFeatureSet {
  never: Never,
  local: Local,
}

impl VZNegotiatedVirtioFeatureSet {
  pub(crate) fn bridge_retained(&self) -> VZNegotiatedVirtioFeatureSet {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_subset0(&self) -> u32 {
    match self.never {}
  }

  pub(crate) fn vz_subset1(&self) -> u32 {
    match self.never {}
  }
}

pub(crate) struct VZVirtioDeviceSpecificConfiguration {
  never: Never,
  local: Local,
}

pub(crate) fn vz_virtio_device_specific_configuration_init(
  _configuration_data: &[u8],
) -> VZVirtioDeviceSpecificConfiguration {
  unreachable!("{GUARDED}")
}

impl VZVirtioDeviceSpecificConfiguration {
  pub(crate) fn bridge_retained(&self) -> VZVirtioDeviceSpecificConfiguration {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_configuration_data(&self) -> Vec<u8> {
    match self.never {}
  }
}

pub(crate) struct VZVirtioFeatureSet {
  never: Never,
  local: Local,
}

impl VZVirtioFeatureSet {
  pub(crate) fn bridge_retained(&self) -> VZVirtioFeatureSet {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_subset0(&self) -> u32 {
    match self.never {}
  }

  pub(crate) fn vz_set_subset0(&self, _subset0: u32) {
    match self.never {}
  }

  pub(crate) fn vz_subset1(&self) -> u32 {
    match self.never {}
  }

  pub(crate) fn vz_set_subset1(&self, _subset1: u32) {
    match self.never {}
  }
}

pub(crate) struct VZVirtioQueue {
  never: Never,
}

impl VZVirtioQueue {
  pub(crate) fn bridge_retained(&self) -> VZVirtioQueue {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_next_element(&self) -> Result<VZVirtioQueueElement, BridgedNil> {
    match self.never {}
  }

  pub(crate) fn vz_queue_index(&self) -> u16 {
    match self.never {}
  }

  pub(crate) fn vz_queue_size(&self) -> u16 {
    match self.never {}
  }
}

pub(crate) struct VZVirtioQueueElement {
  never: Never,
}

impl VZVirtioQueueElement {
  pub(crate) fn bridge_retained(&self) -> VZVirtioQueueElement {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_read_buffers_byte_count(&self) -> isize {
    match self.never {}
  }

  pub(crate) fn vz_read_buffers_available_byte_count(&self) -> isize {
    match self.never {}
  }

  pub(crate) fn vz_write_buffers_byte_count(&self) -> isize {
    match self.never {}
  }

  pub(crate) fn vz_write_buffers_available_byte_count(&self) -> isize {
    match self.never {}
  }

  pub(crate) fn vz_written_byte_count(&self) -> isize {
    match self.never {}
  }

  pub(crate) fn vz_read_buffers(&self) -> Vec<Vec<u8>> {
    match self.never {}
  }

  pub(crate) fn vz_peek_into_read_buffers(&self, _exact_length: isize) -> Result<Vec<u8>, BridgedThrown> {
    match self.never {}
  }

  pub(crate) fn vz_read_bytes(&self, _exact_length: isize) -> Result<Vec<u8>, BridgedThrown> {
    match self.never {}
  }

  pub(crate) fn vz_read_bytes_into_buffer(&self, _buffer: &mut [u8]) -> Result<(), BridgedThrown> {
    match self.never {}
  }

  pub(crate) fn vz_write(&self, _data: &[u8]) -> Result<(), BridgedThrown> {
    match self.never {}
  }

  pub(crate) fn vz_write_buffer(&self, _buffer: &[u8]) -> Result<(), BridgedThrown> {
    match self.never {}
  }

  pub(crate) fn vz_return_to_queue(&self) {
    match self.never {}
  }
}

pub(crate) struct VZVirtioSharedMemoryRegion {
  never: Never,
}

impl VZVirtioSharedMemoryRegion {
  pub(crate) fn bridge_retained(&self) -> VZVirtioSharedMemoryRegion {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_region_id(&self) -> u8 {
    match self.never {}
  }

  pub(crate) fn vz_size(&self) -> u64 {
    match self.never {}
  }

  pub(crate) fn vz_map_memory_pages(
    &self,
    _memory: &MemoryPages,
    _offset: u64,
    _completion_handler: impl FnOnce(Option<Retained<NSError>>) + Send + 'static,
  ) {
    match self.never {}
  }

  pub(crate) fn vz_unmap_memory(
    &self,
    _offset: u64,
    _size: u64,
    _completion_handler: impl FnOnce(Option<Retained<NSError>>) + Send + 'static,
  ) {
    match self.never {}
  }
}

pub(crate) struct VZVirtioSharedMemoryRegionConfiguration {
  never: Never,
  local: Local,
}

pub(crate) fn vz_virtio_shared_memory_region_configuration_init(
  _region_id: u8,
  _size: u64,
) -> VZVirtioSharedMemoryRegionConfiguration {
  unreachable!("{GUARDED}")
}

impl VZVirtioSharedMemoryRegionConfiguration {
  pub(crate) fn bridge_retained(&self) -> VZVirtioSharedMemoryRegionConfiguration {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_region_id(&self) -> u8 {
    match self.never {}
  }

  pub(crate) fn vz_size(&self) -> u64 {
    match self.never {}
  }
}
