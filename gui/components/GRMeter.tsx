import { useDecayFactor as useDecayFactor, useAmplitudeUpdate } from "@/hooks";
import { gainToDb } from "@/lib/utils";
import { useEffect, useRef } from "react";

export function GRMeter() {
  // normalization factor
  // the meter will be full if the amt reduced is >= 100 dB
  const maxDB = 100;

  // dimensions of canvas
  const width = 20;
  const height = 160;

  const gr = useRef(0);
  // use our custom hooks
  const { pre, post } = useAmplitudeUpdate();
  // TODO:
  // should this be tied to our attack parameter?
  const decayFactor = useDecayFactor(100);

  // TODO:
  // should this calculations be inside of a useEffect?
  // this is kind of messy too
  let newDifference = 0;
  if (pre > 0 && post > 0) {
    newDifference = gainToDb(pre) - gainToDb(post);
  }

  // add decay over time
  if (newDifference < gr.current) {
    gr.current = gr.current * decayFactor + newDifference * (1.0 - decayFactor);
  } else {
    gr.current = newDifference;
  }

  const canvasRef = useRef<HTMLCanvasElement | null>(null);

  useEffect(() => {
    let animationRequest: number;
    // TODO: deal with !
    const canvas = canvasRef.current!;
    const ctx = canvas.getContext("2d")!;

    const draw = () => {
      ctx.clearRect(0, 0, width, height);

      ctx.fillStyle = "#b42770";
      // here is where we actually apply the normalization
      ctx.fillRect(0, 0, width, Math.min((gr.current * height) / maxDB, maxDB));

      animationRequest = requestAnimationFrame(draw);
    };
    animationRequest = requestAnimationFrame(draw);

    // cleanup
    return () => {
      cancelAnimationFrame(animationRequest);
    };
  }, []);

  return (
    <div className="text-center text-xs">
      <p>GR</p>
      <canvas
        ref={canvasRef}
        width={width}
        height={height}
        className="border-2 border-gray-800"
      ></canvas>
      <p className="">{Math.round(gr.current)}</p>
    </div>
  );
}
