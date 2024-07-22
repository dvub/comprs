import { Dispatch, SetStateAction, useState } from "react";
import { NormalisableRange } from "@/lib/utils";
import { KnobBase } from "../KnobBase";

export function ThresholdKnob(props: {
  value: number;
  setValue: Dispatch<SetStateAction<number>>;
}) {
  const { value, setValue } = props;
  return (
    <KnobBase
      label="THRESH"
      valueRaw={value}
      setRawValue={setValue}
      valueMin={-100}
      valueMax={5}
      valueDefault={0}
      range={
        new NormalisableRange(
          -100,
          5,
          -25 // idk i just kinda chose this tbh
        )
      }
      valueRawRoundFn={(valueRaw: number) => Number(valueRaw.toFixed(2))}
      valueRawDisplayFn={(valueRaw: number) =>
        `${Number(valueRaw.toFixed(2))} dB`
      }
      size={144}
      stepFn={(valueRaw: number): number => 0}
      stepLargerFn={(valueRaw: number): number => 0}
      parameter="Threshold"
    />
  );
}
