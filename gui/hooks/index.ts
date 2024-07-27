import { Amplitude } from "@/bindings/Amplitude";
import { MutableRefObject, useEffect, useRef, useState } from "react";

// TODO:
// there's probably a better way to do this
function isAmplitudeMessage(message: any): message is Amplitude {
  return (message as Amplitude).pre_amplitude !== undefined;
}
/*



function useDecay() {
      const decayMs: number = 100;
  const calculateDecay = (sampleRate: number) => {
    return Math.pow(0.25, 1 / (sampleRate * (decayMs / 1000.0)));
  };
  const sampleRate = useSampleRate();
  const decayFactor = calculateDecay(sampleRate);

  if (newPreAmplitude < preAmplitude) {
    preAmplitude =
      currentPreAmplitude * decayFactor + newPreAmplitude * (1.0 - decayFactor);
  }

  if (newPostAmplitude < currentPostAmplitude) {
    newPostAmplitude =
      currentPostAmplitude * decayFactor +
      newPostAmplitude * (1.0 - decayFactor);
  }

}
*/

// TODO:
// move this??
export function useDecayFactor(decayMs: number) {
  const sampleRate = useSampleRate();
  // TODO:
  // this Math.min(60) might be weird and not necessary
  return Math.pow(0.25, 1 / (Math.min(60, sampleRate) * (decayMs / 1000.0)));
}

export function useAmplitudeUpdate() {
  const [amplitudes, setAmplitudes] = useState({ pre: 0, post: 0, reduced: 0 });
  // update state based on incoming messages
  useEffect(() => {
    // NOTE:
    // here's im using `any` because addEventListener will complain otherwise
    const handlePluginMessage = (event: any) => {
      // make sure the event type is actually correct
      const message = event.detail;
      if (!isAmplitudeMessage(message)) {
        return;
      }
      setAmplitudes({
        pre: message.pre_amplitude,
        post: message.post_amplitude,
        reduced: message.amt_reduced,
      });
    };

    window.addEventListener("pluginMessage", handlePluginMessage);
    return () => {
      window.removeEventListener("pluginMessage", handlePluginMessage);
    };
  }, []);
  return amplitudes;
}

// Custom hook which keeps track of front-end sample rate.
export function useSampleRate() {
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
