// swift-tools-version: 6.2

import PackageDescription

// Absolute, because the flag reaches the compiler verbatim and its working
// directory is unspecified.
let bridgingHeader = "\(Context.packageDirectory)/Sources/AccessoryAccessBridge/bridging-header.h"

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
      swiftSettings: [.unsafeFlags(["-import-objc-header", bridgingHeader])]
    )
  ]
)
