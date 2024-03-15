declare module "voicevox_core_wasm_api";
import "@types/emscripten";
type Brand<K, T> = K & { __brand: T };

export type VoicevoxResultCode = Brand<number, "VoicevoxResultCode">;
export type Pointer = Brand<number, "Pointer">;

type Functions = {
  voicevox_get_version: () => string;
  voicevox_open_jtalk_rc_new: (
    path: string,
    pointer: Pointer
  ) => VoicevoxResultCode;
  voicevox_error_result_to_message: (code: VoicevoxResultCode) => string;
  setenv: (name: string, value: string) => number;
};
type Ccall = <T extends keyof Functions>(
  name: T,
  returnType: "number" | "string" | "array" | "boolean",
  argTypes: ("number" | "string" | "array" | "boolean")[],
  args: Parameters<Functions[T]>
) => ReturnType<Functions[T]>;
type VoicevoxCore = EmscriptenModule & {
  ccall: Ccall;
  ready: Promise<VoicevoxCore>;
  getValue: typeof getValue;
  FS: typeof FS;
};
export default function (): Promise<
  VoicevoxCore & {
    ready: Promise<VoicevoxCore>;
  }
>;
