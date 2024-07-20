import { KnobBase } from "@/components/KnobBase";
import { ParameterType } from "@/lib";
import { NormalisableRange } from "@/lib/utils";
import { useState } from "react";

export function PercentKnob(props: {
  label: string;
  maxValue: number;
  minValue: number;
  defaultValue: number;
  range: NormalisableRange;
  parameter: ParameterType;
  size: number;
}) {
  const { label, maxValue, minValue, defaultValue, range } = props;
  const [value, setValue] = useState(defaultValue);

  // the sauce of this component
  // LOL
  const displayFn = (valueRaw: number) => {
    return `${Number((valueRaw * 100).toFixed(2))}%`;
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
      size={props.size}
      stepFn={(valueRaw: number): number => 0}
      stepLargerFn={(valueRaw: number): number => 0}
      parameter={props.parameter}
    />
  );
}
