# frozen_string_literal: true

require "tomlrb"

cargo_toml = Tomlrb.load_file("#{__dir__}/../../Cargo.toml", symbolize_keys: true)

Gem::Specification.new do |spec|
  spec.name = "voicevox_core"
  spec.version = cargo_toml[:workspace][:package][:version]
  spec.authors = ["Hiroshiba"]
  spec.email = ["hihokaruta@gmail.com"]

  spec.summary = "無料で使える中品質なテキスト読み上げソフトウェア、VOICEVOXのコア。"
  spec.homepage = "https://github.com/voicevox/voicevox_core"
  spec.required_ruby_version = ">= 3.0"

  spec.metadata["homepage_uri"] = spec.homepage

  # Specify which files should be added to the gem when it is released.
  # The `git ls-files -z` loads the files in the RubyGem that have been added into git.
  spec.files = Dir.chdir(__dir__) do
    `git ls-files -z`.split("\x0").reject do |f|
      (File.expand_path(f) == __FILE__) ||
        f.start_with?(*%w[bin/ test/ spec/ features/ .git .circleci appveyor Gemfile])
    end
  end
  spec.bindir = "exe"
  spec.executables = spec.files.grep(%r{\Aexe/}) { |f| File.basename(f) }
  spec.require_paths = ["lib"]
  spec.extensions = ["ext/voicevox_core/Cargo.toml"]

  # Uncomment to register a new dependency of your gem
  spec.add_dependency "fiddle", "~> 1.1", ">= 1.1.1"

  # For more information and examples about making a new gem, check out our
  # guide at: https://bundler.io/guides/creating_gem.html
end
