import raw, {
  Pointer,
  VoicevoxCore,
  VoicevoxResultCode,
} from "./artifacts/voicevox_core_wasm_api";
import JSZip from "jszip";

let _voicevoxCore: VoicevoxCore | undefined;
let resolvers: ((vvc: VoicevoxCore) => void)[] = [];
const voicevoxCore = (): Promise<VoicevoxCore> => {
  if (_voicevoxCore) {
    return Promise.resolve(_voicevoxCore);
  }
  return new Promise((resolve) => {
    resolvers.push(resolve);
  });
};
raw().then((vvc) => {
  vvc.ready.then(() => {
    console.log("VoicevoxCore is ready");
    console.log(vvc);
    _voicevoxCore = vvc;
    vvc.ccall(
      "setenv",
      "number",
      ["string", "string"],
      ["RUST_BACKTRACE", "full"]
    );

    vvc.FS.mkdir("/data");
    // @ts-ignore
    window.vvc = vvc;
    for (const resolve of resolvers) {
      resolve(vvc);
    }
  });
});

export async function getVersion() {
  const vvc = await voicevoxCore();
  return vvc.ccall("voicevox_get_version", "string", [], []);
}

function throwIfError(vvc: VoicevoxCore, code: VoicevoxResultCode) {
  if (code !== 0) {
    const message = vvc.ccall(
      "voicevox_error_result_to_message",
      "string",
      ["number"],
      [code]
    );
    throw new Error(`VoicevoxCore error: ${message}`);
  }
}

function allocPointer<T>(vvc: VoicevoxCore) {
  return vvc._malloc(4) as Pointer<T>;
}
function getPointerValue<T extends string>(
  vvc: VoicevoxCore,
  pointer: Pointer<`${T}*`>
) {
  return vvc.getValue(pointer, "i32") as Pointer<T>;
}
function utf8ToString(vvc: VoicevoxCore, pointer: Pointer<"string">) {
  return vvc.UTF8ToString(pointer);
}

function fileExists(vvc: VoicevoxCore, path: string) {
  try {
    vvc.FS.stat(path);
    return true;
  } catch (e) {
    return false;
  }
}

const openJtalkFinalizer = new FinalizationRegistry(
  (pointer: Pointer<"OpenJtalkRc">) => {
    const vvc = _voicevoxCore;
    if (vvc) {
      vvc.ccall("voicevox_open_jtalk_rc_delete", "void", ["number"], [pointer]);
    }
  }
);

export class OpenJtalkRc {
  static async new() {
    const vvc = await voicevoxCore();
    const zip = await JSZip.loadAsync(
      await fetch("/open_jtalk_dic.zip").then((res) => res.arrayBuffer())
    );
    if (!fileExists(vvc, "/data/open_jtalk_dic_utf_8-1.11/")) {
      for (const [name, data] of Object.entries(zip.files)) {
        console.log("Extracting", name);
        if (name.endsWith("/")) {
          vvc.FS.mkdir(`/data/${name}`);
        } else {
          vvc.FS.writeFile(`/data/${name}`, await data.async("uint8array"), {
            flags: "w",
          });
        }
      }
    }
    const returnPtr = allocPointer<"OpenJtalkRc*">(vvc);
    throwIfError(
      vvc,
      vvc.ccall(
        "voicevox_open_jtalk_rc_new",
        "number",
        ["string", "number"],
        ["/data/open_jtalk_dic_utf_8-1.11", returnPtr]
      )
    );
    const pointer = vvc.getValue(returnPtr, "i32") as Pointer<"OpenJtalkRc">;

    const openJtalkRc = new OpenJtalkRc(pointer);
    console.log("Initialized OpenJtalkRc", openJtalkRc);

    openJtalkFinalizer.register(openJtalkRc, pointer);

    return openJtalkRc;
  }
  constructor(public _pointer: Pointer<"OpenJtalkRc">) {}
}

const synthesizerFinalizer = new FinalizationRegistry(
  (pointer: Pointer<"VoicevoxSynthesizer">) => {
    const vvc = _voicevoxCore;
    if (vvc) {
      console.log("Deleting synthesizer", pointer);
      vvc.ccall("voicevox_synthesizer_delete", "void", ["number"], [pointer]);
    }
  }
);

