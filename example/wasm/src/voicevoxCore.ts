import raw, {
  Pointer,
  VoicevoxCore,
  VoicevoxResultCode,
} from "./artifacts/voicevox_core_wasm_api";
import dict from "./artifacts/open_jtalk_dic.zip?url";
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

function allocPointer(vvc: VoicevoxCore) {
  return vvc._malloc(4) as Pointer;
}

export class OpenJtalkRc {
  static async new() {
    const vvc = await voicevoxCore();
    const zip = await JSZip.loadAsync(
      await fetch(dict).then((res) => res.arrayBuffer())
    );
    try {
      vvc.FS.mkdir("/data");

      for (const [name, data] of Object.entries(zip.files)) {
        console.log("Extracting", name);
        if (name.endsWith("/")) {
          vvc.FS.mkdir(`/data/${name}`);
        } else {
          vvc.FS.writeFile(`/data/${name}`, await data.async("uint8array"));
        }
      }
    } catch (e) {
      console.error(e);
    }
    const returnPtr = allocPointer(vvc);
    throwIfError(
      vvc,
      vvc.ccall(
        "voicevox_open_jtalk_rc_new",
        "number",
        ["string", "number"],
        ["/data/open_jtalk_dic_utf_8-1.11", returnPtr]
      )
    );
    const pointer = vvc.getValue(returnPtr, "i32") as Pointer;

    return new OpenJtalkRc(pointer);
  }
  constructor(private pointer: Pointer) {}
}

export class Synthesizer {}
export class Model {}
