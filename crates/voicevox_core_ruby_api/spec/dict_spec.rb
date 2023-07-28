# frozen_string_literal: true

RSpec.describe VoicevoxCore::UserDict do
  it "initializes" do
    expect(VoicevoxCore::UserDict.new).to be_a VoicevoxCore::UserDict
  end

  it "can add word" do
    dict = VoicevoxCore::UserDict.new
    word = VoicevoxCore::UserDict::Word.new("hoge", "ホゲ")
    expect { dict.add_word word }.not_to raise_error
  end
end
