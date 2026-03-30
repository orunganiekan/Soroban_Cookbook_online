import React, { type ReactNode } from 'react';
import clsx from 'clsx';
import { Info, AlertTriangle, AlertCircle, CheckCircle, Lightbulb } from 'lucide-react';
import styles from './Callout.module.css';

export type CalloutVariant = 'info' | 'warning' | 'error' | 'success' | 'tip';

export interface CalloutProps {
  variant?: CalloutVariant;
  title?: string;
  children: ReactNode;
  icon?: ReactNode | boolean;
  className?: string;
}

const variantIcons: Record<CalloutVariant, React.ComponentType<{ size?: number }>> = {
  info: Info,
  warning: AlertTriangle,
  error: AlertCircle,
  success: CheckCircle,
  tip: Lightbulb,
};

const defaultTitles: Record<CalloutVariant, string> = {
  info: 'Info',
  warning: 'Warning',
  error: 'Error',
  success: 'Success',
  tip: 'Tip',
};

/**
 * Callout Component
 * -----------------
 * A specialized alert component for documentation pages.
 * Provides visual emphasis for important guidance, tips, warnings, and cautions.
 *
 * @example
 * <Callout variant="tip">
 *   Use this pattern when you need to optimize gas costs.
 * </Callout>
 *
 * <Callout variant="warning" title="Breaking Change">
 *   This API will be deprecated in version 2.0
 * </Callout>
 */
export default function Callout({
  variant = 'info',
  title,
  children,
  icon = true,
  className,
}: CalloutProps) {
  const IconComponent = variantIcons[variant];
  const showIcon = icon !== false;
  const displayTitle = title || defaultTitles[variant];

  return (
    <aside
      className={clsx(styles.callout, styles[`callout--${variant}`], className)}
      role="note"
      aria-label={displayTitle}>
      <div className={styles.callout__header}>
        {showIcon && (
          <div className={styles.callout__icon}>
            {typeof icon === 'boolean' ? <IconComponent size={20} /> : icon}
          </div>
        )}
        <div className={styles.callout__title}>{displayTitle}</div>
      </div>
      <div className={styles.callout__content}>{children}</div>
    </aside>
  );
}
