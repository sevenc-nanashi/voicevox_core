declare module "voicevox_core_wasm_api";
type Functions = {
  voicevox_get_version: () => string;
};
type Ccall = <T extends keyof Functions>(
  name: T,
  returnType: ReturnType<Functions[T]>,
  argTypes: ("number" | "string" | "array" | "boolean")[],
  args: Parameters<Functions[T]>
) => ReturnType<Functions[T]>;
export type VoicevoxCore = {
  ccall: Ccall;
};
export default function (): Promise<
  VoicevoxCore & {
    ready: Promise<VoicevoxCore>;
  }
>;
