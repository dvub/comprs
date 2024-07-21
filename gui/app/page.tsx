"use client";
// TODO: ADD DOCS

import { TimeKnob } from "@/components/knobs/generic/TimeKnob";
import { RatioKnob } from "@/components/knobs/RatioKnob";
import { ThresholdKnob } from "@/components/knobs/ThresholdKnob";
import { NormalisableRange } from "@/lib/utils";
import React, { useEffect, useLayoutEffect } from "react";

import { KneeKnob } from "@/components/knobs/KneeKnob";
import { DBKnob } from "@/components/knobs/generic/DBKnob";
import { Parameter } from "@/bindings/Parameter";
import { sendToPlugin } from "@/lib";
import { PercentKnob } from "@/components/knobs/generic/PercentKnob";

export default function Home() {
  useEffect(() => {
    window.onPluginMessage = (message: Parameter) => {
      let event = new CustomEvent("pluginMessage", { detail: message });
      window.dispatchEvent(event);
    };

    // TODO:
    // prevent knobs from setting themselves after a delay... really annoying and jarring
    sendToPlugin("Init");
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
        <div className="text-center">
          <p>OUT CTRL</p>
          <PercentKnob
            label="DRYWET"
            size={96}
            minValue={0}
            maxValue={1}
            defaultValue={1}
            range={new NormalisableRange(0, 1, 0.5)}
            parameter="DryWet"
          />
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
        <div className="flex gap-3 items-center">
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
          {/* RMS CONTROLS */}
          <div className="text-center">
            <p>RMS CTRL</p>
            <div className="flex gap-3">
              <PercentKnob
                label="RMS%"
                minValue={0}
                maxValue={1}
                defaultValue={0}
                range={new NormalisableRange(0, 1, 0.5)}
                parameter="RmsMix"
                size={96}
              />
              <TimeKnob
                label="RMSLEN"
                minValue={0.001}
                maxValue={0.03}
                defaultValue={0.01}
                range={new NormalisableRange(0.001, 0.03, 0.015)}
                parameter="RmsBufferSize"
              />
              {/*  TODO: dynamically set min/max on this */}
              <TimeKnob
                label="LKAHD"
                minValue={0}
                maxValue={0.03}
                defaultValue={0}
                range={new NormalisableRange(0, 0.03, 0.015)}
                parameter="Lookahead"
              />
            </div>
          </div>
        </div>
      </div>
    </main>
  );
}
