import React from 'react';
import clsx from 'clsx';
import styles from './PatternDoc.module.css';
import { Badge, Tag } from '../Badge';
import type { BadgeDifficulty, BadgeStatus, TagCategory } from '../Badge';

export type PatternMetaProps = {
  slug: string;
  difficulty: BadgeDifficulty;
  category: TagCategory | string;
  status?: BadgeStatus;
  lastReviewed?: string;
};

export function PatternMeta({
  slug,
  difficulty,
  category,
  status = 'stable',
  lastReviewed,
}: PatternMetaProps) {
  return (
    <div className={styles.metaCard} data-pattern-slug={slug}>
      <div className={styles.metaHeader}>
        <Badge variant={difficulty} size="md" asStatus />
        {status && status !== 'stable' && <Badge variant={status} size="sm" />}
      </div>
      <dl className={styles.metaGrid}>
        <div>
          <dt>Category</dt>
          <dd>{category}</dd>
        </div>
        <div>
          <dt>Pattern ID</dt>
          <dd>
            <code>{slug}</code>
          </dd>
        </div>
        <div>
          <dt>Status</dt>
          <dd>{status}</dd>
        </div>
        {lastReviewed ? (
          <div>
            <dt>Last reviewed</dt>
            <dd>{lastReviewed}</dd>
          </div>
        ) : null}
      </dl>
    </div>
  );
}

export function PatternSection({
  id,
  title,
  children,
}: {
  id: string;
  title: string;
  children: React.ReactNode;
}) {
  const headingId = `${id}-heading`;
  return (
    <section id={id} className={styles.section} aria-labelledby={headingId}>
      <h2 className={styles.sectionTitle} id={headingId}>
        {title}
      </h2>
      <div className={styles.sectionBody}>{children}</div>
    </section>
  );
}

export function PatternCallout({
  variant = 'info',
  title,
  children,
}: {
  variant?: 'info' | 'warning' | 'danger' | 'success';
  title?: string;
  children: React.ReactNode;
}) {
  return (
    <aside className={clsx(styles.callout, styles[`callout${variant}`])} role="note">
      {title ? <p className={styles.calloutTitle}>{title}</p> : null}
      <div>{children}</div>
    </aside>
  );
}
