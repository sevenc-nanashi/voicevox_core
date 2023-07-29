# frozen_string_literal: true

RSpec.describe VoicevoxCore::UserDict do
  it "initializes" do
    expect(VoicevoxCore::UserDict.new).to be_a VoicevoxCore::UserDict
  end

  it "can add word" do
    dict = VoicevoxCore::UserDict.new
    word = VoicevoxCore::UserDict::Word.new("hoge", "ホゲ")
    uuid = dict.add_word(word)
    expect(uuid).to be_a String
    expect(dict.get_word(uuid)).to eq word
  end
end
