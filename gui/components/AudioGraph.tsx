import type { Amplitude as AmplitudeMessage } from "@/bindings/Amplitude";
import { useAmplitudeUpdate, useDecayFactor, useSampleRate } from "@/hooks";
import { gainToDb } from "@/lib/utils";
import { MutableRefObject, useEffect, useRef, useState } from "react";

// TODO:
// documentation/comments
// REFACTOR!

export function AudioGraph(props: {
  dryWet: number;
  threshold: number;
  knee: number;
}) {
  const { dryWet, threshold, knee } = props;

  const decayFactor = useDecayFactor(100);

  // canvas configuration
  const meterWidth = 160;
  const meterHeight = meterWidth;
  const bufferSize = 450;
  const canvasRef = useRef<HTMLCanvasElement | null>(null);

  // set up buffers, refs, etc.
  const preAmplitude = useRef(0);
  const postAmplitude = useRef(0);

  const { pre, post } = useAmplitudeUpdate();
  // add decay over time
  if (pre < preAmplitude.current) {
    preAmplitude.current =
      preAmplitude.current * decayFactor + pre * (1.0 - decayFactor);
  } else {
    preAmplitude.current = pre;
  }

  if (post < postAmplitude.current) {
    postAmplitude.current =
      postAmplitude.current * decayFactor + post * (1.0 - decayFactor);
  } else {
    postAmplitude.current = post;
  }

  const preAmplitudeBuffer = useRef(new Array(bufferSize).fill(0));
  const postAmplitudeBuffer = useRef(new Array(bufferSize).fill(0));

  useEffect(() => {
    let animationRequest: number;
    // TODO: deal with !
    const canvas = canvasRef.current!;
    const ctx = canvas.getContext("2d")!;

    // optional
    ctx.imageSmoothingEnabled = true;
    ctx.imageSmoothingQuality = "high";

    const draw = () => {
      // of course, start with a clean slate
      ctx.clearRect(0, 0, canvas.width, canvas.height);

      // update buffers
      preAmplitudeBuffer.current.push(preAmplitude.current);
      preAmplitudeBuffer.current.shift();

      postAmplitudeBuffer.current.push(postAmplitude.current);
      postAmplitudeBuffer.current.shift();

      // draw pre-processed
      drawGain(
        ctx,
        meterWidth,
        meterHeight,
        preAmplitudeBuffer,
        `rgba(113, 197, 156, ${Math.max(1 - dryWet, 0.5)})`
      );
      // draw post-processed
      drawGain(
        ctx,
        meterWidth,
        meterHeight,
        postAmplitudeBuffer,
        `rgba(180, 39, 112, ${Math.max(dryWet, 0.25)})`
      );
      // add threshold line
      drawThresholdLine(ctx, threshold, meterWidth, meterHeight);
      drawKnee(ctx, threshold, knee, meterWidth, meterHeight);
      animationRequest = requestAnimationFrame(draw);
    };
    animationRequest = requestAnimationFrame(draw);
    // cleanup
    return () => {
      cancelAnimationFrame(animationRequest);
    };
  }, [dryWet, threshold, knee]);

  return (
    <div>
      <canvas
        ref={canvasRef}
        width={meterWidth}
        height={meterHeight}
        className="border-2 border-gray-800"
      ></canvas>
    </div>
  );
}

function drawGain(
  ctx: CanvasRenderingContext2D,
  width: number,
  height: number,
  pointBuffer: MutableRefObject<number[]>,
  color: string
) {
  ctx.beginPath();
  ctx.moveTo(0, height);

  const bufferSize = pointBuffer.current.length;
  for (let i = 0; i < bufferSize; i++) {
    const x = (width / bufferSize) * i;
    const y = -(gainToDb(pointBuffer.current[i]) * height) / 100;

    ctx.lineTo(x, y);
  }

  // Draw back to the bottom-right corner to complete the filled area
  ctx.lineTo(width, height);
  ctx.closePath();

  ctx.fillStyle = color; // Replace with your desired solid color
  ctx.fill();
}

function drawThresholdLine(
  ctx: CanvasRenderingContext2D,
  threshold: number,
  width: number,
  height: number
) {
  ctx.fillStyle = "black";

  // optionally, add some text
  // i think it looks to busy though

  //ctx.font = "8px Arial";
  // ctx.fillText("Thresh", 0 + 2.5, -threshold);
  ctx.fillRect(0, (-threshold * height) / 100, width, 1);
}

function drawKnee(
  ctx: CanvasRenderingContext2D,
  threshold: number,
  knee: number,
  width: number,
  height: number
) {
  ctx.fillStyle = "rgba(0,0,0,0.2)";

  // optionally, add some text
  // i think it looks to busy though

  //ctx.font = "8px Arial";
  // ctx.fillText("Thresh", 0 + 2.5, -threshold);
  ctx.fillRect(
    0,
    ((-threshold - knee) * height) / 100,
    width,
    (knee * 2 * height) / 100
  );
}
