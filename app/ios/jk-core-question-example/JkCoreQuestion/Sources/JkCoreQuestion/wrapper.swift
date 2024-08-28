import UIKit

public func testNativeWrapper(ctx: UIViewController) {
    let count = CFGetRetainCount(ctx)
    print("Retain count: \(count)")
    let ptr = Unmanaged.passUnretained(ctx).toOpaque()
    return testNative(ctx: Handle.init(unsafeFromRawPointer: ptr))
}
