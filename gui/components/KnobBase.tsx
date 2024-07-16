"use client";
/**
 * Modified knob BASE -
 * source:
 * https://github.com/satelllte/react-knob-headless/blob/main/apps/docs/src/components/knobs/KnobBase.tsx
 */

import clsx from "clsx";
import { useEffect, useId } from "react";
import {
  KnobHeadless,
  KnobHeadlessOutput,
  useKnobKeyboardControls,
} from "react-knob-headless";
import { mapFrom01Linear, mapTo01Linear } from "@dsp-ts/math";
import { KnobBaseThumb } from "./KnobBaseThumb";
import { sendToPlugin } from "../lib";
import { NormalisableRange } from "@/lib/utils";
import { Action } from "@/bindings/Action";

type KnobHeadlessProps = React.ComponentProps<typeof KnobHeadless>;

type KnobBaseProps = Pick<
  KnobHeadlessProps,
  | "valueMin"
  | "valueMax"
  | "valueRaw"
  | "valueRawRoundFn"
  | "valueRawDisplayFn"
  | "orientation"
  | "mapTo01"
  | "mapFrom01"
> & {
  label: string;
  valueDefault: number;
  stepFn: (valueRaw: number) => number;
  stepLargerFn: (valueRaw: number) => number;
  setRawValue: React.Dispatch<React.SetStateAction<number>>;
  size: number;
  range: NormalisableRange;
  type: Action["type"];
};

export function KnobBase(props: KnobBaseProps) {
  // this value can be tweaked to adjust the feel of the knob
  const dragSensitivity = 0.003;

  let {
    label,
    valueDefault,
    valueMin,
    valueMax,
    valueRawDisplayFn,
    stepFn,
    stepLargerFn,
    setRawValue,
    size,
    type,
    mapTo01 = mapTo01Linear,
    mapFrom01 = mapFrom01Linear,
    valueRaw,
  } = props;

  const knobId = useId();
  const labelId = useId();
  const keyboardControlHandlers = useKnobKeyboardControls({
    valueRaw: valueRaw,
    valueMin,
    valueMax,
    step: stepFn(valueRaw),
    stepLarger: stepLargerFn(valueRaw),
    onValueRawChange: setVal,
  });

  // listen for DAW parameter events and update state
  useEffect(() => {
    // NOTE:
    // here's im using `any` because addEventListener will complain otherwise
    const handlePluginMessage = (event: any) => {
      // to get some type safety back, we can add Action here
      let message: Action = event.detail;
      if (message.type === type) {
        setRawValue(message.value);
      }
    };

    window.addEventListener("pluginMessage", handlePluginMessage);
    return () => {
      window.removeEventListener("pluginMessage", handlePluginMessage);
    };
  }, []);

  // in addition to changing the state,
  // we want to also send a message to the plugin backend here
  function setVal(valueRaw: number) {
    setRawValue(valueRaw);
    sendToPlugin({ type: type, value: valueRaw });
  }

  function resetValue() {
    setVal(valueDefault);
  }

  let thumbProps = {
    value01: mapTo01(valueRaw, valueMin, valueMax),
    label: label,
    resetValue: resetValue,
  };

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
        className={`relative outline-none`}
        style={{ width: `${size}px`, height: `${size}px` }}
        dragSensitivity={dragSensitivity}
        mapTo01={mapTo01}
        mapFrom01={mapFrom01}
        onValueRawChange={setVal}
        {...props}
        {...keyboardControlHandlers}
      >
        <KnobBaseThumb {...thumbProps} />
      </KnobHeadless>
      <KnobHeadlessOutput htmlFor={knobId}>
        {valueRawDisplayFn(valueRaw)}
      </KnobHeadlessOutput>
    </div>
  );
}
