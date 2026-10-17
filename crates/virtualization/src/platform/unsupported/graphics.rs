use std::sync::Arc;

use super::{BridgedCGSize, BridgedThrown, GUARDED, NSObject, NSScreen, Never, Retained};
use crate::GraphicsDisplayObserver;

pub(crate) fn vz_graphics_device_displays(this: &NSObject) -> Vec<Retained<NSObject>> {
  this.never()
}

pub(crate) fn vz_graphics_display_size_in_pixels(this: &NSObject) -> BridgedCGSize {
  this.never()
}

pub(crate) fn vz_graphics_display_reconfigure_size_in_pixels(
  this: &NSObject,
  _size_in_pixels: BridgedCGSize,
) -> Result<(), BridgedThrown> {
  this.never()
}

pub(crate) fn vz_graphics_display_reconfigure_configuration(
  this: &NSObject,
  _configuration: &NSObject,
) -> Result<(), BridgedThrown> {
  this.never()
}

pub(crate) fn vz_graphics_display_add_observer(
  this: &NSObject,
  _observer: Arc<dyn GraphicsDisplayObserver>,
  _key: usize,
) {
  this.never()
}

pub(crate) fn vz_graphics_display_remove_observer(
  this: &NSObject,
  _observer: Arc<dyn GraphicsDisplayObserver>,
  _key: usize,
) {
  this.never()
}

pub(crate) struct VZHostAudioInputStreamSource {
  never: Never,
}

