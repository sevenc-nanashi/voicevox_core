# frozen_string_literal: true

require "tmpdir"
require "async"

RSpec.describe VoicevoxCore::VoiceModel do
  example "VoiceModelのロード" do
    Async do
      model = VoicevoxCore::VoiceModel.from_path("#{__dir__}/../../../model/sample.vvm")
      expect(model.name).to eq("sample")
    end.wait
  end
end
