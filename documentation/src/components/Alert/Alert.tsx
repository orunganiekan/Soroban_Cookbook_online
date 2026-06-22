import React, { type ReactNode } from 'react';
import clsx from 'clsx';
import { Info, AlertTriangle, AlertCircle, CheckCircle } from 'lucide-react';
import styles from './Alert.module.css';

export type AlertVariant = 'info' | 'warning' | 'error' | 'success';
export type AlertDisplay = 'block' | 'inline';

export interface AlertProps {
  variant?: AlertVariant;
  display?: AlertDisplay;
  title?: string;
  children: ReactNode;
  icon?: ReactNode | boolean;
  className?: string;
  onClose?: () => void;
}

const variantIcons: Record<AlertVariant, React.ComponentType<{ size?: number }>> = {
  info: Info,
  warning: AlertTriangle,
  error: AlertCircle,
  success: CheckCircle,
};

/**
 * Alert Component
 * ---------------
 * Displays important messages with visual emphasis for info, warning, error, and success states.
 * Supports both block and inline display patterns with optional icons and titles.
 *
 * @example
 * <Alert variant="info">This is an informational message</Alert>
 * <Alert variant="warning" title="Warning">Be careful with this action</Alert>
 * <Alert variant="error" icon={false}>Error without icon</Alert>
 */
export default function Alert({
  variant = 'info',
  display = 'block',
  title,
  children,
  icon = true,
  className,
  onClose,
}: AlertProps) {
  const IconComponent = variantIcons[variant];
  const showIcon = icon !== false;

  return (
    <div
      className={clsx(
        styles.alert,
        styles[`alert--${variant}`],
        styles[`alert--${display}`],
        className,
      )}
      role="alert"
      aria-live={variant === 'error' ? 'assertive' : 'polite'}>
      {showIcon && (
        <div className={styles.alert__icon}>
          {typeof icon === 'boolean' ? <IconComponent size={20} /> : icon}
        </div>
      )}
      <div className={styles.alert__content}>
        {title && <div className={styles.alert__title}>{title}</div>}
        <div className={styles.alert__body}>{children}</div>
      </div>
      {onClose && (
        <button
          className={styles.alert__close}
          onClick={onClose}
          aria-label="Close alert"
          type="button">
          ×
        </button>
      )}
    </div>
  );
}
