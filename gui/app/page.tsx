"use client";

import { DBKnob } from "@/components/knobs/DBKnob";
import { GenericKnob } from "@/components/knobs/GenericKnob";
import { NormalisableRange } from "@/lib/utils";
import React from "react";

export default function Home() {
  const [rawInputGain, setRawInputGain] = React.useState(0);
  const [rawOutputGain, setRawOutputGain] = React.useState(0);
  const [threshold, setThreshold] = React.useState(0);
  const [ratio, setRatio] = React.useState(0);
  const [dryWet, setDryWet] = React.useState(0);
  const [attackTime, setAttackTime] = React.useState(0);
  const [releaseTime, setReleaseTime] = React.useState(0);
  const [knee, setKnee] = React.useState(0);
  return (
    <main className="main-bg w-screen h-screen overflow-hidden px-3 py-5 text-[#180619] ">
      <div className="flex justify-center items-center h-[25%]">
        <h1 className="font-thin text-7xl">COMPRS</h1>
      </div>
      <div className="h-[50%] flex justify-between items-center gap-3">
        <DBKnob
          label="IN GAIN"
          rawValue={rawInputGain}
          setRawValue={setRawInputGain}
          minValue={-30}
          maxValue={30}
          defaultValue={0}
        />
        <div className="flex gap-3 justify-center">
          <GenericKnob
            label="THRESH"
            rawValue={threshold}
            setRawValue={setThreshold}
            minValue={-100}
            maxValue={5}
            defaultValue={0}
            range={
              new NormalisableRange(
                -100,
                5,
                -25 // idk i just kinda chose this tbh
              )
            }
            roundingFn={(valueRaw) => Number(valueRaw.toFixed(2))}
            displayFn={(valueRaw) => `${Number(valueRaw.toFixed(2))} dB`}
            size={36}
          />
          <div className="w-36 h-36 bg-slate-200"></div>
          <GenericKnob
            label="RATIO"
            rawValue={ratio}
            setRawValue={setRatio}
            minValue={1}
            maxValue={100}
            defaultValue={4}
            range={
              new NormalisableRange(
                1,
                100,
                25 // again, randomly chose
              )
            }
            roundingFn={(valueRaw) => Number(valueRaw.toFixed(2))}
            displayFn={(valueRaw) => `${Number(valueRaw.toFixed(2))}:1 dB`}
            size={36}
          />
        </div>
        <div>
          <GenericKnob
            label="DRY/WET"
            rawValue={dryWet}
            setRawValue={setDryWet}
            minValue={0}
            maxValue={1}
            defaultValue={1}
            range={new NormalisableRange(0, 1, 0.5)}
            roundingFn={(valueRaw) => Number(valueRaw.toFixed(2))}
            displayFn={(valueRaw) => `${Number((valueRaw * 100).toFixed(2))}%`}
            size={24}
          />
          <DBKnob
            label="OUT GAIN"
            rawValue={rawOutputGain}
            setRawValue={setRawOutputGain}
            minValue={-30}
            maxValue={30}
            defaultValue={0}
          />
        </div>
      </div>
      <div className="h-[25%] w-full flex justify-center">
        <div className="flex gap-3">
          <GenericKnob
            label="ATK"
            rawValue={attackTime}
            setRawValue={setAttackTime}
            minValue={0}
            maxValue={1}
            defaultValue={0.001}
            range={new NormalisableRange(0, 1, 0.01)}
            roundingFn={(valueRaw) => Number(valueRaw.toFixed(3))}
            displayFn={(valueRaw) =>
              `${Number((valueRaw * 1000).toFixed(3))}ms`
            }
            size={24}
          />
          <GenericKnob
            label="RLS"
            rawValue={releaseTime}
            setRawValue={setReleaseTime}
            minValue={0}
            maxValue={5}
            defaultValue={0.05}
            range={new NormalisableRange(0, 5, 0.5)}
            roundingFn={(valueRaw) => Number(valueRaw.toFixed(3))}
            displayFn={(valueRaw) =>
              `${Number((valueRaw * 1000).toFixed(1))}ms`
            }
            size={24}
          />
          <GenericKnob
            label="KNEE"
            rawValue={knee}
            setRawValue={setKnee}
            minValue={0}
            maxValue={20}
            defaultValue={5}
            range={new NormalisableRange(0, 20, 10)}
            roundingFn={(valueRaw) => Number(valueRaw.toFixed(2))}
            displayFn={(valueRaw) => `${Number(valueRaw.toFixed(2))} dB`}
            size={24}
          />
        </div>
      </div>
    </main>
  );
}
