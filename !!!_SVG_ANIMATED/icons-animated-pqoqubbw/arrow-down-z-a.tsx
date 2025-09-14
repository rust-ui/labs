'use client';

import type { Transition, Variants } from 'motion/react';
import { motion, useAnimation } from 'motion/react';
import type { HTMLAttributes } from 'react';
import { forwardRef, useCallback, useImperativeHandle, useRef } from 'react';
import { cn } from '@/lib/utils';

export interface ArrowDownZAIconHandle {
  startAnimation: () => void;
  stopAnimation: () => void;
}

interface ArrowDownZAIconProps extends HTMLAttributes<HTMLDivElement> {
  size?: number;
}

const swapTransition: Transition = {
  type: 'spring',
  stiffness: 240,
  damping: 24,
};

const swapVariants: Variants = {
  normal: {
    translateY: 0,
  },
  animate: (custom: number) => ({
    translateY: custom * 10,
  }),
};

const ArrowDownZAIcon = forwardRef<ArrowDownZAIconHandle, ArrowDownZAIconProps>(
  ({ onMouseEnter, onMouseLeave, className, size = 28, ...props }, ref) => {
    const controls = useAnimation();
    const isControlledRef = useRef(false);

    useImperativeHandle(ref, () => {
      isControlledRef.current = true;

      return {
        startAnimation: () => controls.start('animate'),
        stopAnimation: () => controls.start('normal'),
      };
    });

    const handleMouseEnter = useCallback(
      (e: React.MouseEvent<HTMLDivElement>) => {
        if (!isControlledRef.current) {
          controls.start('animate');
        } else {
          onMouseEnter?.(e);
        }
      },
      [controls, onMouseEnter]
    );

    const handleMouseLeave = useCallback(
      (e: React.MouseEvent<HTMLDivElement>) => {
        if (!isControlledRef.current) {
          controls.start('normal');
        } else {
          onMouseLeave?.(e);
        }
      },
      [controls, onMouseLeave]
    );

    return (
      <div
        className={cn(className)}
        onMouseEnter={handleMouseEnter}
        onMouseLeave={handleMouseLeave}
        {...props}
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width={size}
          height={size}
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          strokeWidth="2"
          strokeLinecap="round"
          strokeLinejoin="round"
        >
          <path d="m3 16 4 4 4-4" />
          <path d="M7 20V4" />
          <motion.path
            d="M15 4h5l-5 6h5"
            variants={swapVariants}
            initial="normal"
            animate={controls}
            custom={1}
            transition={swapTransition}
          />
          <motion.g
            variants={swapVariants}
            initial="normal"
            animate={controls}
            custom={-1}
            transition={swapTransition}
          >
            <path d="M20 18h-5" />
            <path d="M15 20v-3.5a2.5 2.5 0 0 1 5 0V20" />
          </motion.g>
        </svg>
      </div>
    );
  }
);

ArrowDownZAIcon.displayName = 'ArrowDownZAIcon';

export { ArrowDownZAIcon };
