//===----------------------------------------------------------------------===//
// `VZVirtioSharedMemoryRegion`, and the pages Rust maps into one, which it
// holds until they're unmapped.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

/// An element of an array of them, which Rust holds.
func vz_virtio_shared_memory_region_bridge_from(object: UnsafeRawPointer) -> VZVirtioSharedMemoryRegion {
  bridging(borrowed: object)
}

/// `memory` is host memory the caller keeps until it's unmapped.
func vz_virtio_shared_memory_region_map_memory(
  this: VZVirtioSharedMemoryRegion,
  memory: UnsafeMutableRawPointer,
  offset: UInt64,
  size: UInt64,
  completion_handler: @escaping (UInt) -> Void
) {
  let completionHandler = Bridged(value: completion_handler)

  this.mapMemory(memory, atOffset: offset, size: size) { completionHandler.value(bridging(retainedError: $0)) }
}

/// `mapMemory` of all of `memory`, which the region holds until mapping it
/// fails or an unmap covering it completes.
func vz_virtio_shared_memory_region_map_memory_pages(
  this: VZVirtioSharedMemoryRegion,
  memory: BridgedMemoryPages,
  offset: UInt64,
  completion_handler: @escaping (UInt) -> Void
) {
  let completionHandler = Bridged(value: completion_handler)
  let size = UInt64(memory.bridge_length())
  let address = UnsafeMutableRawPointer(bitPattern: memory.bridge_address())!
  let mapped = MappedMemoryPages.of(this)

  mapped.keep(memory, offset: offset, size: size)
  this.mapMemory(address, atOffset: offset, size: size) { error in
    // Only these pages: others in the range may still be mapped there.
    if error != nil {
      mapped.release(memory)
    }

    completionHandler.value(bridging(retainedError: error))
  }
}

/// `unmapMemory`, then releasing the pages it covers.
func vz_virtio_shared_memory_region_unmap_memory(
  this: VZVirtioSharedMemoryRegion,
  offset: UInt64,
  size: UInt64,
  completion_handler: @escaping (UInt) -> Void
) {
  let completionHandler = Bridged(value: completion_handler)
  let mapped = MappedMemoryPages.of(this)

  this.unmapMemory(atOffset: offset, size: size) { error in
    if error == nil {
      mapped.releaseUnmapped(offset: offset, size: size)
    }

    completionHandler.value(bridging(retainedError: error))
  }
}

extension VZVirtioSharedMemoryRegion {
  func vz_region_id() -> UInt8 {
    regionID
  }

  func vz_size() -> UInt64 {
    size
  }
}

/// Where a region keeps its `MappedMemoryPages`. Only its address is used.
nonisolated(unsafe) private var mappedKey: UInt8 = 0

/// The pages Rust mapped into a region, each by the offset and size it was
/// mapped at. Completion handlers run on the device's queue, maybe not the
/// caller's thread, so a lock guards them.
private final class MappedMemoryPages: NSObject, @unchecked Sendable {
  private let lock = NSLock()
  private var pages: [(offset: UInt64, size: UInt64, memory: BridgedMemoryPages)] = []

  /// The region's, made the first time it's asked for.
  static func of(_ region: VZVirtioSharedMemoryRegion) -> MappedMemoryPages {
    objc_sync_enter(region)
    defer { objc_sync_exit(region) }

    if let mapped = objc_getAssociatedObject(region, &mappedKey) as? MappedMemoryPages {
      return mapped
    }

    let mapped = MappedMemoryPages()
    objc_setAssociatedObject(region, &mappedKey, mapped, .OBJC_ASSOCIATION_RETAIN)
    return mapped
  }

  func keep(_ memory: BridgedMemoryPages, offset: UInt64, size: UInt64) {
    lock.withLock { pages.append((offset, size, memory)) }
  }

  /// Drops `memory`, kept for a map that failed.
  func release(_ memory: BridgedMemoryPages) {
    lock.withLock { pages.removeAll { $0.memory === memory } }
  }

  /// Drops the pages mapped wholly within `offset..<offset + size`, once
  /// unmapped.
  func releaseUnmapped(offset: UInt64, size: UInt64) {
    lock.withLock { pages.removeAll { $0.offset >= offset && $0.offset + $0.size <= offset + size } }
  }
}
