import { useDecayFactor as useDecayFactor, useAmplitudeUpdate } from "@/hooks";
import { gainToDb } from "@/lib/utils";
import { useEffect, useRef } from "react";

export function GRMeter() {
  // canvas setup
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  // dimensions of canvas
  const width = 20;
  const height = 160;

  // normalization factor
  // the meter will be full if the amt reduced is >= 100 dB
  const maxDB = 100;

  const gr = useRef(0);
  // use our custom hooks
  const { reduced } = useAmplitudeUpdate();
  gr.current = Math.abs(gainToDb(reduced));

  // now for the drawing stuff
  useEffect(() => {
    let animationRequest: number;
    // TODO: deal with !
    const canvas = canvasRef.current!;
    const ctx = canvas.getContext("2d")!;

    const draw = () => {
      ctx.clearRect(0, 0, width, height);

      ctx.fillStyle = "#b42770";

      // here is where we actually apply the normalization
      const h = Math.min((gr.current * height) / maxDB, maxDB);
      ctx.fillRect(0, 0, width, h);

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
