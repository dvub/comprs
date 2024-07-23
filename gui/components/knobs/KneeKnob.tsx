import { useState } from "react";
import { KnobBase } from "../KnobBase";
import { NormalisableRange } from "@/lib/utils";

export function KneeKnob(props: { value: number; setValue: any }) {
  const { value, setValue } = props;
  return (
    <KnobBase
      label="KNEE"
      valueRaw={value}
      setRawValue={setValue}
      valueMin={0}
      valueMax={20}
      valueDefault={5}
      range={new NormalisableRange(0, 20, 10)}
      valueRawRoundFn={(valueRaw) => Number(valueRaw.toFixed(2))}
      valueRawDisplayFn={(valueRaw) => `${Number(valueRaw.toFixed(2))} dB`}
      size={96}
      stepFn={(valueRaw: number): number => 0}
      stepLargerFn={(valueRaw: number): number => 0}
      parameter="KneeWidth"
    />
  );
}
