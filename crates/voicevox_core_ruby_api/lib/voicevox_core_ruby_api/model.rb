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

  SpeakerMeta = Struct.new(:name, :styles, :speaker_uuid, :version)
  StyleMeta = Struct.new(:name, :id)

  AudioQuery = Struct.new(
    :accent_phrases,
    :speed_scale,
    :pitch_scale,
    :intonation_scale,
    :volume_scale,
    :pre_phoneme_length,
    :post_phoneme_length,
    :output_sampling_rate,
    :output_stereo,
    :kana
  )
  AccentPhrase = Struct.new(
    :moras,
    :accent,
    :pause_mora,
    :is_interrogative
  )
  Mora = Struct.new(
    :text,
    :consonant,
    :consonant_length,
    :vowel,
    :vowel_length,
    :pitch
  )
end
