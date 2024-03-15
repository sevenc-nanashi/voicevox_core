import raw, { VoicevoxCore } from "./artifacts/voicevox_core_wasm_api";

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
    _voicevoxCore = vvc;
    for (const resolve of resolvers) {
      resolve(vvc);
    }
  });
});
export async function getVersion() {
  const vvc = await voicevoxCore();
  return vvc.ccall("voicevox_get_version", "string", [], []);
}

export class Synthesizer {}
export class Model {}
