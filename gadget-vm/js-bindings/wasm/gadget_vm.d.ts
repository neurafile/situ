/* tslint:disable */
/* eslint-disable */

export class GadgetVM {
  free(): void;
  [Symbol.dispose](): void;
  constructor();
  parse(source: string): string;
  process(input: string): string;
  /**
   * Play a sound file. Returns the sound ID, or -1 on error.
   * The WASM manages which sounds are playing and can stop them.
   */
  play_sound(path: string, note_name: string): number;
  /**
   * Stop a sound by note name
   */
  stop_sound(note_name: string): boolean;
  /**
   * Stop all currently playing sounds
   */
  stop_all_sounds(): void;
  /**
   * Get list of currently playing note names
   */
  get_active_sounds(): string[];
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_gadgetvm_free: (a: number, b: number) => void;
  readonly gadgetvm_new: () => number;
  readonly gadgetvm_parse: (a: number, b: number, c: number) => [number, number, number, number];
  readonly gadgetvm_process: (a: number, b: number, c: number) => [number, number];
  readonly gadgetvm_play_sound: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly gadgetvm_stop_sound: (a: number, b: number, c: number) => number;
  readonly gadgetvm_stop_all_sounds: (a: number) => void;
  readonly gadgetvm_get_active_sounds: (a: number) => [number, number];
  readonly __wbindgen_externrefs: WebAssembly.Table;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __externref_drop_slice: (a: number, b: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
