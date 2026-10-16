// swift-tools-version: 6.2

import Foundation
import PackageDescription

// Absolute, because the flag reaches the compiler verbatim and its working
// directory is unspecified.
let bridgingHeader =
  "\(URL(fileURLWithPath: #filePath).deletingLastPathComponent().path)/Sources/AccessoryAccessBridge/bridging-header.h"

let package = Package(
  name: "AccessoryAccessBridge",
  platforms: [.macOS("27.0")],
  products: [
    .library(
      name: "AccessoryAccessBridge",
      type: .static,
      targets: ["AccessoryAccessBridge"]
    )
  ],
  targets: [
    .target(
      name: "AccessoryAccessBridge",
      swiftSettings: [.unsafeFlags(["-import-objc-header", bridgingHeader])],
      linkerSettings: [
        .linkedFramework("AccessoryAccess"),
        .linkedFramework("IOUSBHost"),
      ]
    )
  ]
)
