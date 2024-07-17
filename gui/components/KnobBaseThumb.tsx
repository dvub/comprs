// TODO:
// allow user to type in values, maybe through some sort of form, i don't know

/**
 * Modified knob thumb -
 * original source:
 * https://github.com/satelllte/react-knob-headless/blob/main/apps/docs/src/components/knobs/KnobBaseThumb.tsx
 */

import { mapFrom01Linear } from "@dsp-ts/math";

type KnobBaseThumbProps = {
  readonly resetValue: () => void;
  readonly value01: number;
  readonly label: string;
};

export function KnobBaseThumb({
  resetValue,
  label,
  value01,
}: KnobBaseThumbProps) {
  // when the element is double-clicked, we want to call whatever function was passed to reset the knob
  function handleClick(event: { detail: number }) {
    if (event.detail === 2) {
      resetValue();
    }
  }
  const angleMin = -145;
  const angleMax = 145;
  const angle = mapFrom01Linear(value01, angleMin, angleMax);
  return (
    <div
      className="absolute h-full w-full rounded-full bg-[#b42770]  shadow-lg shadow-black/50"
      onClick={handleClick}
    >
      {/* Pointer line thingy - is it called a thumb ?? */}
      <div className="absolute h-full w-full" style={{ rotate: `${angle}deg` }}>
        <div className="absolute left-1/2 top-0 h-1/2 w-[2px] -translate-x-1/2 rounded-sm main-bg z-20" />
        <p className="absolute top-[50%] text-center w-full -translate-y-1/2 font-black text-lg z-10 text-[#fcf3fc] opacity-25">
          {label}
        </p>
      </div>
    </div>
  );
}
