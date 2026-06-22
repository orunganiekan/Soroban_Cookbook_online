import React from 'react';
import styles from './OptimizedImage.module.css';

interface OptimizedImageProps {
  src: string;
  alt: string;
  width?: number;
  height?: number;
  className?: string;
  loading?: 'lazy' | 'eager';
  decoding?: 'async' | 'sync' | 'auto';
}

export default function OptimizedImage({
  src,
  alt,
  width,
  height,
  className,
  loading = 'lazy',
  decoding = 'async',
}: OptimizedImageProps) {
  // Generate WebP version of the image
  const webpSrc = src.replace(/\.(jpg|jpeg|png)$/i, '.webp');
  
  return (
    <picture className={className}>
      <source srcSet={webpSrc} type="image/webp" />
      <source srcSet={src} type={`image/${src.split('.').pop()}`} />
      <img
        src={src}
        alt={alt}
        width={width}
        height={height}
        loading={loading}
        decoding={decoding}
      />
    </picture>
  );
}
