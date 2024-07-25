"use client";
// TODO: ADD DOCS

import { TimeKnob } from "@/components/knobs/generic/TimeKnob";
import { RatioKnob } from "@/components/knobs/RatioKnob";
import { ThresholdKnob } from "@/components/knobs/ThresholdKnob";
import { NormalisableRange } from "@/lib/utils";
import React, { useEffect, useState } from "react";

import { KneeKnob } from "@/components/knobs/KneeKnob";
import { DBKnob } from "@/components/knobs/generic/DBKnob";
import { sendToPlugin } from "@/lib";
import { PercentKnob } from "@/components/knobs/generic/PercentKnob";
import { Message } from "@/bindings/Messages";
import { AudioGraph } from "@/components/AudioGraph";
import { GRMeter } from "@/components/GRMeter";
import { useSampleRate } from "@/hooks";

export default function Home() {
  const a = useSampleRate();

  useEffect(() => {
    window.onPluginMessage = (message: Message) => {
      const event = new CustomEvent("pluginMessage", { detail: message });
      window.dispatchEvent(event);
    };

    // TODO:
    // prevent knobs from setting themselves after a delay... really annoying and jarring
    sendToPlugin("Init");
  }, []);

  const [dryWet, setDryWet] = useState(0);
  const [rmsMix, setRmsMix] = useState(0);
  const [threshold, setThreshold] = useState(0);
  const [knee, setKnee] = useState(0);
  return (
    <main className="relative main-bg w-screen h-screen overflow-hidden px-3 py-5 text-[#180619] ">
      <p className="absolute text-xs bottom-0 right-0 opacity-50">
        {a.toFixed(2)} TPS
      </p>
      <h1 className="absolute bottom-0 left-0">COMPRS vX.0</h1>

      <div className="flex justify-center items-center h-[25%] gap-3">
        <TimeKnob
          label="ATK"
          minValue={0}
          maxValue={1}
          defaultValue={0.001}
          range={new NormalisableRange(0, 1, 0.01)}
          parameter="AttackTime"
        />
        <KneeKnob value={knee} setValue={setKnee} />
        <TimeKnob
          label="RLS"
          minValue={0}
          maxValue={5}
          defaultValue={0.05}
          range={new NormalisableRange(0, 5, 0.5)}
          parameter="ReleaseTime"
        />
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
        <div className="flex gap-3 justify-center items-center">
          <ThresholdKnob value={threshold} setValue={setThreshold} />
          {/* WOWOWOWOWOW */}
          <AudioGraph dryWet={dryWet} threshold={threshold} knee={knee} />
          <GRMeter />
          <RatioKnob />
        </div>
        {/* this div contains output-related knobs */}
        <div className="text-center ">
          <p>OUT CTRL</p>
          <PercentKnob
            label="DRYWET"
            size={96}
            minValue={0}
            maxValue={1}
            defaultValue={1}
            range={new NormalisableRange(0, 1, 0.5)}
            parameter="DryWet"
            value={dryWet}
            setValue={setDryWet}
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
                value={rmsMix}
                setValue={setRmsMix}
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
