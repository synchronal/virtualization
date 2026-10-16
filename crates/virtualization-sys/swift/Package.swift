// swift-tools-version: 6.2

import Foundation
import PackageDescription

// Absolute, because the flag reaches the compiler verbatim and its working
// directory is unspecified.
let bridgingHeader =
  "\(URL(fileURLWithPath: #filePath).deletingLastPathComponent().path)/Sources/VirtualizationBridge/bridging-header.h"

let package = Package(
  name: "VirtualizationBridge",
  platforms: [.macOS("27.0")],
  products: [
    .library(
      name: "VirtualizationBridge",
      type: .static,
      targets: ["VirtualizationBridge"]
    )
  ],
  targets: [
    .target(
      name: "VirtualizationBridge",
      swiftSettings: [.unsafeFlags(["-import-objc-header", bridgingHeader])],
      linkerSettings: [
        .linkedFramework("Virtualization"), .linkedFramework("DiskImageKit"), .linkedFramework("Security"),
        .linkedFramework("vmnet"), .linkedFramework("AccessoryAccess"),
      ]
    )
  ]
)
