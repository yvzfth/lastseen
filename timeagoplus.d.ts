/* tslint:disable */
/* eslint-disable */
/**
 * Calculates the time ago from a given timestamp or `Date` object.
 *
 * # Arguments
 *
 * * `input` - The timestamp or `Date` object representing the time to calculate the time ago from.
 * * `detailed_label` - An optional boolean value indicating whether to include detailed labels (e.g., "2 hours, 30 minutes ago") or not. Defaults to `false`.
 * * `lang` - An optional `Language` enum value indicating the language to use for the labels. Defaults to the default language.
 *
 * # Returns
 *
 * A `String` representing the time ago from the given timestamp or `Date` object.
 *
 * # Panics
 *
 * This function will panic if the `input` is not a valid `Date` object or timestamp.
 *
 * # Example
 *
 * ```
 * import React, { useState, useEffect } from 'react';
 * import init, { time_ago, Language } from 'timeagoplus';
 *
 * const TimeAgoComponent: React.FC = () => {
 *     const [timeAgoString, setTimeAgoString] = useState<string>('');
 *
 *     useEffect(() => {
 *         const fetchData = async () => {
 *             await init();
 *             const timestamp = Date.now();
 *             const result = time_ago(timestamp, true, Language.English);
 *             setTimeAgoString(result);
 *         };
 *
 *         fetchData();
 *     }, []);
 *
 *     return (
 *         <div>
 *             Time ago: {timeAgoString}
 *         </div>
 *     );
 * };
 *
 * export default TimeAgoComponent;
 * ```
 * @param {any} input
 * @param {boolean | undefined} [detailed_label]
 * @param {Language | undefined} [lang]
 * @returns {string}
 */
export function time_ago(
  input: Date | number,
  detailed_label?: boolean,
  lang?: Language
): string;
/**
 */
export enum Language {
  CHN = 0,
  SPN = 1,
  ENG = 2,
  BNG = 3,
  HND = 4,
  POR = 5,
  RUS = 6,
  JPN = 7,
  GER = 8,
  WUU = 9,
  JAN = 10,
  KKN = 11,
  FRN = 12,
  VIE = 13,
  TCW = 14,
  YUH = 15,
  MRT = 16,
  TCV = 17,
  TRK = 18,
  URD = 19,
  CFR = 20,
  CJY = 21,
  GJR = 22,
  PQL = 23,
  ARZ = 24,
  UKR = 25,
  ITN = 26,
  HSN = 27,
  MJS = 28,
  HAK = 29,
  KJV = 30,
  ORY = 31,
  PNB = 32,
  SUO = 33,
  PNJ = 34,
  RUM = 35,
  BHJ = 36,
  AZB = 37,
  PES = 38,
  MKP = 39,
  HUA = 40,
  ARQ = 41,
  BMS = 42,
  SRC = 43,
  KNN = 44,
  AWD = 45,
  THJ = 46,
  DUT = 47,
  YOR = 48,
  SND = 49,
  ARY = 50,
  AEC = 51,
  UZB = 52,
  MLI = 53,
  AMH = 54,
  INZ = 55,
  IGR = 56,
  TGL = 57,
  NEP = 58,
  APD = 59,
  SKR = 60,
  CEB = 61,
  APC = 62,
  TTS = 63,
  ASM = 64,
  HNG = 65,
  CIT = 66,
  ACM = 67,
  MHJ = 68,
  SNH = 69,
  BGC = 70,
  MKD = 71,
  CZC = 72,
  GRK = 73,
  MQM = 74,
  HNE = 75,
  DCC = 76,
  MNP = 77,
  RUW = 78,
  CCX = 79,
  ARS = 80,
  PBU = 81,
  SOM = 82,
  MEX = 83,
  AEB = 84,
  RUA = 85,
  ZUU = 86,
  BLG = 87,
  SWD = 88,
  LMO = 89,
  GAZ = 90,
  PBT = 91,
  KAZ = 92,
  ILO = 93,
  TTR = 94,
  FUV = 95,
  AYN = 96,
  UIG = 97,
  HAT = 98,
  AZE = 99,
  NPL = 100,
  KMR = 101,
  PRS = 102,
  TWS = 103,
  HIL = 104,
  KUR = 105,
  SHD = 106,
}

export type InitInput =
  | RequestInfo
  | URL
  | Response
  | BufferSource
  | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly time_ago: (a: number, b: number, c: number, d: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (
    a: number,
    b: number,
    c: number,
    d: number
  ) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {SyncInitInput} module
 *
 * @returns {InitOutput}
 */
export function initSync(module: SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {InitInput | Promise<InitInput>} module_or_path
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init(
  module_or_path?: InitInput | Promise<InitInput>
): Promise<InitOutput>;
