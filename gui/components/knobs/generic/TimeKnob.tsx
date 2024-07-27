import { Dispatch, SetStateAction, useState } from "react";
import { KnobBase } from "../../KnobBase";
import { NormalisableRange } from "@/lib/utils";
import { Parameter } from "@/bindings/Parameter";
import { ParameterType } from "@/lib";

export function TimeKnob(props: {
  label: string;
  maxValue: number;
  minValue: number;
  defaultValue: number;
  range: NormalisableRange;
  parameter: ParameterType;
  value: number;
  setValue: Dispatch<SetStateAction<number>>;
}) {
  const { label, maxValue, minValue, defaultValue, range, value, setValue } =
    props;

  // the sauce of this component
  // LOL
  const displayFn = (valueRaw: number) => {
    let multiplier = 1;
    let unit = " s";
    if (valueRaw < 1) {
      multiplier = 1000;
      unit = " ms";
    }
    return `${Number((valueRaw * multiplier).toFixed(2))}${unit}`;
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
      parameter={props.parameter}
    />
  );
}
