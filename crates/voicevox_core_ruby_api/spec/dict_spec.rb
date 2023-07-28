# frozen_string_literal: true

RSpec.describe VoicevoxCore::UserDict do
  it "initializes" do
    expect(VoicevoxCore::UserDict.new).to be_a VoicevoxCore::UserDict
  end

  it "can add entries" do
    pending
    dict = VoicevoxCore::UserDict.new
    dict.add_entry("foo", "bar")
    expect(dict.entries).to eq [%w[foo bar]]
  end
end
