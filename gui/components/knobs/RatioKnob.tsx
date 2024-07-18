import { useState } from "react";
import { KnobBase } from "../KnobBase";
import { NormalisableRange } from "@/lib/utils";

export function RatioKnob() {
  const [ratio, setRatio] = useState(0);
  return (
    <KnobBase
      label="RATIO"
      valueRaw={ratio}
      setRawValue={setRatio}
      valueMin={1}
      valueMax={100}
      valueDefault={4}
      range={
        new NormalisableRange(
          1,
          100,
          25 // again, randomly chose
        )
      }
      valueRawRoundFn={(valueRaw) => Number(valueRaw.toFixed(2))}
      valueRawDisplayFn={(valueRaw) => `${Number(valueRaw.toFixed(2))}:1 dB`}
      size={144}
      stepFn={(valueRaw: number): number => 0}
      stepLargerFn={(valueRaw: number): number => 0}
      parameter="Ratio"
    />
  );
}
