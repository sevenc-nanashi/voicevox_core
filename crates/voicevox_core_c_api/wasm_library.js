addToLibrary({
  __cxx_global_var_init: function () {/* noop */},

  emscripten_memcpy_js: function (dest, src, num) {
    return HEAPU8.set(HEAPU8.subarray(src, src + num), dest);
  },
});
