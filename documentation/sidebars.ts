import type { SidebarsConfig } from '@docusaurus/plugin-content-docs';

/**
 * Soroban Cookbook Sidebar Configuration
 * Creating a structured learning path for Soroban development
 */
const sidebars: SidebarsConfig = {
  tutorialSidebar: [
    {
      type: 'category',
      label: 'Getting Started',
      items: [
        'getting-started/setup',
        'getting-started/setup-linux',
        'getting-started/setup-windows',
        'getting-started/first-contract',
        'getting-started/deploy-testnet',
        'getting-started/contract-interaction',
        'contributing',
      ],
    },
    {
      type: 'category',
      label: 'Core Concepts',
      items: [
        'concepts/introduction',
        'concepts/overview',
        'concepts/storage',
        'concepts/authorization',
        'concepts/events',
        'concepts/gas-and-resources',
      ],
    },
    {
      type: 'category',
      label: 'Patterns',
      items: [
        'patterns/overview',
        'patterns/hello-world',
      ],
    },
    {
      type: 'category',
      label: 'Security',
      items: [
        'security/fundamentals',
      ],
    },
    {
      type: 'category',
      label: 'Design System',
      items: [
        'design-system/buttons',
        'design-system/typography',
        'design-system/badges-tags',
        'design-system/empty-states',
      ],
    },
    {
      type: 'category',
      label: 'Components',
      items: [
        'components/buttons',
        'components/testimonials',
      ],
    },
    {
      type: 'category',
      label: 'Responsive',
      items: [
        'responsive/breakpoints',
      ],
    },
  ],
};

export default sidebars;
