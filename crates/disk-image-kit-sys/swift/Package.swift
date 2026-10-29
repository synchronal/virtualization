// swift-tools-version: 6.2

import PackageDescription

// Absolute, because the flag reaches the compiler verbatim and its working
// directory is unspecified.
let bridgingHeader = "\(Context.packageDirectory)/Sources/DiskImageKitBridge/bridging-header.h"

let package = Package(
  name: "DiskImageKitBridge",
  platforms: [.macOS("27.0")],
  products: [
    .library(
      name: "DiskImageKitBridge",
      type: .static,
      targets: ["DiskImageKitBridge"]
    )
  ],
  targets: [
    .target(
      name: "DiskImageKitBridge",
      swiftSettings: [.unsafeFlags(["-import-objc-header", bridgingHeader])]
    )
  ]
)
