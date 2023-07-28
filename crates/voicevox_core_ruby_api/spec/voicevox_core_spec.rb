# frozen_string_literal: true

RSpec.describe VoicevoxCore do
  it "has a version number" do
    expect(VoicevoxCore::VERSION).to be_a String
  end

  specify "VoicevoxCore.supported_devices.cpu is always true" do
    supported_devices = VoicevoxCore.supported_devices
    expect(supported_devices.cpu).to be true
  end
end