pub(crate) fn vz_host_audio_input_stream_source_init() -> VZHostAudioInputStreamSource {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_host_audio_input_stream_source_bridge_cast(object: &NSObject) -> Option<VZHostAudioInputStreamSource> {
  object.never()
}

impl VZHostAudioInputStreamSource {
  pub(crate) fn bridge_retained(&self) -> VZHostAudioInputStreamSource {
    match self.never {}
  }

  pub(crate) fn bridge_object(&self) -> &NSObject {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }
}

pub(crate) struct VZHostAudioOutputStreamSink {
  never: Never,
}

pub(crate) fn vz_host_audio_output_stream_sink_init() -> VZHostAudioOutputStreamSink {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_host_audio_output_stream_sink_bridge_cast(object: &NSObject) -> Option<VZHostAudioOutputStreamSink> {
  object.never()
}

impl VZHostAudioOutputStreamSink {
  pub(crate) fn bridge_retained(&self) -> VZHostAudioOutputStreamSink {
    match self.never {}
  }

  pub(crate) fn bridge_object(&self) -> &NSObject {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }
}

pub(crate) struct VZUSBKeyboardConfiguration {
  never: Never,
}

pub(crate) fn vz_usb_keyboard_configuration_init() -> VZUSBKeyboardConfiguration {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_usb_keyboard_configuration_bridge_cast(object: &NSObject) -> Option<VZUSBKeyboardConfiguration> {
  object.never()
}

impl VZUSBKeyboardConfiguration {
  pub(crate) fn bridge_retained(&self) -> VZUSBKeyboardConfiguration {
    match self.never {}
  }

  pub(crate) fn bridge_object(&self) -> &NSObject {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }
}

pub(crate) struct VZUSBScreenCoordinatePointingDeviceConfiguration {
  never: Never,
}

pub(crate) fn vz_usb_screen_coordinate_pointing_device_configuration_init()
-> VZUSBScreenCoordinatePointingDeviceConfiguration {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_usb_screen_coordinate_pointing_device_configuration_bridge_cast(
  object: &NSObject,
) -> Option<VZUSBScreenCoordinatePointingDeviceConfiguration> {
  object.never()
}

impl VZUSBScreenCoordinatePointingDeviceConfiguration {
  pub(crate) fn bridge_retained(&self) -> VZUSBScreenCoordinatePointingDeviceConfiguration {
    match self.never {}
  }

  pub(crate) fn bridge_object(&self) -> &NSObject {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }
}

pub(crate) struct VZVirtioGraphicsDevice {
  never: Never,
}

pub(crate) fn vz_virtio_graphics_device_bridge_cast(object: &NSObject) -> Option<VZVirtioGraphicsDevice> {
  object.never()
}

impl VZVirtioGraphicsDevice {
  pub(crate) fn bridge_retained(&self) -> VZVirtioGraphicsDevice {
    match self.never {}
  }

  pub(crate) fn bridge_object(&self) -> &NSObject {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }
}

pub(crate) struct VZVirtioGraphicsDeviceConfiguration {
  never: Never,
}

pub(crate) fn vz_virtio_graphics_device_configuration_init() -> VZVirtioGraphicsDeviceConfiguration {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_virtio_graphics_device_configuration_bridge_cast(
  object: &NSObject,
) -> Option<VZVirtioGraphicsDeviceConfiguration> {
  object.never()
}

impl VZVirtioGraphicsDeviceConfiguration {
  pub(crate) fn bridge_retained(&self) -> VZVirtioGraphicsDeviceConfiguration {
    match self.never {}
  }

  pub(crate) fn bridge_object(&self) -> &NSObject {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_scanouts(&self) -> Vec<VZVirtioGraphicsScanoutConfiguration> {
    match self.never {}
  }

  pub(crate) fn vz_set_scanouts(&self, _scanouts: &[&VZVirtioGraphicsScanoutConfiguration]) {
    match self.never {}
  }
}

pub(crate) struct VZVirtioGraphicsScanout {
  never: Never,
}

pub(crate) fn vz_virtio_graphics_scanout_bridge_cast(object: &NSObject) -> Option<VZVirtioGraphicsScanout> {
  object.never()
}

impl VZVirtioGraphicsScanout {
  pub(crate) fn bridge_retained(&self) -> VZVirtioGraphicsScanout {
    match self.never {}
  }

  pub(crate) fn bridge_object(&self) -> &NSObject {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }
}

pub(crate) struct VZVirtioGraphicsScanoutConfiguration {
  never: Never,
}

pub(crate) fn vz_virtio_graphics_scanout_configuration_init(
  _width_in_pixels: isize,
  _height_in_pixels: isize,
) -> VZVirtioGraphicsScanoutConfiguration {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_virtio_graphics_scanout_configuration_bridge_cast(
  object: &NSObject,
) -> Option<VZVirtioGraphicsScanoutConfiguration> {
  object.never()
}

impl VZVirtioGraphicsScanoutConfiguration {
  pub(crate) fn bridge_retained(&self) -> VZVirtioGraphicsScanoutConfiguration {
    match self.never {}
  }

  pub(crate) fn bridge_object(&self) -> &NSObject {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_width_in_pixels(&self) -> isize {
    match self.never {}
  }

  pub(crate) fn vz_set_width_in_pixels(&self, _width_in_pixels: isize) {
    match self.never {}
  }

  pub(crate) fn vz_height_in_pixels(&self) -> isize {
    match self.never {}
  }

  pub(crate) fn vz_set_height_in_pixels(&self, _height_in_pixels: isize) {
    match self.never {}
  }
}

pub(crate) struct VZVirtioSoundDeviceConfiguration {
  never: Never,
}

pub(crate) fn vz_virtio_sound_device_configuration_init() -> VZVirtioSoundDeviceConfiguration {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_virtio_sound_device_configuration_bridge_cast(
  object: &NSObject,
) -> Option<VZVirtioSoundDeviceConfiguration> {
  object.never()
}

impl VZVirtioSoundDeviceConfiguration {
  pub(crate) fn bridge_retained(&self) -> VZVirtioSoundDeviceConfiguration {
    match self.never {}
  }

  pub(crate) fn bridge_object(&self) -> &NSObject {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_streams(&self) -> Vec<Retained<NSObject>> {
    match self.never {}
  }

  pub(crate) fn vz_set_streams(&self, _streams: &[&NSObject]) {
    match self.never {}
  }
}

pub(crate) struct VZVirtioSoundDeviceInputStreamConfiguration {
  never: Never,
}

pub(crate) fn vz_virtio_sound_device_input_stream_configuration_init() -> VZVirtioSoundDeviceInputStreamConfiguration {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_virtio_sound_device_input_stream_configuration_bridge_cast(
  object: &NSObject,
) -> Option<VZVirtioSoundDeviceInputStreamConfiguration> {
  object.never()
}

impl VZVirtioSoundDeviceInputStreamConfiguration {
  pub(crate) fn bridge_retained(&self) -> VZVirtioSoundDeviceInputStreamConfiguration {
    match self.never {}
  }

  pub(crate) fn bridge_object(&self) -> &NSObject {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_source(&self) -> Option<Retained<NSObject>> {
    match self.never {}
  }

  pub(crate) fn vz_set_source(&self, _source: Option<&NSObject>) {
    match self.never {}
  }
}

pub(crate) struct VZVirtioSoundDeviceOutputStreamConfiguration {
  never: Never,
}

pub(crate) fn vz_virtio_sound_device_output_stream_configuration_init() -> VZVirtioSoundDeviceOutputStreamConfiguration
{
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_virtio_sound_device_output_stream_configuration_bridge_cast(
  object: &NSObject,
) -> Option<VZVirtioSoundDeviceOutputStreamConfiguration> {
  object.never()
}

impl VZVirtioSoundDeviceOutputStreamConfiguration {
  pub(crate) fn bridge_retained(&self) -> VZVirtioSoundDeviceOutputStreamConfiguration {
    match self.never {}
  }

  pub(crate) fn bridge_object(&self) -> &NSObject {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_sink(&self) -> Option<Retained<NSObject>> {
    match self.never {}
  }

  pub(crate) fn vz_set_sink(&self, _sink: Option<&NSObject>) {
    match self.never {}
  }
}

pub(crate) struct VZMacGraphicsDevice {
  never: Never,
}

pub(crate) fn vz_mac_graphics_device_bridge_cast(object: &NSObject) -> Option<VZMacGraphicsDevice> {
  object.never()
}

impl VZMacGraphicsDevice {
  pub(crate) fn bridge_retained(&self) -> VZMacGraphicsDevice {
    match self.never {}
  }

  pub(crate) fn bridge_object(&self) -> &NSObject {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }
}

pub(crate) struct VZMacGraphicsDeviceConfiguration {
  never: Never,
}

pub(crate) fn vz_mac_graphics_device_configuration_init() -> VZMacGraphicsDeviceConfiguration {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_mac_graphics_device_configuration_bridge_cast(
  object: &NSObject,
) -> Option<VZMacGraphicsDeviceConfiguration> {
  object.never()
}

impl VZMacGraphicsDeviceConfiguration {
  pub(crate) fn bridge_retained(&self) -> VZMacGraphicsDeviceConfiguration {
    match self.never {}
  }

  pub(crate) fn bridge_object(&self) -> &NSObject {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_displays(&self) -> Vec<VZMacGraphicsDisplayConfiguration> {
    match self.never {}
  }

  pub(crate) fn vz_set_displays(&self, _displays: &[&VZMacGraphicsDisplayConfiguration]) {
    match self.never {}
  }
}

pub(crate) struct VZMacGraphicsDisplay {
  never: Never,
}

pub(crate) fn vz_mac_graphics_display_bridge_cast(object: &NSObject) -> Option<VZMacGraphicsDisplay> {
  object.never()
}

impl VZMacGraphicsDisplay {
  pub(crate) fn bridge_retained(&self) -> VZMacGraphicsDisplay {
    match self.never {}
  }

  pub(crate) fn bridge_object(&self) -> &NSObject {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_pixels_per_inch(&self) -> isize {
    match self.never {}
  }
}

pub(crate) struct VZMacGraphicsDisplayConfiguration {
  never: Never,
}

pub(crate) fn vz_mac_graphics_display_configuration_init(
  _width_in_pixels: isize,
  _height_in_pixels: isize,
  _pixels_per_inch: isize,
) -> VZMacGraphicsDisplayConfiguration {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_mac_graphics_display_configuration_init_for_size_in_points(
  _screen: &NSScreen,
  _size_in_points: BridgedCGSize,
) -> VZMacGraphicsDisplayConfiguration {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_mac_graphics_display_configuration_bridge_cast(
  object: &NSObject,
) -> Option<VZMacGraphicsDisplayConfiguration> {
  object.never()
}

impl VZMacGraphicsDisplayConfiguration {
  pub(crate) fn bridge_retained(&self) -> VZMacGraphicsDisplayConfiguration {
    match self.never {}
  }

  pub(crate) fn bridge_object(&self) -> &NSObject {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_width_in_pixels(&self) -> isize {
    match self.never {}
  }

  pub(crate) fn vz_set_width_in_pixels(&self, _width_in_pixels: isize) {
    match self.never {}
  }

  pub(crate) fn vz_height_in_pixels(&self) -> isize {
    match self.never {}
  }

  pub(crate) fn vz_set_height_in_pixels(&self, _height_in_pixels: isize) {
    match self.never {}
  }

  pub(crate) fn vz_pixels_per_inch(&self) -> isize {
    match self.never {}
  }

  pub(crate) fn vz_set_pixels_per_inch(&self, _pixels_per_inch: isize) {
    match self.never {}
  }
}

pub(crate) struct VZMacKeyboardConfiguration {
  never: Never,
}

pub(crate) fn vz_mac_keyboard_configuration_init() -> VZMacKeyboardConfiguration {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_mac_keyboard_configuration_bridge_cast(object: &NSObject) -> Option<VZMacKeyboardConfiguration> {
  object.never()
}

impl VZMacKeyboardConfiguration {
  pub(crate) fn bridge_retained(&self) -> VZMacKeyboardConfiguration {
    match self.never {}
  }

  pub(crate) fn bridge_object(&self) -> &NSObject {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }
}

pub(crate) struct VZMacTrackpadConfiguration {
  never: Never,
}

pub(crate) fn vz_mac_trackpad_configuration_init() -> VZMacTrackpadConfiguration {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_mac_trackpad_configuration_bridge_cast(object: &NSObject) -> Option<VZMacTrackpadConfiguration> {
  object.never()
}

impl VZMacTrackpadConfiguration {
  pub(crate) fn bridge_retained(&self) -> VZMacTrackpadConfiguration {
    match self.never {}
  }

  pub(crate) fn bridge_object(&self) -> &NSObject {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }
}
