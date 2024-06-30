import { clamp01 } from "@dsp-ts/math";

// Conversion functions rewritten from NIH-plug
// https://github.com/robbert-vdh/nih-plug/blob/ffe9b61fcb0441c9d33f4413f5ebe7394637b21f/src/util.rs#L30

export function dbToGain(db: number) {
  return 10 ** (db * 0.05);
}

export function gainToDb(gain: number) {
  return Math.log10(Math.max(gain, -100.0)) * 20.0;
}

// Full credit:
// https://github.com/satelllte/react-knob-headless/blob/main/apps/docs/src/utils/math/NormalisableRange.ts

/**
 * Partial implementation of the "NormalisableRange" class from JUCE Framework.
 * In particular, the only part taken from JUCE is "skew" calculation by given "center" point of the range.
 * ---
 * Useful for making logarithmic interpolations for things like frequency inputs, ADR inputs, etc.
 * ---
 * References:
 * - https://docs.juce.com/master/classNormalisableRange.html
 * - https://github.com/juce-framework/JUCE/blob/master/modules/juce_core/maths/juce_NormalisableRange.h
 */
export class NormalisableRange {
  private readonly _min: number;
  private readonly _max: number;
  private readonly _skew: number;

  constructor(min: number, max: number, center: number) {
    this._min = min;
    this._max = max;
    this._skew = Math.log(0.5) / Math.log((center - min) / (max - min));
  }

  public mapTo01(x: number): number {
    const proportion = clamp01((x - this._min) / (this._max - this._min));

    if (this._skew === 1) {
      return proportion;
    }

    return proportion ** this._skew;
  }

  public mapFrom01(proportion: number): number {
    proportion = clamp01(proportion);

    if (this._skew !== 1 && proportion > 0) {
      proportion = Math.exp(Math.log(proportion) / this._skew);
    }

    return this._min + (this._max - this._min) * proportion;
  }
}
