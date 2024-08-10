#
# To learn more about a Podspec see http://guides.cocoapods.org/syntax/podspec.html.
# Run `pod lib lint app_device_integrity.podspec` to validate before publishing.
#
Pod::Spec.new do |s|
  s.name             = 'app_device_integrity'
  s.version          = '0.0.1'
  s.summary          = 'A Flutter plugin for app attestation using Apple App Attest.'
  s.description      = <<-DESC
A Flutter plugin that simplifies app attestation by using Apple's App Attest to generate tokens for server verification.
                       DESC
  s.homepage         = 'https://bubotech.club'
  s.license          = { :file => '../LICENSE' }
  s.author           = { 'Bubotech' => 'info@bubotech.club' }
  s.source           = { :path => '.' }
  s.source_files = 'Classes/**/*'
  s.dependency 'Flutter'
  s.platform = :ios, '14.0'
  s.osx.deployment_target = '11.0'

  # Flutter.framework does not contain a i386 slice.
  s.pod_target_xcconfig = { 'DEFINES_MODULE' => 'YES', 'EXCLUDED_ARCHS[sdk=iphonesimulator*]' => 'i386' }
  s.swift_version = '5.0'
end
