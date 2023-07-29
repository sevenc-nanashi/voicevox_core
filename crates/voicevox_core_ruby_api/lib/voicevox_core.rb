# frozen_string_literal: true

require "fiddle"
Fiddle.dlopen("#{__dir__}/voicevox_core_ruby_api/libonnxruntime.so.1.14.0")
require_relative "voicevox_core_ruby_api/model"
require_relative "voicevox_core_ruby_api/voicevox_core_ruby_api"

# 無料で使える中品質なテキスト読み上げソフトウェア、VOICEVOXのコア。
module VoicevoxCore
end
