# frozen_string_literal: true

RSpec.describe VoicevoxCore::UserDict do
  let(:dict) { VoicevoxCore::UserDict.new }
  let(:word) { VoicevoxCore::UserDict::Word.new("hoge", "ホゲ") }
  let(:word2) { VoicevoxCore::UserDict::Word.new("fuga", "フガ") }

  it "initializes" do
    expect(VoicevoxCore::UserDict.new).to be_a VoicevoxCore::UserDict
  end

  it "can add word" do
    uuid = dict.add_word(word)
    expect(uuid).to be_a String
    expect(dict.get_word(uuid)).to eq word
  end

  it "can remove word" do
    uuid = dict.add_word(word)
    expect(dict.get_word(uuid)).to eq word
    dict.remove_word(uuid)
    expect(dict.get_word(uuid)).to be_nil
  end

  it "can update word" do
    uuid = dict.add_word(word)
    expect(dict.get_word(uuid)).to eq word
    dict.update_word(uuid, word2)
    expect(dict.get_word(uuid)).to eq word2
  end

  it "enumerates words" do
    uuid = dict.add_word(word)
    uuid2 = dict.add_word(word2)
    expect(dict.to_h).to eq({ uuid => word, uuid2 => word2 })
  end
end
