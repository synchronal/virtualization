//===----------------------------------------------------------------------===//
// `VZGraphicsDisplay`, abstract: its object crosses by address. Its
// observers, `VZGraphicsDisplayObserver`, are Rust's.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_graphics_display_size_in_pixels(this: UnsafeRawPointer) -> BridgedCGSize {
  BridgedCGSize((bridging(borrowed: this) as VZGraphicsDisplay).sizeInPixels)
}

func vz_graphics_display_reconfigure_size_in_pixels(this: UnsafeRawPointer, size_in_pixels: BridgedCGSize)
  throws(BridgedThrown)
{
  let display: VZGraphicsDisplay = bridging(borrowed: this)

  try bridging {
    try display.reconfigure(sizeInPixels: CGSize(size_in_pixels))
  }
}

/// `configuration` is any `VZGraphicsDisplayConfiguration`.
func vz_graphics_display_reconfigure_configuration(this: UnsafeRawPointer, configuration: UnsafeRawPointer)
  throws(BridgedThrown)
{
  let display: VZGraphicsDisplay = bridging(borrowed: this)

  try bridging {
    try display.reconfigure(configuration: bridging(borrowed: configuration))
  }
}

/// `key` names Rust's observer (see `bridging(addingObserver:key:to:)`).
func vz_graphics_display_add_observer(this: UnsafeRawPointer, observer: VZGraphicsDisplayObserverBridge, key: UInt) {
  let display: VZGraphicsDisplay = bridging(borrowed: this)

  display.addObserver(bridging(addingObserver: observer, key: key, to: display))
}

/// `key` names Rust's observer (see `bridging(addingObserver:key:to:)`).
func vz_graphics_display_remove_observer(
  this: UnsafeRawPointer,
  observer: VZGraphicsDisplayObserverBridge,
  key: UInt
) {
  let display: VZGraphicsDisplay = bridging(borrowed: this)

  display.removeObserver(bridging(removingObserver: observer, key: key, from: display))
}

/// A `VZGraphicsDisplayObserver` that forwards to Rust's.
final class BridgedGraphicsDisplayObserver: NSObject, VZGraphicsDisplayObserver {
  let observer: VZGraphicsDisplayObserverBridge

  init(_ observer: VZGraphicsDisplayObserverBridge) {
    self.observer = observer
  }

  /// `display` crosses as a pointer, retained for Rust.
  func displayDidBeginReconfiguration(_ display: VZGraphicsDisplay) {
    observer.display_did_begin_reconfiguration(bridging(retained: display))
  }

  /// `display` crosses as a pointer, retained for Rust.
  func displayDidEndReconfiguration(_ display: VZGraphicsDisplay) {
    observer.display_did_end_reconfiguration(bridging(retained: display))
  }
}

/// Each Rust observer's `BridgedGraphicsDisplayObserver` on a display, by its
/// key: a display knows an observer by its object, so adding and removing one
/// Rust observer must pass the same one. A display holds observers weakly, so
/// each is kept from adding until removed: once, however often added, as the
/// display removes it.
final class BridgedGraphicsDisplayObservers {
  var observers: [UInt: BridgedGraphicsDisplayObserver] = [:]
}

/// Where a display keeps its `BridgedGraphicsDisplayObservers`. Only its
/// address is used.
nonisolated(unsafe) private var observersKey: UInt8 = 0

/// The observers `display` keeps, made on first use.
private func bridging(observersOf display: VZGraphicsDisplay) -> BridgedGraphicsDisplayObservers {
  if let observers = objc_getAssociatedObject(display, &observersKey) as? BridgedGraphicsDisplayObservers {
    return observers
  }

  let observers = BridgedGraphicsDisplayObservers()

  objc_setAssociatedObject(display, &observersKey, observers, .OBJC_ASSOCIATION_RETAIN)
  return observers
}

/// The `BridgedGraphicsDisplayObserver` for the Rust observer `key` names,
/// kept by `display` for adding.
func bridging(addingObserver observer: VZGraphicsDisplayObserverBridge, key: UInt, to display: VZGraphicsDisplay)
  -> BridgedGraphicsDisplayObserver
{
  let observers = bridging(observersOf: display)

  if let bridged = observers.observers[key] {
    return bridged
  }

  let bridged = BridgedGraphicsDisplayObserver(observer)

  observers.observers[key] = bridged
  return bridged
}

/// The `BridgedGraphicsDisplayObserver` for the Rust observer `key` names, no
/// longer kept by `display`, for removing: the one added, or a new one for an
/// observer never added, as the display would be given in Swift.
func bridging(
  removingObserver observer: VZGraphicsDisplayObserverBridge,
  key: UInt,
  from display: VZGraphicsDisplay
) -> BridgedGraphicsDisplayObserver {
  bridging(observersOf: display).observers.removeValue(forKey: key) ?? BridgedGraphicsDisplayObserver(observer)
}
