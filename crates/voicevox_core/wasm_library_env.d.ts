declare function addToLibrary(
  functions: Record<string, string[] | string | Function>,
): void;
declare function _malloc(size: number): number;
declare function _free(ptr: number): void;
declare function dynCall(sig: string, ptr: number, args: any[]): any;
declare function UTF8ToString(ptr: number): string;

declare const HEAPU8: Uint8Array;
declare const HEAPU16: Uint16Array;
declare const HEAPU32: Uint32Array;
declare const HEAP8: Int8Array;
declare const HEAP16: Int16Array;
declare const HEAP32: Int32Array;
