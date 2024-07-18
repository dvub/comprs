import { NormalisableRange } from "@/lib/utils";
import { KnobBase } from "../KnobBase";
import { useState } from "react";

export function DryWetKnob() {
  const [dryWet, setDryWet] = useState(1);
  return (
    <KnobBase
      label="DRY/WET"
      valueRaw={dryWet}
      setRawValue={setDryWet}
      valueMin={0}
      valueMax={1}
      valueDefault={1}
      range={new NormalisableRange(0, 1, 0.5)}
      valueRawRoundFn={(valueRaw) => Number(valueRaw.toFixed(2))}
      valueRawDisplayFn={(valueRaw) =>
        `${Number((valueRaw * 100).toFixed(2))}%`
      }
      size={96}
      stepFn={(valueRaw: number): number => 0}
      stepLargerFn={(valueRaw: number): number => 0}
      parameter="DryWet"
    />
  );
}
