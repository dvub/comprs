"use client";
// TODO: ADD DOCS

import { TimeKnob } from "@/components/knobs/generic/TimeKnob";
import { RatioKnob } from "@/components/knobs/RatioKnob";
import { ThresholdKnob } from "@/components/knobs/ThresholdKnob";
import { NormalisableRange } from "@/lib/utils";
import React, { useEffect, useLayoutEffect } from "react";
import { DryWetKnob } from "@/components/knobs/DryWetKnob";
import { KneeKnob } from "@/components/knobs/KneeKnob";
import { DBKnob } from "@/components/knobs/generic/DBKnob";
import { ParameterEvent } from "@/bindings/ParameterEvent";
import { sendToPlugin } from "@/lib";

export default function Home() {
  useEffect(() => {
    sendToPlugin("Init");
    window.onPluginMessage = (message: ParameterEvent) => {
      let event = new CustomEvent("pluginMessage", { detail: message });
      window.dispatchEvent(event);
    };
  }, []);

  return (
    <main className="main-bg w-screen h-screen overflow-hidden px-3 py-5 text-[#180619] ">
      <div className="flex justify-center items-center h-[25%]">
        <h1 className="font-thin text-7xl">COMPRS</h1>
      </div>
      <div className="h-[50%] flex justify-between items-center gap-3">
        <DBKnob
          label="IN"
          maxValue={30}
          minValue={-30}
          defaultValue={0}
          parameter="InputGain"
        />
        {/* middle section, contains [IMO] the most important parameters */}
        <div className="flex gap-3 justify-center">
          <ThresholdKnob />
          <div className="w-36 h-36 bg-slate-200"></div>
          <RatioKnob />
        </div>
        {/* this div contains output-related knobs */}
        <div>
          <DryWetKnob />
          <DBKnob
            label="OUT"
            maxValue={30}
            minValue={-30}
            defaultValue={0}
            parameter="OutputGain"
          />
        </div>
      </div>
      {/* bottom section, just kinda put misc params here */}
      <div className="h-[25%] w-full flex justify-center">
        <div className="flex gap-3">
          <TimeKnob
            label="ATK"
            minValue={0}
            maxValue={1}
            defaultValue={0.001}
            range={new NormalisableRange(0, 1, 0.01)}
            parameter="AttackTime"
          />
          <TimeKnob
            label="RLS"
            minValue={0}
            maxValue={5}
            defaultValue={0.05}
            range={new NormalisableRange(0, 5, 0.5)}
            parameter="ReleaseTime"
          />
          <KneeKnob />
        </div>
      </div>
    </main>
  );
}
