base = File.read("./crates/voicevox_core_wasm_api/__gi_test_web/public/voicevox_core_wasm_api.js")

functions = {
  "_emscripten_memcpy_js" => "function _emscripten_memcpy_js(dest, src, num) { return HEAPU8.set(HEAPU8.subarray(src, src + num), dest); }",
  "__emscripten_get_now" => "function __emscripten_get_now() { return performance.now(); }",
  "___cxx_global_var_init" => "function ___cxx_global_var_init() { }",
}

functions.each do |name, body|
  unless base.gsub!(/function #{name}\(\s*\).+#{name}\.stub = true;/m, body)
    warn "function #{name} not found"
  end
end

File.write("./crates/voicevox_core_wasm_api/__gi_test_web/public/voicevox_core_wasm_api.js", base)
