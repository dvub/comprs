import { getDecay, useAmplitudeUpdate, useSampleRate } from "@/hooks";
import { gainToDb } from "@/lib/utils";
import { useEffect, useRef, useState } from "react";

export function GRMeter() {
  const width = 20;
  const height = 144;

  const gr = useRef(0);
  const { pre, post } = useAmplitudeUpdate();

  let newDb = 0;
  if (pre > 0 && post > 0) {
    newDb = gainToDb(pre) - gainToDb(post);
  }

  const decayFactor = getDecay(100);
  if (newDb < gr.current) {
    gr.current = gr.current * decayFactor + newDb * (1.0 - decayFactor);
  } else {
    gr.current = newDb;
  }

  // TODO:
  // normalize GR!!

  const canvasRef = useRef<HTMLCanvasElement | null>(null);

  useEffect(() => {
    let animationRequest: number;
    // TODO: deal with !
    const canvas = canvasRef.current!;
    const ctx = canvas.getContext("2d")!;

    const draw = () => {
      ctx.clearRect(0, 0, width, height);

      ctx.fillStyle = "#b42770";
      ctx.fillRect(0, 0, width, gr.current);

      animationRequest = requestAnimationFrame(draw);
    };
    animationRequest = requestAnimationFrame(draw);

    // cleanup
    return () => {
      cancelAnimationFrame(animationRequest);
    };
  }, []);

  return (
    <div className="relative border-2 border-gray-800">
      <canvas ref={canvasRef} width={width} height={height}></canvas>
    </div>
  );
}
