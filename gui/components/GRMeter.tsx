import { useAmplitudeUpdate } from "@/hooks";
import { gainToDb } from "@/lib/utils";
import { useEffect, useRef, useState } from "react";

export function GRMeter() {
  const width = 20;
  const height = 144;

  // set up buffers, refs, etc.
  const preAmplitude = useRef(0);
  const postAmplitude = useRef(0);
  const [gr, setGr] = useState(0);
  // TODO: BETTER NAME HERE
  useAmplitudeUpdate(preAmplitude, postAmplitude);
  const canvasRef = useRef<HTMLCanvasElement | null>(null);

  useEffect(() => {
    let animationRequest: number;
    // TODO: deal with !
    const canvas = canvasRef.current!;
    const ctx = canvas.getContext("2d")!;

    const draw = () => {
      const reducedAmplitude =
        gainToDb(preAmplitude.current) - gainToDb(postAmplitude.current);
      setGr(reducedAmplitude);
      ctx.clearRect(0, 0, width, height);
      console.log(reducedAmplitude);
      ctx.fillStyle = "green";
      ctx.fillRect(0, 0, width, reducedAmplitude);

      animationRequest = requestAnimationFrame(draw);
    };
    animationRequest = requestAnimationFrame(draw);

    // cleanup
    return () => {
      cancelAnimationFrame(animationRequest);
    };
  }, []);

  return (
    <div className="relative">
      <p className="text-xs">GR: {gr.toFixed(2)} dB</p>
      <canvas ref={canvasRef} width={width} height={height}></canvas>
    </div>
  );
}
