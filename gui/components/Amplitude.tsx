import { Amplitude as Amp } from "@/bindings/Amplitude";
import { useEffect, useRef, useState } from "react";

export function Amplitude() {
  const [amplitude, setAmplitude] = useState(0);
  const [postAmplitude, setPostAmplitude] = useState(0);
  const canvasRef = useRef(null);

  const meterWidth = 144; // Width of the meter (adjust as needed)
  const meterHeight = 144; // Height of the meter (adjust as needed)
  const bufferSize = 50; // Number of data points to keep in the buffer

  const amplitudeBuffer = useRef(new Array(bufferSize).fill(0));
  const postAmplitudeBuffer = useRef(new Array(bufferSize).fill(0));

  // update state based on incoming messages
  useEffect(() => {
    // NOTE:
    // here's im using `any` because addEventListener will complain otherwise
    const handlePluginMessage = (event: any) => {
      let message: Amp = event.detail;
      setAmplitude(message.pre_amplitude);
      setPostAmplitude(message.post_amplitude);
    };

    window.addEventListener("pluginMessage", handlePluginMessage);
    return () => {
      window.removeEventListener("pluginMessage", handlePluginMessage);
    };
  }, []);
  useEffect(() => {
    const canvas = canvasRef.current;
    const ctx = canvas.getContext("2d");

    // of course, start with a clean slate
    ctx.clearRect(0, 0, canvas.width, canvas.height);

    // optional
    // ctx.imageSmoothingEnabled = true;
    // ctx.imageSmoothingQuality = "high";

    // update buffers
    amplitudeBuffer.current.push(amplitude);
    amplitudeBuffer.current.shift();

    postAmplitudeBuffer.current.push(postAmplitude);
    postAmplitudeBuffer.current.shift();

    ctx.beginPath();
    ctx.moveTo(0, meterHeight); // Start from bottom-left corner

    for (let i = 0; i < amplitudeBuffer.current.length; i++) {
      const x = (meterWidth / bufferSize) * i;
      const y = meterHeight - meterHeight * amplitudeBuffer.current[i];

      ctx.lineTo(x, y);
    }

    // Draw back to the bottom-right corner to complete the filled area
    ctx.lineTo(meterWidth, meterHeight);
    ctx.closePath();

    ctx.fillStyle = "rgba(92, 92, 92, 0.5)"; // Replace with your desired solid color
    ctx.fill();

    ctx.beginPath();
    ctx.moveTo(0, meterHeight); // Start from bottom-left corner

    for (let i = 0; i < postAmplitudeBuffer.current.length; i++) {
      const x = (meterWidth / bufferSize) * i;
      const y = meterHeight - meterHeight * postAmplitudeBuffer.current[i];

      ctx.lineTo(x, y);
    }

    // Draw back to the bottom-right corner to complete the filled area
    ctx.lineTo(meterWidth, meterHeight);
    ctx.closePath();

    ctx.fillStyle = "rgba(92, 92, 92, 0.5)"; // Replace with your desired solid color
    ctx.fill();
  }, [amplitude]);

  // bleh
  return (
    <div className="bg-white">
      <canvas ref={canvasRef} width={meterWidth} height={meterHeight}></canvas>
    </div>
  );
}
