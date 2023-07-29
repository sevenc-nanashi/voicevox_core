# frozen_string_literal: true

module VoicevoxCore
  class VoicevoxError < StandardError
  end
  SupportedDevices = Struct.new(:cpu, :cuda, :dml, keyword_init: true)

  class UserDict
    Word = Struct.new(:surface, :pronunciation, :priority, :accent_type, :word_type, keyword_init: true) do
      def initialize(surface, pronunciation,
                     # Rustから呼び出せるようにするため、位置引数でも受け取れるようにする
                     priority_ = nil, accent_type_ = nil, word_type_ = nil,
                     priority: 5, accent_type: 0, word_type: :proper_noun)
        VoicevoxCore._validate_pronunciation(pronunciation)
        super(
          surface: VoicevoxCore._to_zenkaku(surface),
          pronunciation: pronunciation,
          priority: priority_ || priority,
          accent_type: accent_type_ || accent_type,
          word_type: word_type_ || word_type
        )
      end
    end
  end
end
