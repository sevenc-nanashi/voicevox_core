// @ts-check
/// <reference path="./wasm_library_env.d.ts" />
/// <reference types="vite/client" />

addToLibrary({
  $onnxruntime_injection__postset: "onnxruntime_injection();",
  $onnxruntime_injection: function () {
    /** @type {typeof import("onnxruntime-web")} */
    let onnxruntime;
    import("onnxruntime-web/all").then((onnxruntime_) => {
      onnxruntime = onnxruntime_;
      console.log("onnxruntime-web loaded");
      console.log(onnxruntime_);
      onnxruntime_.env.wasm.wasmPaths = import.meta.env?.BASE_URL || "/";
    });

    let nonce = 0;
    const generateNonce = () => {
      return (nonce++).toString(16);
    };
    const toCharPtr = (/** @type {string} */ str) => {
      const bin = new TextEncoder().encode(str);
      const ptr = _malloc(bin.length + 1);
      HEAP8.set(bin, ptr);
      HEAP8[ptr + bin.length] = 0;
      return ptr;
    };

    /** @type {{[key: string]: import("onnxruntime-web").InferenceSession}} */
    const sessions = {};
    class Onnxruntime {
      newSession(
        /** @type {number} */ model,
        /** @type {number} */ modelLen,
        /** @type {number} */ useGpu,
        /** @type {number} */ callback,
      ) {
        const nonce = generateNonce();
        const modelDataRef = new Uint8Array(HEAPU8.buffer, model, modelLen);
        const modelData = modelDataRef.slice().buffer;
        (async () => {
          try {
            let session;
            if (useGpu) {
              console.log("onnxruntime session create with GPU");
              session = await onnxruntime.InferenceSession.create(modelData, {
                executionProviders: ["webnn", "wasm", "cpu"],
              }).catch((e) => {
                console.error("Failed to create session with GPU", e);
                console.error(e);
                return undefined;
              });
            }
            if (!session) {
              console.log("onnxruntime session create with WASM");
              session = await onnxruntime.InferenceSession.create(modelData, {
                executionProviders: ["wasm", "cpu"],
              });
            }
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
                }),
              ),
            ]);
          } catch (e) {
            const result = {
              type: "err",
              payload: String(e),
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
        /** @type {number} */ callback,
      ) {
        const session = sessions[UTF8ToString(sessionHandle)];
        const inputsObj =
          /** @type [name: string, tensor: {shape: number[], data: {kind: "int64" | "float64", array: number[]}}][] */ (
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
            const result = await session.run(
              Object.fromEntries(
                inputsObj.map(([name, { shape, data }]) => [
                  name,
                  new onnxruntime.Tensor(data.kind, data.array, shape),
                ]),
              ),
            );
            console.log("onnxruntime session run result");
            const tensors = await Promise.all(
              Object.values(result).map(async (tensor) => {
                const data = await tensor.getData();
                return {
                  shape: tensor.dims,
                  data: {
                    kind: tensor.type,
                    // @ts-expect-error string[]
                    array: Array.from(data),
                  },
                };
              }),
            );
            console.log(tensors);
            const resultStr = JSON.stringify({
              type: "ok",
              payload: tensors,
            });
            dynCall("vii", callback, [toCharPtr(nonce), toCharPtr(resultStr)]);
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

        console.log("sessionRun called", nonce);

        return toCharPtr(nonce);
      }
    }

    const inst = new Onnxruntime();
    // @ts-expect-error
    _onnxruntime_inference_session_new = inst.newSession.bind(inst);
    // @ts-expect-error
    _onnxruntime_inference_session_run = inst.sessionRun.bind(inst);
  },
  onnxruntime_inference_session_new: function () {},
  onnxruntime_inference_session_new__deps: ["$onnxruntime_injection"],
  onnxruntime_inference_session_run: function () {},
  onnxruntime_inference_session_run__deps: ["$onnxruntime_injection"],
});
