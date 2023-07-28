# frozen_string_literal: true

module VoicevoxCore
  class VoicevoxError < StandardError
  end
  SupportedDevices = Struct.new(:cpu, :cuda, :dml, keyword_init: true)

  class UserDict
    Word = Struct.new(:surface, :pronunciation, :priority, :accent_type, :word_type, keyword_init: true) do
      def initialize(surface, pronunciation, priority: 5, accent_type: 0, word_type: :proper_noun)
        super(
          surface: surface,
          pronunciation: pronunciation,
          priority: priority,
          accent_type: accent_type,
          word_type: word_type
        )
      end
    end
  end
end
