#
# To learn more about a Podspec see http://guides.cocoapods.org/syntax/podspec.html.
# Run `pod lib lint app_device_integrity.podspec` to validate before publishing.
#
Pod::Spec.new do |s|
    s.name             = 'app_device_integrity'
    s.version          = '0.0.1'
    s.summary          = 'A Flutter plugin for app attestation using Apple App Attest on macOS.'
    s.description      = <<-DESC
  A Flutter plugin that simplifies app attestation by using Apple's App Attest to generate tokens for server verification on macOS.
                         DESC
    s.homepage         = 'https://bubotech.club'
    s.license          = { :file => '../LICENSE' }
    s.author           = { 'Bubotech' => 'info@bubotech.club' }
    s.source           = { :path => '.' }
    s.source_files = 'Classes/**/*'
    s.dependency 'FlutterMacOS'
    s.platform = :osx, '11.0'
  
    s.pod_target_xcconfig = { 'DEFINES_MODULE' => 'YES' }
    s.swift_version = '5.0'
end
