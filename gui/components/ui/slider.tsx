'use client';

import * as React from 'react';
import * as SliderPrimitive from '@radix-ui/react-slider';

import { cn } from '@/lib/utils';

const Slider = React.forwardRef<
	React.ElementRef<typeof SliderPrimitive.Root>,
	React.ComponentPropsWithoutRef<typeof SliderPrimitive.Root>
>(({ className, ...props }, ref) => (
	<SliderPrimitive.Root
		ref={ref}
		className={cn('relative flex w-full touch-none select-none', className)}
		orientation='vertical'
		{...props}
	>
		<SliderPrimitive.Track className='relative w-2 h-72 overflow-hidden bg-secondary'>
			<SliderPrimitive.Range className='absolute h-full bg-primary' />
		</SliderPrimitive.Track>
		<SliderPrimitive.Thumb className='block h-2 w-5 border-2 border-primary bg-background ring-offset-background transition-colors focus-visible:outline-none disabled:pointer-events-none disabled:opacity-50' />
	</SliderPrimitive.Root>
));
Slider.displayName = SliderPrimitive.Root.displayName;

export { Slider };
