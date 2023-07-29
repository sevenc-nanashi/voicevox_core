# frozen_string_literal: true

require "tmpdir"

RSpec.describe VoicevoxCore::UserDict do
  example "ユーザー辞書の操作" do
    dict = VoicevoxCore::UserDict.new

    # 単語の追加
    word = VoicevoxCore::UserDict::Word.new("hoge", "ホゲ")
    uuid = dict.add_word(word)
    expect(dict[uuid]).to eq(word)

    # 単語の更新
    word2 = VoicevoxCore::UserDict::Word.new("fuga", "フガ")
    dict.update_word(uuid, word2)
    expect(dict[uuid]).to eq(word2)

    # 辞書のインポート
    dict2 = VoicevoxCore::UserDict.new
    word3 = VoicevoxCore::UserDict::Word.new("piyo", "ピヨ")
    uuid2 = dict2.add_word(word3)

    dict.import(dict2)
    expect(dict[uuid2]).to eq(word3)

    # 辞書のエクスポート
    begin
      temp_path = Dir.tmpdir + "/voicevox_core_test_temp_#{
        Time.now.to_i
      }.json"

      dict.save(temp_path)
      dict3 = VoicevoxCore::UserDict.new
      dict3.load(temp_path)
      expect(dict3[uuid2]).to eq(word3)
    ensure
      File.delete(temp_path)
    end

    # 単語のバリデーション
    expect { VoicevoxCore::UserDict::Word.new("hoge", "漢字") }.to raise_error(VoicevoxCore::VoicevoxError)
  end
end
