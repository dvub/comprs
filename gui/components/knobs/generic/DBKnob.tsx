/**
 * DB Knob
 *
 * Source:
 * https://github.com/satelllte/react-knob-headless/blob/main/apps/docs/src/components/knobs/KnobFrequency.tsx
 */

import { NormalisableRange, dbToGain, gainToDb } from "@/lib/utils";
import { KnobBase } from "../../KnobBase";
import { useState } from "react";
import { Parameter } from "@/bindings/Parameter";
import { ParameterType } from "@/lib";

export function DBKnob(props: {
  label: string;
  minValue: number;
  maxValue: number;
  defaultValue: number;
  parameter: ParameterType;
}) {
  const { label, minValue, maxValue, defaultValue } = props;

  const valueMin = dbToGain(minValue);
  const valueMax = dbToGain(maxValue);
  const valueDefault = dbToGain(defaultValue);
  const [gain, setGain] = useState(valueDefault);
  // step functions are for keyboard control
  const stepFn = (valueRaw: number): number => 0;
  const stepLargerFn = (valueRaw: number): number => 0;
  //
  const normalisableRange = new NormalisableRange(
    valueMin,
    valueMax,
    valueDefault
  );
  const valueRawRoundFn = (x: number): number => Number(x.toFixed(2));
  const valueRawDisplayFn = (valueRaw: number): string =>
    `${valueRawRoundFn(gainToDb(valueRaw))} dB`;

  return (
    <KnobBase
      valueRaw={gain}
      setRawValue={setGain}
      label={label}
      valueDefault={valueDefault}
      valueMin={valueMin}
      valueMax={valueMax}
      stepFn={stepFn}
      stepLargerFn={stepLargerFn}
      valueRawRoundFn={valueRawRoundFn}
      valueRawDisplayFn={valueRawDisplayFn}
      size={96}
      range={normalisableRange}
      parameter={props.parameter}
    />
  );
}
