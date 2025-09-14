'use client';

import { type Variants, motion, useAnimation } from 'motion/react';
import type { HTMLAttributes } from 'react';
import { forwardRef, useCallback, useImperativeHandle, useRef } from 'react';
import { cn } from '@/lib/utils';

export interface BatteryFullIconHandle {
  startAnimation: () => void;
  stopAnimation: () => void;
}

interface BatteryFullIconProps extends HTMLAttributes<HTMLDivElement> {
  size?: number;
}

const lineVariants: Variants = {
  initial: { opacity: 1 },
  fadeOut: {
    opacity: 0,
    transition: {
      duration: 0.4,
      ease: 'easeInOut',
    },
  },
  fadeIn: (i: number) => ({
    opacity: 1,
    transition: {
      duration: 0.6,
      delay: i * 0.4,
      ease: 'easeInOut',
    },
  }),
};

const BatteryFullIcon = forwardRef<BatteryFullIconHandle, BatteryFullIconProps>(
  ({ onMouseEnter, onMouseLeave, className, size = 28, ...props }, ref) => {
    const controls = useAnimation();
    const isControlledRef = useRef(false);

    useImperativeHandle(ref, () => {
      isControlledRef.current = true;

      return {
        startAnimation: async () => {
          await controls.start('fadeOut');
          controls.start('fadeIn');
        },
        stopAnimation: () => controls.start('initial'),
      };
    });

    const handleMouseEnter = useCallback(
      async (e: React.MouseEvent<HTMLDivElement>) => {
        if (!isControlledRef.current) {
          await controls.start('fadeOut');
          controls.start('fadeIn');
        } else {
          onMouseEnter?.(e);
        }
      },
      [controls, onMouseEnter]
    );

    const handleMouseLeave = useCallback(
      (e: React.MouseEvent<HTMLDivElement>) => {
        if (!isControlledRef.current) {
          controls.start('initial');
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
        <motion.svg
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
          <rect width="16" height="10" x="2" y="7" rx="2" ry="2" />
          <line x1="22" x2="22" y1="11" y2="13" />
          <motion.line
            x1="6"
            x2="6"
            y1="11"
            y2="13"
            variants={lineVariants}
            initial="initial"
            animate={controls}
            custom={0}
          />
          <motion.line
            x1="10"
            x2="10"
            y1="11"
            y2="13"
            variants={lineVariants}
            initial="initial"
            animate={controls}
            custom={1}
          />
          <motion.line
            x1="14"
            x2="14"
            y1="11"
            y2="13"
            variants={lineVariants}
            initial="initial"
            animate={controls}
            custom={2}
          />
        </motion.svg>
      </div>
    );
  }
);

BatteryFullIcon.displayName = 'BatteryFullIcon';

export { BatteryFullIcon };