export class Synthesizer {
  static async new(openJtalkRc: OpenJtalkRc) {
    const vvc = await voicevoxCore();
    const accelerationModePtr = allocPointer<"AccelerationMode">(vvc);
    const cpuNumThreadsPtr = allocPointer<"i32">(vvc);

    vvc.ccall(
      "voicevox_make_default_initialize_options_wasm",
      "void",
      ["number", "number"],
      [accelerationModePtr, cpuNumThreadsPtr]
    );
    // const accelerationMode = vvc.getValue(accelerationModePtr, "i32");
    const accelerationMode = 1
    const cpuNumThreads = vvc.getValue(cpuNumThreadsPtr, "i32");

    const returnPtr = allocPointer<"VoicevoxSynthesizer*">(vvc);
    throwIfError(
      vvc,
      vvc.ccall(
        "voicevox_synthesizer_new_wasm",
        "number",
        ["number", "number", "number", "number"],
        [openJtalkRc._pointer, accelerationMode, cpuNumThreads, returnPtr]
      )
    );

    const returnPtrValue = getPointerValue(vvc, returnPtr);
    const synthesizer = new Synthesizer(openJtalkRc, returnPtrValue);
    console.log("Initialized Synthesizer", synthesizer);

    synthesizerFinalizer.register(synthesizer, returnPtrValue);

    return synthesizer;
  }
  constructor(
    private openJtalkRc: OpenJtalkRc,
    public _pointer: Pointer<"VoicevoxSynthesizer">
  ) {}

  async loadVoiceModel(model: VoiceModel) {
    const vvc = await voicevoxCore();
    throwIfError(
      vvc,
      await vvc.ccall(
        "voicevox_synthesizer_load_voice_model",
        "number",
        ["number", "number"],
        [this._pointer, model._pointer]
      )
    );
  }

  async tts(text: string, speaker: number) {
    const vvc = await voicevoxCore();
    const enableInterrogativeUpspeakPtr = allocPointer<"boolean">(vvc);
    vvc.ccall(
      "voicevox_make_default_tts_options_wasm",
      "void",
      ["number"],
      [enableInterrogativeUpspeakPtr]
    );

    const enableInterrogativeUpspeak =
      vvc.getValue(enableInterrogativeUpspeakPtr, "i8") !== 0;

    const outputWavLengthPtr = allocPointer<"i32">(vvc);
    const outputWavPtrPtr = allocPointer<"u8*">(vvc);

    throwIfError(
      vvc,
      await vvc.ccall(
        "voicevox_synthesizer_tts",
        "number",
        ["number", "string", "number", "boolean", "number", "number"],
        [
          this._pointer,
          text,
          speaker,
          enableInterrogativeUpspeak,
          outputWavLengthPtr,
          outputWavPtrPtr,
        ]
      )
    );

    const outputWavLength = vvc.getValue(outputWavLengthPtr, "i32");
    const outputWavPtr = getPointerValue(vvc, outputWavPtrPtr);

    const outputWavRef = new Uint8Array(
      vvc.HEAPU8.buffer,
      outputWavPtr,
      outputWavLength
    );
    const outputWav = outputWavRef.slice();
    vvc.ccall("voicevox_wav_free", "void", ["number"], [outputWavPtr]);

    return outputWav;
  }

  async metas() {
    const vvc = await voicevoxCore();
    const returnPtr = vvc.ccall(
      "voicevox_synthesizer_create_metas_json",
      "number",
      ["number"],
      [this._pointer]
    );
    const metas = utf8ToString(vvc, returnPtr);
    vvc.ccall("voicevox_json_free", "void", ["number"], [returnPtr]);
    return JSON.parse(metas);
  }
}
const voiceModelFinalizer = new FinalizationRegistry(
  (pointer: Pointer<"VoicevoxVoiceModel">) => {
    const vvc = _voicevoxCore;
    if (vvc) {
      console.log("Deleting voice model", pointer);
      vvc.ccall("voicevox_voice_model_delete", "void", ["number"], [pointer]);
    }
  }
);
export class VoiceModel {
  static async newFromPath(model: Uint8Array) {
    const vvc = await voicevoxCore();
    const nonce = Math.floor(Math.random() * 1000000);
    vvc.FS.writeFile(`/data/voice_model_${nonce}.vvm`, model, { flags: "w" });
    const returnPtr = allocPointer<"VoicevoxVoiceModel*">(vvc);
    throwIfError(
      vvc,
      vvc.ccall(
        "voicevox_voice_model_new_from_path",
        "number",
        ["string", "number"],
        [`/data/voice_model_${nonce}.vvm`, returnPtr]
      )
    );
    const pointer = getPointerValue(vvc, returnPtr);
    const voiceModel = new VoiceModel(pointer);
    console.log("Initialized VoiceModel", voiceModel);
    voiceModelFinalizer.register(voiceModel, pointer);
    return voiceModel;
  }
  constructor(public _pointer: Pointer<"VoicevoxVoiceModel">) {}

  async metas() {
    const vvc = await voicevoxCore();
    const returnPtr = vvc.ccall(
      "voicevox_voice_model_get_metas_json",
      "number",
      ["number"],
      [this._pointer]
    );
    const metas = utf8ToString(vvc, returnPtr);
    return JSON.parse(metas);
  }
}
