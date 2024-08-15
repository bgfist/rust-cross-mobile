// swift-tools-version:5.5
// The swift-tools-version declares the minimum version of Swift required to build this package.
// Swift Package: JkCoreQuestion

import PackageDescription;

let package = Package(
    name: "JkCoreQuestion",
    platforms: [
        .iOS(.v13),
        .macOS(.v10_15)
    ],
    products: [
        .library(
            name: "JkCoreQuestion",
            targets: ["JkCoreQuestion"]
        )
    ],
    dependencies: [ 
        
    ],
    targets: [
        .binaryTarget(
            name: "RustFramework",
            path: "./RustFramework.xcframework"
        ),
        .target(
            name: "JkCoreQuestion",
            dependencies: [
                .target(name: "RustFramework")
            ],
            linkerSettings: [
                .linkedLibrary("sqlite3"), // 添加链接器标志以链接到 sqlite3
            ]
        ),
    ]
)
