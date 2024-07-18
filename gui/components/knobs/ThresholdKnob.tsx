import { useState } from "react";
import { NormalisableRange } from "@/lib/utils";
import { KnobBase } from "../KnobBase";

export function ThresholdKnob() {
  const [threshold, setThreshold] = useState(0);
  return (
    <KnobBase
      label="THRESH"
      valueRaw={threshold}
      setRawValue={setThreshold}
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
