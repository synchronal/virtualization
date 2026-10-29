// swift-tools-version: 6.2

import PackageDescription

// Absolute, because the flag reaches the compiler verbatim and its working
// directory is unspecified.
let bridgingHeader = "\(Context.packageDirectory)/Sources/VmnetBridge/bridging-header.h"

let package = Package(
  name: "VmnetBridge",
  platforms: [.macOS("27.0")],
  products: [
    .library(
      name: "VmnetBridge",
      type: .static,
      targets: ["VmnetBridge"]
    )
  ],
  targets: [
    .target(
      name: "VmnetBridge",
      swiftSettings: [.unsafeFlags(["-import-objc-header", bridgingHeader])]
    )
  ]
)
