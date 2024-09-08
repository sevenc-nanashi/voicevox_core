declare module "voicevox_core_wasm_api";
import "@types/emscripten";
type Brand<K, T> = K & { __brand: T };

export type VoicevoxResultCode = Brand<number, "VoicevoxResultCode">;
export type Pointer<T> = number & {
  __brand: `Pointer<${T extends Pointer<any> ? T["__type"] : T}>`;
  __type: T;
};

type Functions = {
  voicevox_get_version: () => string;
  voicevox_onnxruntime_get: () => Pointer<"VoicevoxOnnxruntime">;
  voicevox_onnxruntime_init_once_wasm: (
    out_onnxruntime: Pointer<Pointer<"VoicevoxOnnxruntime">>,
  ) => VoicevoxResultCode;
  voicevox_open_jtalk_rc_new: (
    path: string,
    pointer: Pointer<Pointer<"OpenJtalkRc">>,
  ) => VoicevoxResultCode;
  voicevox_open_jtalk_rc_delete: (
    pointer: Pointer<"OpenJtalkRc">,
  ) => VoicevoxResultCode;
  voicevox_error_result_to_message: (code: VoicevoxResultCode) => string;
  voicevox_make_default_initialize_options_wasm: (
    acceleration_mode: Pointer<"AccelerationMode">,
    cpu_num_threads: Pointer<"i32">,
  ) => void;
  voicevox_make_default_tts_options_wasm: (
    enable_interrogative_upspeak: Pointer<"boolean">,
  ) => void;
  voicevox_synthesizer_new_wasm: (
    onnxruntime: Pointer<"VoicevoxOnnxruntime">,
    open_jtalk: Pointer<"OpenJtalkRc">,
    options_acceleration_mode: number,
    options_cpu_num_threads: number,
    out_synthesizer: Pointer<Pointer<"VoicevoxSynthesizer">>,
  ) => VoicevoxResultCode;
  voicevox_synthesizer_load_voice_model: (
    pointer: Pointer<"VoicevoxSynthesizer">,
    model: Pointer<"VoicevoxVoiceModel">,
  ) => Promise<VoicevoxResultCode>;
  voicevox_synthesizer_create_metas_json: (
    pointer: Pointer<"VoicevoxSynthesizer">,
  ) => Pointer<"string">;
  voicevox_synthesizer_tts: (
    pointer: Pointer<"VoicevoxSynthesizer">,
    text: string,
    speaker: number,
    options_enable_interrogative_upspeak: boolean,
    output_wav_length: Pointer<"i32">,
    output_wav: Pointer<Pointer<"u8">>,
  ) => Promise<VoicevoxResultCode>;
  voicevox_synthesizer_delete: (
    pointer: Pointer<"VoicevoxSynthesizer">,
  ) => void;
  voicevox_voice_model_new_from_path: (
    path: string,
    pointer: Pointer<Pointer<"VoicevoxVoiceModel">>,
  ) => VoicevoxResultCode;
  voicevox_voice_model_get_metas_json: (
    pointer: Pointer<"VoicevoxVoiceModel">,
  ) => Pointer<"string">;
  voicevox_voice_model_delete: (
    pointer: Pointer<"VoicevoxVoiceModel">,
  ) => VoicevoxResultCode;
  voicevox_json_free: (pointer: Pointer<"string">) => void;
  voicevox_wav_free: (pointer: Pointer<"u8">) => void;
  setenv: (name: string, value: string) => number;
};
type ToWasmType<T> = T extends number
  ? "number"
  : T extends string
    ? "string"
    : T extends boolean
      ? "boolean"
      : T extends Pointer<any>
        ? "number"
        : T extends void
          ? "void"
          : never;
type Ccall = <T extends keyof Functions>(
  name: T,
  returnType: ReturnType<Functions[T]> extends Promise<infer U>
    ? ToWasmType<U>
    : ToWasmType<ReturnType<Functions[T]>>,
  argTypes: ("number" | "string" | "boolean" | "void")[],
  args: Parameters<Functions[T]>,
) => ReturnType<Functions[T]>;
type VoicevoxCore = EmscriptenModule & {
  ccall: Ccall;
  ready: Promise<VoicevoxCore>;
  getValue: typeof getValue;
  stackSave: typeof stackSave;
  stackRestore: typeof stackRestore;
  stackAlloc: typeof stackAlloc;
  UTF8ToString: typeof UTF8ToString;
  FS: typeof FS;
};
export default function (): Promise<
  VoicevoxCore & {
    ready: Promise<VoicevoxCore>;
  }
>;
