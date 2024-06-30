/**
 * DB Knob
 *
 *
 *
 * Source:
 * https://github.com/satelllte/react-knob-headless/blob/main/apps/docs/src/components/knobs/KnobFrequency.tsx
 */

import { NormalisableRange } from "@/lib/utils";
import { KnobBase } from "./KnobBase";

export function GenericKnob(props: {
  minValue: number;
  maxValue: number;
  defaultValue: number;
  label: string;
  rawValue: number;
  setRawValue: React.Dispatch<React.SetStateAction<number>>;
  range: NormalisableRange;
  roundingFn: (valueRaw: number) => number;
  displayFn: (valueRaw: number) => string;
  size: number;
}) {
  const { minValue, maxValue, defaultValue, range, roundingFn, displayFn } =
    props;

  // step functions are for keyboard control
  const stepFn = (valueRaw: number): number => 0;
  const stepLargerFn = (valueRaw: number): number => 0;
  const mapTo01 = (x: number) => range.mapTo01(x);
  const mapFrom01 = (x: number) => range.mapFrom01(x);
  return (
    <KnobBase
      valueDefault={defaultValue}
      valueMin={minValue}
      valueMax={maxValue}
      stepFn={stepFn}
      stepLargerFn={stepLargerFn}
      valueRawRoundFn={roundingFn}
      valueRawDisplayFn={displayFn}
      mapTo01={mapTo01}
      mapFrom01={mapFrom01}
      {...props}
    />
  );
}
