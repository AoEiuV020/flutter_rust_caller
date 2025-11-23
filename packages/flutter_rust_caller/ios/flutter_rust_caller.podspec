#
# To learn more about a Podspec see http://guides.cocoapods.org/syntax/podspec.html.
# Run `pod lib lint flutter_rust_caller.podspec` to validate before publishing.
#
Pod::Spec.new do |s|
  s.name             = 'flutter_rust_caller'
  s.version          = '0.0.1'
  s.summary          = 'A Rust FFI plugin project.'
  s.description      = <<-DESC
A Flutter FFI plugin project using prebuilt Rust libraries.
                       DESC
  s.homepage         = 'http://example.com'
  s.license          = { :file => '../LICENSE' }
  s.author           = { 'Your Company' => 'email@example.com' }

  s.source           = { :path => '.' }
  s.source_files = 'Classes/**/*'
  s.dependency 'Flutter'
  s.platform = :ios, '13.0'

  s.script_phase = {
    :name => 'update rust library',
    :script => 'touch ${BUILT_PRODUCTS_DIR}/prebuild.touch',
    :execution_position=> :before_compile,
    :input_files => ['${PODS_TARGET_SRCROOT}/../prebuild/${PLATFORM_FAMILY_NAME}/'],
    :output_files => ["${BUILT_PRODUCTS_DIR}/prebuild.touch"],
  }
  s.pod_target_xcconfig = {
    'DEFINES_MODULE' => 'YES',
    # Flutter.framework does not contain a i386 slice.
    'EXCLUDED_ARCHS[sdk=iphonesimulator*]' => 'i386',
    # We use `-force_load` instead of `-l` since Xcode strips out unused symbols from static libraries.
    'OTHER_LDFLAGS' => "-force_load ${PODS_TARGET_SRCROOT}/../prebuild/${PLATFORM_FAMILY_NAME}/${PLATFORM_NAME}/${CURRENT_ARCH}/libflutter_rust_caller.a",
  }
  s.swift_version = '5.0'
end
