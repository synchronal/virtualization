// swift-tools-version: 6.2

import PackageDescription

// Absolute, because the flag reaches the compiler verbatim and its working
// directory is unspecified.
let bridgingHeader = "\(Context.packageDirectory)/Sources/IOUSBHostBridge/bridging-header.h"

let package = Package(
  name: "IOUSBHostBridge",
  platforms: [.macOS("27.0")],
  products: [
    .library(
      name: "IOUSBHostBridge",
      type: .static,
      targets: ["IOUSBHostBridge"]
    )
  ],
  targets: [
    .target(
      name: "IOUSBHostBridge",
      swiftSettings: [.unsafeFlags(["-import-objc-header", bridgingHeader])]
    )
  ]
)
