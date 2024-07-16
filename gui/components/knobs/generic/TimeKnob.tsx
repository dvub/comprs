import { useState } from "react";
import { KnobBase } from "../../KnobBase";
import { NormalisableRange } from "@/lib/utils";
import { Action } from "@/bindings/Action";

export function TimeKnob(props: {
  label: string;
  maxValue: number;
  minValue: number;
  defaultValue: number;
  range: NormalisableRange;
  type: Action["type"];
}) {
  const { label, maxValue, minValue, defaultValue, range } = props;
  const [value, setValue] = useState(defaultValue);

  // the sauce of this component
  // LOL
  const displayFn = (valueRaw: number) => {
    let multiplier = 1;
    let unit = " s";
    if (valueRaw < 1) {
      multiplier = 1000;
      unit = " ms";
    }
    return `${Number((valueRaw * multiplier).toFixed(3))}${unit}`;
  };
  return (
    <KnobBase
      label={label}
      valueRaw={value}
      setRawValue={setValue}
      valueMin={minValue}
      valueMax={maxValue}
      valueDefault={defaultValue}
      range={range}
      valueRawRoundFn={(valueRaw) => Number(valueRaw.toFixed(3))}
      valueRawDisplayFn={displayFn}
      size={96}
      stepFn={(valueRaw: number): number => 0}
      stepLargerFn={(valueRaw: number): number => 0}
      type={props.type}
    />
  );
}
