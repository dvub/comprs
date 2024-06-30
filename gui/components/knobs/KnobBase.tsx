/**
 * Modified knob BASE -
 * source:
 * https://github.com/satelllte/react-knob-headless/blob/main/apps/docs/src/components/knobs/KnobBase.tsx
 */

import clsx from "clsx";
import { useId, useState } from "react";
import {
  KnobHeadless,
  KnobHeadlessLabel,
  KnobHeadlessOutput,
  useKnobKeyboardControls,
} from "react-knob-headless";
import { mapFrom01Linear, mapTo01Linear } from "@dsp-ts/math";
import { KnobBaseThumb } from "./KnobBaseThumb";
import { sendToPlugin } from "../../lib";

type KnobHeadlessProps = React.ComponentProps<typeof KnobHeadless>;
type KnobBaseThumbProps = React.ComponentProps<typeof KnobBaseThumb>;
type KnobBaseProps = Pick<
  KnobHeadlessProps,
  | "valueMin"
  | "valueMax"
  | "valueRawRoundFn"
  | "valueRawDisplayFn"
  | "orientation"
  | "mapTo01"
  | "mapFrom01"
> & {
  readonly label: string;
  readonly valueDefault: number;
  readonly stepFn: (valueRaw: number) => number;
  readonly stepLargerFn: (valueRaw: number) => number;
  rawValue: number;
  setRawValue: React.Dispatch<React.SetStateAction<number>>;
  size: number;
};

export function KnobBase({
  label,
  valueDefault,
  valueMin,
  valueMax,
  valueRawRoundFn,
  valueRawDisplayFn,
  orientation,
  stepFn,
  stepLargerFn,
  rawValue,
  setRawValue,
  size,
  mapTo01 = mapTo01Linear,
  mapFrom01 = mapFrom01Linear,
}: KnobBaseProps) {
  const knobId = useId();
  const labelId = useId();
  const value01 = mapTo01(rawValue, valueMin, valueMax);
  const step = stepFn(rawValue);
  const stepLarger = stepLargerFn(rawValue);
  const dragSensitivity = 0.003;
  const keyboardControlHandlers = useKnobKeyboardControls({
    valueRaw: rawValue,
    valueMin,
    valueMax,
    step,
    stepLarger,
    onValueRawChange: setVal,
  });

  // in addition to changing the state,
  // we want to also send a message to the plugin backend here
  function setVal(valueRaw: number) {
    setRawValue(valueRaw);
    sendToPlugin({ type: "SetGain", value: valueRaw });
  }

  function resetValue() {
    setVal(valueDefault);
  }
  return (
    <div
      className={clsx(
        "flex flex-col gap-0.5 justify-center items-center text-xs select-none",
        "outline-none"
      )}
    >
      {/*<KnobHeadlessLabel id={labelId}>{label}</KnobHeadlessLabel>*/}
      <KnobHeadless
        id={knobId}
        aria-labelledby={labelId}
        className={`relative w-${size} h-${size} outline-none`}
        valueMin={valueMin}
        valueMax={valueMax}
        valueRaw={rawValue}
        valueRawRoundFn={valueRawRoundFn}
        valueRawDisplayFn={valueRawDisplayFn}
        dragSensitivity={dragSensitivity}
        orientation={orientation}
        mapTo01={mapTo01}
        mapFrom01={mapFrom01}
        onValueRawChange={setVal}
        {...keyboardControlHandlers}
      >
        <KnobBaseThumb
          value01={value01}
          label={label}
          resetValue={resetValue}
        />
      </KnobHeadless>
      <KnobHeadlessOutput
        htmlFor={knobId}
        onClick={(e) => console.log("clicked!")}
      >
        {/* TODO: ADD <input> HERE */}
        {valueRawDisplayFn(rawValue)}
      </KnobHeadlessOutput>
    </div>
  );
}
