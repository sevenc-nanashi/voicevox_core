addToLibrary({
  $onnxruntime_injection__postset: "onnxruntime_injection();",
  $onnxruntime_injection: function () {
    let onnxruntime;
    import("onnxruntime-web").then((onnxruntime_) => {
      onnxruntime = onnxruntime_;
      console.log("onnxruntime-web loaded");
      console.log(onnxruntime_);
      onnxruntime_.env.wasm.wasmPaths = "/node_modules/onnxruntime-web/dist/";
    });

    let nonce = 0;
    const generateNonce = () => {
      return (nonce++).toString(16);
    };
    const toCharPtr = (str) => {
      const bin = new TextEncoder().encode(str);
      const ptr = _malloc(bin.length + 1);
      HEAP8.set(bin, ptr);
      HEAP8[ptr + bin.length] = 0;
      return ptr;
    };

    const sessions = {};
    class Onnxruntime {
      newSession(
        /** @type {number} */ model,
        /** @type {number} */ modelLen,
        /** @type {number} */ useGpu,
        /** @type {number} */ callback
      ) {
        const nonce = generateNonce();
        const modelDataRef = new Uint8Array(HEAPU8.buffer, model, modelLen);
        const modelData = modelDataRef.slice().buffer;
        (async () => {
          try {
            const session = await onnxruntime.InferenceSession.create(
              modelData,
              {
                executionProviders: useGpu
                  ? ["webgpu", "webgl", "wasm"]
                  : ["wasm"],
              }
            );
            sessions[nonce] = session;
            console.log("onnxruntime session created");
            console.log(session);
            const result = {
              handle: nonce,
            };

            dynCall("vii", callback, [
              toCharPtr(nonce),
              toCharPtr(
                JSON.stringify({
                  type: "ok",
                  payload: result,
                })
              ),
            ]);
          } catch (e) {
            const result = {
              type: "err",
              payload: e.message,
            };
            dynCall("vii", callback, [
              toCharPtr(nonce),
              toCharPtr(JSON.stringify(result)),
            ]);
          }
        })();

        console.log("newSession called", nonce);

        return toCharPtr(nonce);
      }

      sessionRun(
        /** @type {number} */ sessionHandle,
        /** @type {number} */ inputs,
        /** @type {number} */ callback
      ) {
        const session = sessions[UTF8ToString(sessionHandle)];
        const inputsObj =
          /** @type {{shape: number[], data: {kind: string, array: number[]}}[] */ (
            JSON.parse(UTF8ToString(inputs))
          );
        const nonce = generateNonce();

        (async () => {
          try {
            console.log("onnxruntime session run");
            if (!session) {
              throw new Error("session not found");
            }
            console.log(inputsObj);
            const result =
              /** @type {{[key: string]: {cpuData: {[key: number]: number}, dims: number[], type: string}}} */ (
                await session.run(
                  Object.fromEntries(
                    inputsObj.map((input, i) => [
                      session.inputNames[i],
                      new onnxruntime.Tensor(
                        input.data.kind,
                        input.data.array,
                        input.shape
                      ),
                    ])
                  )
                )
              );
            console.log("onnxruntime session run result");
            console.log(result);
            const resultStr = JSON.stringify({
              type: "ok",
              payload: Object.values(result).map((tensor) => ({
                shape: tensor.dims,
                data: {
                  kind: tensor.type,
                  array: Object.entries(tensor.cpuData)
                    .sort(([a], [b]) => a - b)
                    .map(([, value]) => value),
                },
              })),
            });
            dynCall("vii", callback, [toCharPtr(nonce), toCharPtr(resultStr)]);
          } catch (e) {
            const result = {
              type: "err",
              payload: e.message,
            };
            dynCall("vii", callback, [
              toCharPtr(""),
              toCharPtr(JSON.stringify(result)),
            ]);
          }
        })();

        console.log("sessionRun called", nonce);

        return toCharPtr(nonce);
      }
    }

    const inst = new Onnxruntime();
    _onnxruntime_inference_session_new = inst.newSession.bind(inst);
    _onnxruntime_inference_session_run = inst.sessionRun.bind(inst);
  },
  onnxruntime_inference_session_new: function () {},
  onnxruntime_inference_session_new__deps: ["$onnxruntime_injection"],
  onnxruntime_inference_session_run: function () {},
  onnxruntime_inference_session_run__deps: ["$onnxruntime_injection"],

  emscripten_memcpy_js: (dest, src, num) =>
    HEAPU8.copyWithin(dest, src, src + num),
});
