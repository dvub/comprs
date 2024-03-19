'use client';

import Image from 'next/image';
import { Slider } from '@/components/ui/slider';
import { Knob, Value, Pointer, Arc } from 'rc-knob';
import React from 'react';
export default function Home() {
	const [threshold, setThreshold] = React.useState<number>(0);
	return (
		<main className='m-3 border-2 rounded-md border-gray-200 '>
			<div className='m-3'>
				<h1 className='w-full text-center'>COMPRS</h1>
				<div className='PARAMS flex justify-between'>
					<div className='THRESHOLD'>
						<p>Threshold</p>
						<Slider
							defaultValue={[threshold]}
							max={10}
							min={-100}
							step={0.1}
							onValueChange={(e: number[]) => setThreshold(e[0])}
						></Slider>
						<p>{threshold} dB</p>
					</div>
					<div className='RATIO'>
						<p className='text-center'>Ratio</p>

						<Knob
							size={100}
							angleOffset={220}
							angleRange={280}
							min={0}
							max={100}
						>
							<Arc arcWidth={5} color='#FC5A96' />
							<Pointer
								width={5}
								height={40}
								radius={10}
								type='rect'
								color='#FC5A96'
							/>
						</Knob>
					</div>
					<div className='ATTACK'>
						<p className='text-center'>Attack</p>

						<Knob
							size={100}
							angleOffset={220}
							angleRange={280}
							min={0}
							max={100}
						>
							<Arc arcWidth={5} color='#FC5A96' />
							<Pointer
								width={5}
								height={40}
								radius={10}
								type='rect'
								color='#FC5A96'
							/>
						</Knob>
					</div>
				</div>
			</div>
		</main>
	);
}
