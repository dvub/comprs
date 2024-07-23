import type { Amplitude as AmplitudeMessage } from "@/bindings/Amplitude";
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

  // canvas configuration
  const meterWidth = 144;
  const meterHeight = 144;
  const bufferSize = 450;
  const canvasRef = useRef<HTMLCanvasElement | null>(null);

  // set up buffers, refs, etc.
  const preAmplitude = useRef(0);
  const postAmplitude = useRef(0);

  const preAmplitudeBuffer = useRef(new Array(bufferSize).fill(0));
  const postAmplitudeBuffer = useRef(new Array(bufferSize).fill(0));
  // TODO: BETTER NAME HERE
  useAmplitudeUpdate(preAmplitude, postAmplitude);

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
        `rgba(150, 150, 150, ${Math.max(1 - dryWet, 0.15)})`
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
      drawThresholdLine(ctx, threshold, meterWidth);
      drawKnee(ctx, threshold, knee, meterWidth);
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
      <canvas ref={canvasRef} width={meterWidth} height={meterHeight}></canvas>
    </div>
  );
}

function useAmplitudeUpdate(
  amplitude: MutableRefObject<number>,
  postAmplitude: MutableRefObject<number>
) {
  const sampleRate = useSampleRate();
  // update state based on incoming messages
  useEffect(() => {
    // NOTE:
    // here's im using `any` because addEventListener will complain otherwise
    const handlePluginMessage = (event: any) => {
      // make sure the event type is actually correctly
      const message = event.detail;
      if (!isAmplitudeMessage(message)) {
        return;
      }
      const currentPreAmplitude = amplitude.current;
      const currentPostAmplitude = postAmplitude.current;
      let newPreAmplitude = message.pre_amplitude;
      let newPostAmplitude = message.post_amplitude;

      const decayMs: number = 100;
      const calculateDecay = (sampleRate: number) => {
        return Math.pow(0.25, 1 / (sampleRate * (decayMs / 1000.0)));
      };
      const decayFactor = calculateDecay(sampleRate);

      if (newPreAmplitude < currentPreAmplitude) {
        newPreAmplitude =
          currentPreAmplitude * decayFactor +
          newPreAmplitude * (1.0 - decayFactor);
      }

      if (newPostAmplitude < currentPostAmplitude) {
        newPostAmplitude =
          currentPostAmplitude * decayFactor +
          newPostAmplitude * (1.0 - decayFactor);
      }

      amplitude.current = newPreAmplitude;
      postAmplitude.current = newPostAmplitude;
    };

    window.addEventListener("pluginMessage", handlePluginMessage);
    return () => {
      window.removeEventListener("pluginMessage", handlePluginMessage);
    };
  }, [sampleRate]);
}

// Custom hook which keeps track of front-end sample rate.
function useSampleRate() {
  const initTime = useRef(Date.now());

  const [totalEvents, setTotalEvents] = useState(0);
  const [elapsedTime, setElapsedTime] = useState(0);
  const [sampleRate, setSampleRate] = useState(0);

  useEffect(() => {
    // TODO:
    // only update event count when event is an amplitude event, not a parameter update, etc.
    const handlePluginMessage = (event: any) => {
      const message = event.detail;
      if (!isAmplitudeMessage(message)) {
        return;
      }
      setTotalEvents((prev) => prev + 1);
      setElapsedTime((Date.now() - initTime.current) / 1000);

      setSampleRate(totalEvents / elapsedTime);
    };
    window.addEventListener("pluginMessage", handlePluginMessage);
    return () => {
      window.removeEventListener("pluginMessage", handlePluginMessage);
    };
  }, [totalEvents]);
  // keeps numbers from getting too large, probably not necessary
  useEffect(() => {
    if (elapsedTime > 10) {
      setTotalEvents(0);
      initTime.current = Date.now();
    }
  }, [elapsedTime]);

  return sampleRate;
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
    const y = -gainToDb(pointBuffer.current[i]);

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
  width: number
) {
  ctx.fillStyle = "black";

  // optionally, add some text
  // i think it looks to busy though

  //ctx.font = "8px Arial";
  // ctx.fillText("Thresh", 0 + 2.5, -threshold);
  ctx.fillRect(0, -threshold, width, 1);
}

function drawKnee(
  ctx: CanvasRenderingContext2D,
  threshold: number,
  knee: number,
  width: number
) {
  ctx.fillStyle = "rgba(0,0,0,0.5)";

  // optionally, add some text
  // i think it looks to busy though

  //ctx.font = "8px Arial";
  // ctx.fillText("Thresh", 0 + 2.5, -threshold);
  ctx.fillRect(0, -threshold + knee, width, 1);
  ctx.fillRect(0, -threshold - knee, width, 1);
}

// TODO:
// there's probably a better way to do this
function isAmplitudeMessage(message: any): message is AmplitudeMessage {
  return (message as AmplitudeMessage).pre_amplitude !== undefined;
}
