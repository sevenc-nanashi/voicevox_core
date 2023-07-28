# frozen_string_literal: true

module VoicevoxCore
  class VoicevoxError < StandardError
  end
  SupportedDevices = Struct.new(:cpu, :cuda, :dml, keyword_init: true)

  UserDictWord = Struct.new(:surface, :pronunciation, :priority, :accent_type, :word_type, keyword_init: true) do
    def initialize(word:, pronunciation:, priority: 0, accent_type: 0, word_type: 0)
      super
    end
  end
end
