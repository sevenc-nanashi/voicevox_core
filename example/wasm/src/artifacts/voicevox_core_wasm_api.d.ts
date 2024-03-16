declare module "voicevox_core_wasm_api";
import "@types/emscripten";
type Brand<K, T> = K & { __brand: T };

export type VoicevoxResultCode = Brand<number, "VoicevoxResultCode">;
export type Pointer<T> = number & { __brand: "Pointer"; __type: T };

type Functions = {
  voicevox_get_version: () => string;
  voicevox_open_jtalk_rc_new: (
    path: string,
    pointer: Pointer<"OpenJtalkRc">
  ) => VoicevoxResultCode;
  voicevox_open_jtalk_rc_delete: (
    pointer: Pointer<"OpenJtalkRc">
  ) => VoicevoxResultCode;
  voicevox_error_result_to_message: (code: VoicevoxResultCode) => string;
  voicevox_make_default_initialize_options_wasm: (
    acceleration_mode: Pointer<"AccelerationMode">,
    cpu_num_threads: Pointer<"i32">
  ) => void;
  voicevox_synthesizer_new_wasm: (
    open_jtalk: Pointer<"OpenJtalkRc">,
    options_acceleration_mode: i32,
    options_cpu_num_threads: i32,
    out_synthesizer: Pointer<"VoicevoxSynthesizer">
  ) => VoicevoxResultCode;
  voicevox_synthesizer_load_voice_model: (
    pointer: Pointer<"VoicevoxSynthesizer">,
    model: Pointer<"VoicevoxVoiceModel">
  ) => VoicevoxResultCode;
  voicevox_synthesizer_delete: (
    pointer: Pointer<"VoicevoxSynthesizer">
  ) => void;
  voicevox_voice_model_new_from_path: (
    path: string,
    pointer: Pointer<"VoicevoxVoiceModel">
  ) => VoicevoxResultCode;
  voicevox_voice_model_delete: (
    pointer: Pointer<"VoicevoxVoiceModel">
  ) => VoicevoxResultCode;
  setenv: (name: string, value: string) => number;
};
type Ccall = <T extends keyof Functions>(
  name: T,
  returnType: "number" | "string" | "array" | "boolean" | "void",
  argTypes: ("number" | "string" | "array" | "boolean")[],
  args: Parameters<Functions[T]>
) => ReturnType<Functions[T]>;
type VoicevoxCore = EmscriptenModule & {
  ccall: Ccall;
  ready: Promise<VoicevoxCore>;
  getValue: typeof getValue;
  stackSave: typeof stackSave;
  stackRestore: typeof stackRestore;
  stackAlloc: typeof stackAlloc;
  FS: typeof FS;
};
export default function (): Promise<
  VoicevoxCore & {
    ready: Promise<VoicevoxCore>;
  }
>;
